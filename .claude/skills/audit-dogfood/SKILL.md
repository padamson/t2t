---
name: audit-dogfood
description: Review app/ and t2t for workarounds that should be features in author-maintained tools
user-invocable: true
disable-model-invocation: true
argument-hint: "[optional: chapter number or file path to focus on]"
---

# Audit Dogfooding Gaps

You are reviewing the app/ codebase and t2t manuscript for workarounds — places
where an author-maintained tool *should* be handling something but the developer worked
around the limitation instead. These workarounds are often invisible because the code
works fine; the problem is that the tool isn't getting the feature request it deserves.

IMPORTANT: This audit compares what the code does against what the tools should do.
Not every hand-written piece of code is a workaround — some things are intentionally
outside the tools' scope. Ask the user to confirm before filing issues.

## Tools to audit against

- **panschema** — schema-driven code/migration generation from LinkML
- **playwright-rust** — E2E browser testing
- **theoria** — component explorer for Leptos
- **dokime** — component testing framework for Leptos
- **mdbook ecosystem** — book build toolchain (plugins, preprocessors, backends)

## What to look for

### panschema workarounds (in app/)

1. **Hand-written Rust types that should be generated.** Check the LinkML schema at
   `schema/t2t.yaml` (or wherever it lives in app/). Then grep for
   struct and enum definitions in `src/models/`, `src/services/`, and `src/api/`.
   Flag any types that:
   - Mirror a class or enum defined in the LinkML schema but aren't in the generated output
   - Have `serde`, `sqlx::FromRow`, or `utoipa::ToSchema` derives that panschema should generate

2. **Hand-written SQL migrations.** Check `migrations/` for DDL that doesn't match
   panschema's generated SQL. Differences may indicate schema features panschema
   doesn't support yet (constraints, indexes, defaults, foreign keys).

3. **Manual `From`/`Into` implementations** between types that panschema should be
   generating. Look in `src/models/` and `src/api/` for `impl From<...>`.

4. **JSON Schema files** that were written by hand instead of generated.

5. **Any `// TODO`, `// HACK`, `// WORKAROUND` comments** mentioning schemas,
   types, generation, or panschema.

### playwright-rust workarounds (in app/)

1. **Raw JavaScript evaluation** (`evaluate`, `evaluate_handle`) used where a native
   playwright-rust API should exist.

2. **Test helper functions** in `tests/e2e/` that wrap playwright-rust calls to work
   around API gaps or missing convenience methods.

3. **Skipped or commented-out tests** with notes about playwright-rust limitations.

4. **Any `// TODO`, `// HACK`, `// WORKAROUND` comments** mentioning playwright,
   browser, or E2E testing.

### theoria workarounds (in app/)

1. **Inline component demos** in regular app routes instead of in the component explorer.
   Components should be registered in theoria with configurable props.

2. **Missing prop controls.** Components that can only be previewed with hardcoded props
   instead of dynamically configurable controls. May indicate theoria needs
   support for that prop type.

3. **Any `// TODO`, `// HACK`, `// WORKAROUND` comments** mentioning storybook,
   component explorer, or component preview.

### mdbook toolchain workarounds (in t2t)

1. **Raw HTML in Markdown** that works around missing mdbook features (e.g., inline
   HTML for admonitions because mdbook-admonish isn't working, HTML tables because
   mdbook's table support is insufficient).

2. **Missing code callouts.** Look for numbered lists after code blocks that are
   manually synced with code line numbers — this should be a callout preprocessor.

3. **Workarounds in book.toml** — commented-out preprocessors, pinned versions,
   disabled features.

4. **Any `<!-- TODO -->`, `<!-- HACK -->`, `<!-- WORKAROUND -->` comments** in
   chapter Markdown files.

5. **External tool usage** — any non-Rust tools used in the build pipeline that
   should have Rust equivalents (e.g., using Pandoc for format conversion).

## Process

1. **Read the current state.** Read `dogfood-gaps.md`, the LinkML schema, and `book.toml`.

2. **Scan the codebase.** Use Grep and Glob systematically:
   - `Grep` for `struct `, `enum `, `impl From`, `impl Into` in app/ `src/`
   - `Grep` for `TODO`, `HACK`, `WORKAROUND`, `panschema`, `playwright` in app/
   - `Grep` for `evaluate(`, `evaluate_handle(` in `tests/e2e/`
   - `Glob` for `migrations/*.sql` and compare against generated DDL
   - `Grep` for `<!-- TODO`, `<!-- HACK`, `<!-- WORKAROUND` in t2t `src/`
   - `Grep` for `<div`, `<table`, `<span` in t2t `src/` (raw HTML workarounds)
   - Read `book.toml` for commented-out or pinned preprocessors

3. **Cross-reference with existing issues.** Read `dogfood-gaps.md` and check
   `gh issue list -R padamson/<repo> -l enhancement` for each tool to avoid duplicates.

4. **If the user passed an argument**, focus the audit on that chapter's code or
   the specific file path.

5. **Present findings as a categorized list:**
   ```
   ## Audit Results

   ### Likely panschema gaps
   1. `src/models/question.rs:15` — Hand-written `Question` struct with
      `#[derive(FromRow, ToSchema)]`. The LinkML schema defines `Question`
      but panschema doesn't generate `FromRow` derives yet.
      -> Existing issue? No
      -> Recommendation: /blocker panschema "Generate FromRow derives"

   ### Likely playwright-rust gaps
   1. `tests/e2e/question_test.rs:42` — Uses `page.evaluate("document.querySelector...")`.
      -> Might be a usage issue, not a tool gap. Ask user.

   ### Likely mdbook toolchain gaps
   1. `book.toml:13-16` — mdbook-admonish commented out due to mdbook 0.5 incompatibility.
      -> Existing issue? Check dogfood-gaps.md
   2. `src/ch03-the-database.md:45` — Numbered list manually keyed to code lines (should be callouts).
      -> No callout preprocessor exists yet.

   ### Intentional (not workarounds)
   - `src/api/responses.rs` — Custom response wrapper types. API-layer concerns, not schema-layer.
   ```

6. **Enter plan mode with proposed actions.** Use the EnterPlanMode tool to present
   the findings and proposed actions for the user to review.

   Format the plan as:

   ```
   # Dogfood Audit Results

   ## Confirmed Gaps (will file via /blocker)

   ### 1. [tool-name]: [description]
   **File:** [path:line]
   **Evidence:** [what the code does]
   **Expected:** [what the tool should do instead]
   **Action:** File via /blocker

   ## Needs Clarification

   ### 2. [tool-name]: [description]
   **File:** [path:line]
   **Evidence:** [what the code does]
   **Question:** Is this a workaround or intentional?

   ## Intentional (no action)
   - [path] — [why this is not a workaround]

   ## dogfood-gaps.md Updates
   [entries to be added]
   ```

   The user can approve, reject, or reclassify each finding by commenting on the plan.
   After approval, exit plan mode and:
   - Run `/blocker` for each confirmed gap
   - Update dogfood-gaps.md with newly discovered gaps
   - Skip anything the user rejected or marked intentional
