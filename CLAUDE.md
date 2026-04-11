# Trunk to Table

A full-stack Rust web application with an open-source book documenting its development through Agentic Continuous Delivery practices.

## Repo structure

Monorepo containing both the application and the book:

- `app/` — Leptos + Axum application (grows with each chapter)
- `book/` — mdbook manuscript (chapters, images, custom preprocessors)
- `book/book-plan.md` — Full book plan with progress table (read this for context on any chapter)
- `infra/` — Terraform IaC
- `schema/` — LinkML schema (single source of truth for data model)
- `CHANGELOG.md` — What changed in each version
- `dogfood-gaps.md` — Tracked gaps in tools (created by `/blocker`)
- `.github/workflows/` — CI: app build/test/deploy + book HTML/PDF/Pages

## Book toolchain

See `README.md` for install instructions. All Rust-based:
- `mdbook`, `mdbook-typst-pdf`, `mdbook-admonish` (fork), `mdbook-quiz` (fork), `mdbook-quiz-pdf` (custom)

## App toolchain

See `book/book-plan.md` Section 4 (Technology Stack) for the full stack. The book chapters are the authoritative documentation for every tool choice and its rationale.

## Code inclusion

Chapters reference app source directly via `{{#include ../../app/src/file.rs:anchor}}` with ANCHOR comments. No separate listings directory. The monorepo is tagged at chapter boundaries.

## Key conventions

- **Monorepo.** A PR that adds a feature updates both `app/` and `book/src/` in the same commit.
- **No book references in public tool repos.** Public feature requests created by `/blocker` must stand on their own.
- **Schema-driven development.** Data model in LinkML YAML (`schema/t2t.yaml`); panschema generates types, SQL, JSON Schema.
- **Rust-only toolchain.** Fork/fix/contribute rather than switching to non-Rust alternatives.
- **Versioning.** Semver tags: `v0.x.0` for chapters/revisions, `v0.x.y` for fixes, `v1.0.0` for first edition. See `CHANGELOG.md`.

## Related repos

| Repo | Visibility | Role |
|---|---|---|
| `padamson/t2t` | Public | This repo |
| `padamson/panschema` | Public | Schema-driven dev tool (dogfooded) |
| `padamson/playwright-rust` | Public | E2E testing framework (dogfooded) |
| `padamson/theoria` | Public | Component explorer for Leptos (dogfooded) |
| `padamson/dokime` | Public | Component testing for Leptos (dogfooded) |
| `bdfinst/cd-migration` | Public | MinimumCD Practice Guide (pinned commit) |

## Custom skills

### `/blocker [tool-name] <description>`
Log a dogfooding blocker. Forks third-party tools, creates tracking issues, records in `dogfood-gaps.md`.

### `/resume [issue-number|repo-name]`
Check blocker status and resume where you left off.

### `/audit-dogfood [chapter-number|file-path]`
Review `app/` for invisible workarounds that should be features in dogfooded tools.

### `/book-pre-commit-review [file-path]`
Review manuscript for plagiarism and LLM-tell patterns before committing.

### `/book-extract-listings <chapter-number>`
Verify ANCHOR comments in `app/src/` match `{{#include}}` directives in chapter markdown.

## Hooks

- **Pre-commit manuscript warning:** When committing `book/src/*.md`, warns if `/book-pre-commit-review` hasn't been run. Checks `{{#include}}` directives resolve.
