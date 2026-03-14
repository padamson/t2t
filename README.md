# Trunk to Table

*Building a Full-Stack Rust Application with Agentic Continuous Delivery*

An open-source book about building a full-stack Rust web application from scratch, structured around Agentic Continuous Delivery practices from [MinimumCD](https://minimumcd.org). The reader builds [CuisineIQ](https://github.com/padamson/cuisineiq-rust) (a grocery list and recipe management app) as a real, deployable product.

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (for installing mdbook and backends)

## Install

```sh
cargo install mdbook mdbook-typst-pdf
cargo install --git https://github.com/padamson/mdbook-quiz
cargo install --git https://github.com/padamson/mdbook-admonish --branch feat/mdbook-0.5-compat
mdbook-admonish install
```

`mdbook-quiz` and `mdbook-admonish` are installed from forks until upstream PRs for mdbook 0.5 support are merged ([quiz #62](https://github.com/cognitive-engineering-lab/mdbook-quiz/pull/62), [admonish #235](https://github.com/tommilligan/mdbook-admonish/pull/235)).

### Fonts (optional, eliminates PDF build warnings)

The PDF uses open-source fonts. On macOS:

```sh
brew install --cask font-inter font-jetbrains-mono font-charis-sil
```

Without these, the PDF falls back to macOS system fonts (Avenir Next, Charter, Menlo) which work fine but produce warnings during build. CI installs these fonts automatically.

## Serve locally

```sh
mdbook serve --open
```

Opens the book at `http://localhost:3000` with live reload on file changes.

## Build PDF

```sh
./build-pdf.sh
```

Generates both `build/html/` and `build/typst-pdf/Trunk to Table.pdf`. PDF generation is separate from `mdbook serve` to avoid rebuild loops.

## Repo structure

```
t2t/
├── src/                    # Chapter markdown files
│   ├── SUMMARY.md          # Table of contents
│   ├── ch00-before-you-begin.md
│   ├── ch01-design-decisions.md
│   └── ...
├── listings/               # Per-chapter code snapshots from cuisineiq-rust
├── code/
│   └── cuisineiq-rust/     # Git submodule → companion app repo
├── images/                 # Figures and diagrams
├── template.typ            # Typst template for PDF generation
├── book.toml               # mdbook configuration
├── t2t-book-plan.md        # Full book plan (progress, chapter outlines, strategy)
├── CHANGELOG.md            # What changed in each version
└── CLAUDE.md               # AI assistant context
```

## Versioning

Tags follow semver: `v0.x.0` for new chapters or significant revisions, `v0.x.y` for small edits. Each tag triggers a GitHub Release with a downloadable PDF. See [CHANGELOG.md](CHANGELOG.md) for details.

## Related repos

| Repo | Role |
|------|------|
| [cuisineiq-rust](https://github.com/padamson/cuisineiq-rust) | The app built in the book |
| [panschema](https://github.com/padamson/panschema) | Schema-driven development CLI |
| [playwright-rust](https://github.com/padamson/playwright-rust) | E2E testing framework |
| [theoria](https://github.com/padamson/theoria) | Component explorer for Leptos |
| [dokime](https://github.com/padamson/dokime) | Component testing for Leptos |
