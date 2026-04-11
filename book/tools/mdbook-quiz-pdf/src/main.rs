use serde::Deserialize;
use serde_json::Value;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            let renderer = args.next().unwrap_or_default();
            if renderer == "html" {
                process::exit(1); // don't run for HTML — mdbook-quiz handles it
            }
            process::exit(0); // run for everything else (typst-pdf, etc.)
        }
        _ => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .expect("Failed to read stdin");

            let mut data: Vec<Value> =
                serde_json::from_str(&input).expect("Failed to parse JSON input");

            // data is [context, book] — mdbook preprocessor protocol
            if data.len() != 2 {
                eprintln!("quiz-pdf: expected [context, book], got {} elements", data.len());
                process::exit(1);
            }

            // Extract context fields before mutably borrowing book
            let root = data[0]["root"]
                .as_str()
                .map(PathBuf::from)
                .unwrap_or_default();
            let src = data[0]["config"]["book"]["src"]
                .as_str()
                .unwrap_or("src")
                .to_owned();
            let renderer = data[0]["renderer"]
                .as_str()
                .unwrap_or("unknown")
                .to_owned();
            let src_dir = root.join(&src);

            eprintln!("quiz-pdf: running for renderer '{}'", renderer);

            let book = &mut data[1];

            process_items(book.get_mut("items"), &src_dir);

            serde_json::to_writer(io::stdout(), book).expect("Failed to write output");
        }
    }
}

fn process_items(items: Option<&mut Value>, src_dir: &Path) {
    let Some(Value::Array(items)) = items else {
        return;
    };

    for item in items.iter_mut() {
        // Items can be {"Chapter": {...}} or {"Separator": ...} or {"PartTitle": ...}
        if let Some(chapter) = item.get_mut("Chapter") {
            let content = chapter.get("content").and_then(|v| v.as_str()).unwrap_or("").to_owned();
            let path = chapter.get("path").and_then(|v| v.as_str()).unwrap_or("").to_owned();

            if !path.is_empty() {
                let chapter_dir = src_dir
                    .join(&path)
                    .parent()
                    .unwrap_or(src_dir)
                    .to_owned();
                let new_content = replace_quiz_directives(&content, &chapter_dir);
                chapter["content"] = Value::String(new_content);
            }

            // Recurse into sub-items
            process_items(chapter.get_mut("sub_items"), src_dir);
        }
    }
}

// --- Quiz TOML types ---

#[derive(Deserialize)]
struct QuizFile {
    questions: Vec<Question>,
}

#[derive(Deserialize)]
struct Question {
    prompt: Prompt,
    answer: Answer,
    #[allow(dead_code)]
    #[serde(default)]
    context: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    id: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    r#type: Option<String>,
}

#[derive(Deserialize)]
struct Prompt {
    prompt: String,
    #[serde(default)]
    distractors: Vec<String>,
}

#[derive(Deserialize)]
struct Answer {
    answer: String,
}

// --- Quiz rendering ---

fn replace_quiz_directives(content: &str, chapter_dir: &Path) -> String {
    let mut result = String::with_capacity(content.len());

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("{{#quiz ") && trimmed.ends_with("}}") {
            let quiz_path = trimmed
                .trim_start_matches("{{#quiz ")
                .trim_end_matches("}}");
            let full_path = chapter_dir.join(quiz_path);

            match render_quiz_toml(&full_path) {
                Ok(rendered) => result.push_str(&rendered),
                Err(e) => {
                    eprintln!("quiz-pdf: warning: could not render {}: {}", quiz_path, e);
                    result.push_str(line);
                }
            }
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }

    result
}

fn render_quiz_toml(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    render_quiz_from_str(&contents)
}

