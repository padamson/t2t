# Changelog

All notable changes to the Trunk to Theory manuscript are documented here.
The format is based on [Keep a Changelog](https://keepachangelog.com/).

Version scheme: `v0.y.z` where `y` = first draft of chapter `y`, `z` = fixes or revisions to chapter `y` or earlier. `v1.0.0` marks the complete first edition. Each tagged version triggers a GitHub Release with a downloadable PDF.

## [v0.1.5] - 2026-04-11

### Changed
- **Domain pivot:** Renamed from "Trunk to Table" to "Trunk to Theory"
- Domain changed from grocery/recipe management to scientific knowledge management (scimantic)
- Core entities: Questions → Evidence → Hypotheses → Experiments → Results
- Dual database architecture: Oxigraph (RDF knowledge graph) + PostgreSQL (app state)
- panschema now generates SHACL shapes in addition to Rust types, SQL DDL, JSON Schema
- Ontology lives in separate `padamson/scimantic-ontology` repo
- Chapter 7 renamed: "Check Off Items" → "Evidence"
- Chapter 9 renamed: "Shared Grocery Lists" → "Hypotheses"
- Chapter 10 renamed: "Recipe Creation" → "Experiments"
- Chapter 11 renamed: "Recipe-to-Grocery Integration" → "Results & Analysis"
- Part II renamed: "Building on the Foundation" → "The Scientific Workflow"
- REST API serves external consumers (CLI tools, Jupyter integrations) instead of mobile apps
- Book plan, chapter scaffolds, and all references updated for new domain

### Added
- Oxigraph (Rust-native RDF triple store) to technology stack
- sophia_rs (RDF toolkit) to technology stack
- SPARQL endpoint in REST API chapter
- `padamson/scimantic-ontology` as related repo

## [v0.1.4] - 2026-04-11

### Changed
- Unified monorepo: app and book in one repo (app/ + book/)
- Renamed "CuisineIQ" to "Trunk to Table" throughout
- Removed cuisineiq-rust submodule; app code lives in app/
- Book content moved from repo root to book/ subdirectory
- Skills renamed with scope prefix (book-pre-commit-review, book-extract-listings)
- CI workflow updated for monorepo structure

### Added
- cargo-vet for supply chain vetting with trusted audit imports
- SLSA provenance attestation on release artifacts
- Scheduled weekly security workflow
- Private key detection in pre-commit hooks
- prek added to toolchain and greenfield checklist

### Removed
- cuisineiq-rust submodule
- listings/ directory (chapters include directly from app/src/)

## [v0.1.3] - 2026-03-31

### Added
- mdbook-quiz-pdf preprocessor: renders quizzes as styled admonish boxes in PDF output
- Custom "quiz" admonish type with purple question-mark icon
- GitHub repo link in mdbook site header

## [v0.1.2] - 2026-03-28

### Changed
- Switch mdbook-typst-pdf from fork to upstream v0.7.3 (admonish support merged)
- Re-enable CI caching for mdbook-typst-pdf
- First dogfood blocker fully resolved: found gap, forked, fixed, contributed, merged, released

## [v0.1.1] - 2026-03-27

### Fixed
- PDF admonish blocks now render as styled boxes (using fork padamson/mdbook-typst-pdf)

### Changed
- CI uses forked mdbook-typst-pdf with admonish support

### Added
- Track mdbook-typst-pdf blocker in dogfood-gaps.md (padamson/t2t#3)

## [v0.1.0] - 2026-03-22

### Added
- Ch 00 draft: Before You Begin (MinimumCD setup, reading assignments, prerequisites)
- Ch 01 draft: Design Decisions (pipeline-first, schema-driven, dual-interface architecture, testing pyramid, greenfield checklist)
- Ch 02-14 scaffolds with chapter outlines
- Appendix A scaffold
- PDF generation via mdbook-typst-pdf with custom Typst template (Charis SIL, Inter, JetBrains Mono)
- mdbook-quiz enabled via fork (padamson/mdbook-quiz) for Ch 01 MinimumCD quiz
- mdbook-admonish enabled via fork (padamson/mdbook-admonish) for admonitions
- CI/CD: build HTML + PDF on every push, PDF attached to GitHub Releases on tag, GitHub Pages deployment
- Progress table with chapter status, dependencies, and code scope
- Versioning convention: sequential semver, decoupled from chapter numbers
- Mutation testing (cargo-mutants) woven throughout chapters 1-5
- Structured logging (tracing) and error handling (thiserror) added to chapter progression
- Dogfood blocker tracking for third-party tools (mdbook-admonish, mdbook-quiz)

[v0.1.5]: https://github.com/padamson/t2t/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/padamson/t2t/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/padamson/t2t/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/padamson/t2t/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/padamson/t2t/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/padamson/t2t/releases/tag/v0.1.0
