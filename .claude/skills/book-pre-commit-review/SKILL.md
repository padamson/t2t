---
name: pre-commit-review
description: Review changed manuscript text for plagiarism, missing attribution, and LLM-tell patterns before committing
user-invocable: true
disable-model-invocation: true
argument-hint: "[optional: file path to review instead of diff]"
---

# Pre-Commit Manuscript Review

Review new or changed manuscript text for two concerns before committing:
1. Plagiarism and missing attribution
2. LLM-generated text patterns ("tells")

## Process

1. **Read the style guide.** Read `style-guide.md` in the t2t repo root. This file contains
   author-specific patterns, recurring issues, and attribution rules learned from previous
   reviews. Apply everything in it alongside the checks below. If the file doesn't exist,
   proceed without it — it will be created at the end of this review.

2. **Get the text to review.** If the user passed a file path as $ARGUMENTS, read that file.
   Otherwise, run `git diff` and `git diff --cached` to get unstaged and staged changes.
   Focus only on `.md` files under `src/`. Ignore non-manuscript files.

3. **Check for plagiarism and attribution.**

   The book draws from these known sources. Check changed text against them:

   - **MinimumCD manifesto** (https://minimumcd.org) — CD/CI/TBD definitions
   - **MinimumCD Greenfield guide** (https://beyond.minimumcd.org/docs/migrate-to-cd/greenfield/)
   - **MinimumCD ACD framework** (https://beyond.minimumcd.org/docs/agentic-cd/)
   - **"Obey the Testing Goat"** by Harry Percival — pedagogical approach, structure
   - **"The Rust Programming Language"** (The Book) — Rust concepts
   - **Leptos documentation** (https://book.leptos.dev)
   - **Axum documentation**
   - **SQLx documentation**

   For each flagged passage:
   - Quote the passage
   - Identify the likely source
   - Rate severity: **verbatim** (must rewrite), **close paraphrase** (should rewrite or attribute),
     **common knowledge** (probably fine)
   - Suggest: rewrite in the author's voice, or add explicit attribution

   Attribution is fine — this book openly credits its influences. The issue is *unattributed*
   close paraphrasing.

4. **Check for LLM-tell patterns.**

   Flag text that exhibits common LLM writing patterns. These make prose feel generic and
   erode reader trust. Check for:

   **Structural tells:**
   - Tripled parallel structure overuse ("Not X. Not Y. Z." or "It's not about X. It's about Y.")
   - Three-item lists with escalating emphasis (small, medium, large pattern)
   - Every paragraph opening with a different rhetorical device (question, statement, fragment)
     in an obviously rotated pattern
   - Formulaic section transitions ("Now that we've covered X, let's look at Y")
   - Symmetrical pro/con or compare/contrast structures that feel too balanced

   **Punctuation tells:**
   - Em-dashes. Eliminate them. They have become so strongly associated with LLM-generated
     text that even legitimate uses trigger reader suspicion. Replace every em-dash with a
     comma, parentheses, colon, semicolon, or separate sentence. Target: zero per chapter.

   **Lexical tells:**
   - "Here's the thing" / "Here's what" framing
   - "Let's" when the author isn't actually doing the thing with the reader right now
   - "Straightforward" / "trivial" / "simply" — minimizing language
   - "Robust" / "elegant" / "powerful" / "seamless" — inflating language
   - "Landscape" / "ecosystem" / "paradigm" / "leverage" (as a verb)
   - "Dive into" / "deep dive" / "journey"
   - "It's worth noting" / "It's important to note" / "Notably"
   - "In this section, we'll" / "In this chapter, we'll" — unnecessary signposting
   - "This is where X shines" / "This is where it gets interesting"
   - "At the end of the day"
   - Starting sentences with "And" or "But" excessively (occasional is fine; a pattern is a tell)

   **Rhetorical tells:**
   - Ending sections with grandiose one-liners ("These trends are converging")
   - Asking a rhetorical question then immediately answering it, repeatedly
   - Over-explaining the significance of what was just said ("This matters because...")
     when the reader can draw their own conclusion
   - Overly smooth transitions — real writing has some roughness
   - Excessive hedging followed by strong assertion ("You might think X. But actually Y.")

   **Tonal tells:**
   - Uniformly enthusiastic tone without variation
   - Every technology description sounds like marketing copy
   - Absence of the author's genuine opinions, doubts, or trade-off acknowledgments
   - Everything presented as obvious in hindsight — no acknowledgment of difficulty

   For each flagged passage:
   - Quote the passage
   - Name the specific pattern
   - Suggest a rewrite that sounds more like natural technical writing

5. **Present results as a structured review.**

   ```
   ## Pre-Commit Review: src/ch01-design-decisions.md

   ### Attribution Issues

   1. **Close paraphrase** (lines 35-37)
      > "Every one of these is trivial to add to an empty project and expensive to retrofit
      > into a mature codebase."
      Source: MinimumCD Greenfield guide (near-verbatim quote)
      Suggestion: Either quote it directly with attribution, or rewrite:
      "These checks cost almost nothing to add on day one. Try adding them to a
      two-year-old codebase with no test suite and see how that goes."

   ### LLM Tells

   1. **Tripled parallel structure** (lines 12-14)
      > "Not an afterthought. Not something you 'set up later.' The pipeline shapes the code."
      Pattern: Classic LLM triple-beat rhythm
      Suggestion: Vary the structure. "It's not an afterthought — the pipeline shapes the code,
      not the other way around."

   2. **Grandiose closing** (line 142)
      > "These trends are converging, and this book sits at the intersection."
      Pattern: LLM-style section closer that overpromises
      Suggestion: Cut it entirely. The reader can see what the book covers from the TOC.

   ### Clean
   - Lines 50-80: Reads naturally, good author voice.
   - The architecture diagram section has a clear, direct tone.
   ```

6. **Enter plan mode with proposed fixes.** After presenting the review, use the
   EnterPlanMode tool to create a plan listing every proposed fix. Each fix should be
   a discrete item the user can approve, reject, or modify by commenting on the plan.

   Format the plan as:

   ```
   # Pre-Commit Review Fixes: [filename]

   ## Attribution Fixes

   ### Fix 1: [description]
   **Line(s):** [line numbers]
   **Current:** [quoted text]
   **Proposed:** [rewritten text or attribution added]
   **Reason:** [attribution rule or source identified]

   ## LLM Tell Fixes

   ### Fix 2: [description]
   **Line(s):** [line numbers]
   **Current:** [quoted text]
   **Proposed:** [rewritten text]
   **Reason:** [pattern name from style guide or skill]

   ## Em-Dash Reductions

   ### Fix 3: [description]
   **Line(s):** [line numbers]
   **Current:** [quoted text with em-dash]
   **Proposed:** [rewritten with comma/colon/parentheses/separate sentence]

   ...
   ```

   After the user reviews and comments on the plan, exit plan mode and apply only
   the approved fixes using the Edit tool. Skip any the user rejected or modified.

7. **Update the style guide.** After the review (whether or not fixes are applied),
   update `style-guide.md` with any new observations. Create the file if it doesn't exist.

   The style guide has three sections:

   ```markdown
   # Style Guide

   Learned patterns from manuscript reviews. Read by `/pre-commit-review` at the
   start of every review.

   ## Attribution Rules
   <!-- Sources that need attribution every time they're referenced -->
   - MinimumCD quotes must be blockquoted or explicitly attributed — "feature zero"
     and "trivial to add to an empty project" are direct quotes

   ## Recurring LLM Tells
   <!-- Patterns this specific author/LLM combination tends to produce -->
   - Tripled parallel structure ("Not X. Not Y. Z.") — appears 3x in ch01, limit to 1 per chapter
   - "Here's" sentence openers — appeared 2x in ch01
   - Technology lists where every bullet follows the same template with no caveats

   ## Voice Notes
   <!-- What the author's natural voice sounds like when it's working -->
   - The architecture diagram section in ch01 reads well — direct, specific, no filler
   - Table format for technology-to-constraint mapping works better than prose for this author
   ```

   Rules for updating:
   - Add new patterns when they're confirmed (user agreed it was a problem)
   - Remove patterns the user explicitly rejected as false positives
   - Add "voice notes" when sections read well — these help calibrate future reviews
   - Keep it concise — this is a checklist, not an essay
   - Never remove entries without the user's approval

## Important Notes

- This skill reviews *prose*, not code blocks or configuration examples.
- False positives are fine — flag aggressively, let the author decide.
- The goal is not to eliminate all LLM assistance. It's to ensure the final text
  reads in the author's natural voice. Some patterns are fine in isolation; they
  become tells when they repeat.
- When suggesting rewrites, aim for direct, opinionated technical prose — the kind
  you'd find in the best technical books (No Starch Press, O'Reilly). Concrete over
  abstract. Specific over general. Short sentences mixed with long ones, not uniform length.
