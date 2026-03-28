# Changelog

All notable changes to the Trunk to Table manuscript are documented here.
The format is based on [Keep a Changelog](https://keepachangelog.com/).

Version scheme: `v0.x.0` for new chapters or significant revisions, `v0.x.y` for small edits and fixes. `v1.0.0` marks the complete first edition. Each tagged version triggers a GitHub Release with a downloadable PDF.

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

[v0.1.2]: https://github.com/padamson/t2t/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/padamson/t2t/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/padamson/t2t/releases/tag/v0.1.0