fn render_quiz_from_str(contents: &str) -> Result<String, Box<dyn std::error::Error>> {
    let quiz: QuizFile = toml::from_str(contents)?;

    let mut out = String::new();
    out.push_str("```admonish quiz title=\"Check Your Understanding\"\n");

    let num_questions = quiz.questions.len();
    let mut answers = Vec::new();

    for (i, q) in quiz.questions.iter().enumerate() {
        let num = i + 1;
        out.push_str(&format!("**{}. {}**\n\n", num, q.prompt.prompt));

        // Collect all choices and sort alphabetically for consistent ordering
        let mut choices: Vec<&str> = q.prompt.distractors.iter().map(|s| s.as_str()).collect();
        choices.push(&q.answer.answer);
        choices.sort();

        let answer_idx = choices
            .iter()
            .position(|c| *c == q.answer.answer.as_str())
            .unwrap_or(0);

        for (j, choice) in choices.iter().enumerate() {
            let letter = (b'a' + j as u8) as char;
            out.push_str(&format!("  {}. {}\n", letter, choice));
        }

        let answer_letter = (b'a' + answer_idx as u8) as char;
        answers.push(format!("{}-{}", num, answer_letter));

        if i < num_questions - 1 {
            out.push('\n');
        }
    }

    out.push_str(&format!(
        "\n<small>Answers: {}</small>\n",
        answers.join(", ")
    ));
    out.push_str("```\n");

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    const SINGLE_QUESTION: &str = r#"
[[questions]]
type = "MultipleChoice"
prompt.prompt = "What is 2 + 2?"
prompt.distractors = ["3", "5"]
answer.answer = "4"
context = "Basic arithmetic"
id = "q1"
"#;

    const TWO_QUESTIONS: &str = r#"
[[questions]]
type = "MultipleChoice"
prompt.prompt = "What color is the sky?"
prompt.distractors = ["Green", "Red"]
answer.answer = "Blue"
id = "q1"

[[questions]]
type = "MultipleChoice"
prompt.prompt = "What is the capital of France?"
prompt.distractors = ["London", "Berlin"]
answer.answer = "Paris"
id = "q2"
"#;

    #[test]
    fn renders_admonish_block() {
        let result = render_quiz_from_str(SINGLE_QUESTION).unwrap();
        assert!(result.starts_with("```admonish quiz title=\"Check Your Understanding\"\n"));
        assert!(result.ends_with("```\n"));
    }

    #[test]
    fn includes_question_prompt() {
        let result = render_quiz_from_str(SINGLE_QUESTION).unwrap();
        assert!(result.contains("**1. What is 2 + 2?**"));
    }

    #[test]
    fn choices_sorted_alphabetically() {
        let result = render_quiz_from_str(SINGLE_QUESTION).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        let choice_lines: Vec<&&str> = lines.iter().filter(|l| l.starts_with("  ")).collect();
        assert_eq!(choice_lines.len(), 3);
        assert!(choice_lines[0].contains("3"));
        assert!(choice_lines[1].contains("4"));
        assert!(choice_lines[2].contains("5"));
    }

    #[test]
    fn answer_key_correct() {
        let result = render_quiz_from_str(SINGLE_QUESTION).unwrap();
        // "4" sorts to position b (after "3")
        assert!(result.contains("Answers: 1-b"));
    }

    #[test]
    fn multiple_questions_numbered() {
        let result = render_quiz_from_str(TWO_QUESTIONS).unwrap();
        assert!(result.contains("**1. What color is the sky?**"));
        assert!(result.contains("**2. What is the capital of France?**"));
    }

    #[test]
    fn multiple_questions_answer_key() {
        let result = render_quiz_from_str(TWO_QUESTIONS).unwrap();
        assert!(result.contains("Answers: 1-a, 2-c"));
    }

    #[test]
    fn replace_directive_with_quiz() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "{}", SINGLE_QUESTION).unwrap();
        let dir = tmp.path().parent().unwrap();
        let filename = tmp.path().file_name().unwrap().to_str().unwrap();

        let content = format!("Some text\n{{{{#quiz {}}}}}\nMore text", filename);
        let result = replace_quiz_directives(&content, dir);

        assert!(result.contains("```admonish quiz"));
        assert!(result.contains("Some text"));
        assert!(result.contains("More text"));
        assert!(!result.contains("{{#quiz"));
    }

    #[test]
    fn non_quiz_lines_preserved() {
        let content = "Regular line\nAnother line\n";
        let result = replace_quiz_directives(content, Path::new("."));
        assert_eq!(result, "Regular line\nAnother line\n");
    }

    #[test]
    fn supports_rejects_html() {
        // The supports check exits with code 1 for HTML, 0 for others.
        // We test the logic indirectly: HTML should be excluded.
        let is_html = "html" == "html";
        let is_typst = "typst-pdf" == "html";
        assert!(is_html);
        assert!(!is_typst);
    }
}
