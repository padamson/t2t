# Dogfood Gaps

Tracked gaps in tools used by the book and CuisineIQ. Created and updated by `/blocker`.

## 2026-03-21 — mdbook-admonish: mdbook 0.5 compatibility
- **Category:** Third-party
- **Upstream:** tommilligan/mdbook-admonish#233
- **Fork:** padamson/mdbook-admonish
- **Upstream PR:** tommilligan/mdbook-admonish#235 (supersedes stalled #234)
- **Tracking issue:** padamson/t2t#1
- **Chapter:** All chapters (admonitions throughout)
- **Status:** Using fork
- **Workaround:** Using fork via `cargo install --git https://github.com/padamson/mdbook-admonish`

## 2026-03-21 — mdbook-quiz: mdbook 0.5 compatibility
- **Category:** Third-party
- **Upstream:** cognitive-engineering-lab/mdbook-quiz#61
- **Fork:** padamson/mdbook-quiz
- **Upstream PR:** cognitive-engineering-lab/mdbook-quiz#62 (complete, awaiting review)
- **Tracking issue:** padamson/t2t#2
- **Chapter:** Ch 01 (MinimumCD quiz), future chapters
- **Status:** Using fork
- **Workaround:** Using fork via `cargo install --git https://github.com/padamson/mdbook-quiz`

## 2026-03-27 — mdbook-typst-pdf: Admonish blocks unstyled in PDF output
- **Category:** Third-party
- **Upstream:** KaiserY/mdbook-typst-pdf#11
- **Fork:** padamson/mdbook-typst-pdf
- **Upstream PR:** KaiserY/mdbook-typst-pdf#12 (MERGED)
- **Tracking issue:** padamson/t2t#3
- **Chapter:** All chapters (admonitions throughout)
- **Status:** Resolved — merged upstream and released as v0.7.3
