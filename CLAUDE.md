# Trunk to Theory

A full-stack Rust web application with an open-source book documenting its development through Agentic Continuous Delivery practices.

## Repo structure

Monorepo containing both the application and the book:

- `app/` — Leptos + Axum application (grows with each chapter)
- `book/` — mdbook manuscript (chapters, images, custom preprocessors)
- `book/book-plan.md` — Full book plan with progress table (read this for context on any chapter)
- `infra/` — Terraform IaC
- `schema/` — LinkML schema (single source of truth for data model)

## Application

The application built in this book is **Scimantic** — a scientific knowledge management platform. "Trunk to Theory" is the book; "Scimantic" is the product.
- `CHANGELOG.md` — What changed in each version
- `.github/workflows/` — CI: app build/test/deploy + book HTML/PDF/Pages

## Book toolchain

See `README.md` for install instructions. All Rust-based:
- `mdbook`, `mdbook-typst-pdf`, `mdbook-admonish` (fork), `mdbook-quiz` (fork), `mdbook-quiz-pdf` (custom)

## App toolchain

See `book/book-plan.md` Section 4 (Technology Stack) for the full stack. The book chapters are the authoritative documentation for every tool choice and its rationale.

## Domain

Scientific knowledge management based on scimantic concepts. Core entities follow the scientific workflow: Questions → Evidence → Hypotheses → Experiments → Results. Dual database architecture: Oxigraph (Rust-native RDF triple store) for the knowledge graph + PostgreSQL for app state (users, sessions). The scimantic ontology lives in a separate repo (`padamson/scimantic-schema`).

## Code inclusion

Chapters reference app source directly via `{{#include ../../app/src/file.rs:anchor}}` with ANCHOR comments. No separate listings directory. The monorepo is tagged at chapter boundaries.

## Key conventions

- **Monorepo.** A PR that adds a feature updates both `app/` and `book/src/` in the same commit.
- **No book references in public tool repos.** Public feature requests filed against author-maintained or third-party tools must stand on their own as generic requests.
- **Schema-driven development.** Data model in LinkML YAML; panschema generates types, SQL DDL, SHACL shapes, JSON Schema. The scimantic ontology (`padamson/scimantic-schema`) is the authoritative scientific schema source; app-state schema lives locally at `app/schema/scimantic-server.yaml`.
- **Rust-only toolchain.** Fork/fix/contribute rather than switching to non-Rust alternatives.
- **Versioning.** Tags: `v0.y.0` = first draft of chapter `y`, `v0.y.z` = fixes/revisions to chapter `y` or earlier, `v1.0.0` = first edition. See `CHANGELOG.md`.

## Related repos

| Repo | Visibility | Role |
|---|---|---|
| `padamson/t2t` | Public | This repo |
| `padamson/scimantic-schema` | Public | LinkML schema for the scimantic domain |
| `padamson/panschema` | Public | Schema-driven dev tool (dogfooded) |
| `padamson/playwright-rust` | Public | E2E testing framework (dogfooded) |
| `padamson/theoria` | Public | Component explorer for Leptos (dogfooded) |
| `padamson/dokime` | Public | Component testing for Leptos (dogfooded) |
| `bdfinst/cd-migration` | Public | MinimumCD Practice Guide (pinned commit) |

## Custom skills

### `/book-pre-commit-review [file-path]`
Review manuscript for plagiarism and LLM-tell patterns before committing.

### `/book-extract-listings <chapter-number>`
Verify ANCHOR comments in `app/src/` match `{{#include}}` directives in chapter markdown.

## Hooks

- **Pre-commit manuscript warning:** When committing `book/src/*.md`, warns if `/book-pre-commit-review` hasn't been run. Checks `{{#include}}` directives resolve.
