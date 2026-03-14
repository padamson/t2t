# Changelog

All notable changes to the Trunk to Table manuscript are documented here.

Version scheme: `v0.x.0` for new chapters or significant revisions, `v0.x.y` for small edits and fixes. `v1.0.0` marks the complete first edition. Each tagged version triggers a GitHub Release with a downloadable PDF.

## Unreleased

### Chapters
- Ch 00 draft: Before You Begin (MinimumCD setup, reading assignments, prerequisites)
- Ch 01 draft: Design Decisions (pipeline-first, schema-driven, dual-interface architecture, testing pyramid, greenfield checklist)
- Ch 02-14 scaffolds with chapter outlines
- Appendix A scaffold

### Toolchain
- PDF generation via mdbook-typst-pdf with custom Typst template (Charis SIL, Inter, JetBrains Mono)
- mdbook-quiz enabled via fork (padamson/mdbook-quiz) for Ch 01 MinimumCD quiz
- mdbook-admonish enabled via fork (padamson/mdbook-admonish) for admonitions
- CI/CD: build HTML + PDF on every push, PDF attached to GitHub Releases on tag

### Book plan
- Progress table with chapter status, dependencies, and code scope
- Versioning convention: sequential semver, decoupled from chapter numbers
- Mutation testing (cargo-mutants) woven throughout chapters 1-5
- Structured logging (tracing) and error handling (thiserror) added to chapter progression
- Dogfood blocker tracking for third-party tools (mdbook-admonish, mdbook-quiz)
