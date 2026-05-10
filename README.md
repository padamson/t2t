# Trunk to Theory

*Building a Full-Stack Rust Application with Agentic Continuous Delivery*

An open-source book documenting the journey of building [Scimantic](https://github.com/padamson/t2t) — a scientific knowledge management platform — from scratch, structured around Agentic Continuous Delivery practices from [MinimumCD](https://minimumcd.org).

Scimantic lets researchers pose Questions, gather Evidence, form Hypotheses, design Experiments, and record Results, powered by a dual-database architecture (Oxigraph knowledge graph + PostgreSQL app state). It's available as a hosted service or self-deployable.

**Read the book:** [padamson.github.io/t2t](https://padamson.github.io/t2t)

## Repo structure

```
t2t/
├── app/          # Leptos + Axum application (grows with each chapter)
├── book/         # mdbook manuscript
├── infra/        # Terraform IaC
├── schema/       # LinkML data model
└── .github/      # CI workflows (app + book)
```

## Building the book

### Prerequisites

- [Rust toolchain](https://rustup.rs/)

### Install book toolchain

```sh
cargo install mdbook mdbook-typst-pdf
cargo install --git https://github.com/padamson/mdbook-quiz --branch feature/mdbook-0.5-support
cargo install --git https://github.com/padamson/mdbook-admonish --branch feat/mdbook-0.5-compat
mdbook-admonish install --dir book
cargo install --git https://github.com/padamson/mdbook-listings --branch main --force mdbook-listings
cargo install --path book/tools/mdbook-quiz-pdf
```

`mdbook-quiz` and `mdbook-admonish` are installed from forks until upstream PRs for mdbook 0.5 support are merged ([quiz #62](https://github.com/cognitive-engineering-lab/mdbook-quiz/pull/62), [admonish #235](https://github.com/tommilligan/mdbook-admonish/pull/235)). `mdbook-listings` tracks `main` while it's pre-1.0; it'll switch to a crates.io install once v0.1.0 is published. Re-run the `cargo install --force ... mdbook-listings` command to pull the latest main; the preprocessor refreshes its bundled CSS/JS on every build, so no separate reinstall step is needed.

### Fonts (optional, eliminates PDF build warnings)

```sh
brew install --cask font-inter font-jetbrains-mono font-charis-sil
```

### Serve locally

```sh
cd book && mdbook serve --open
```

### Build PDF

```sh
cd book && ./build-pdf.sh
```

## Versioning

Tags use `v0.y.z` where `y` = first draft of chapter `y`, `z` = fixes or revisions to chapter `y` or earlier. Each tag triggers a GitHub Release with a downloadable PDF. See [CHANGELOG.md](CHANGELOG.md).

## Related repos

| Repo | Role |
|------|------|
| [panschema](https://github.com/padamson/panschema) | Schema-driven development CLI |
| [scimantic-schema](https://github.com/padamson/scimantic-schema) | LinkML schema for the scimantic domain |
| [playwright-rust](https://github.com/padamson/playwright-rust) | E2E testing framework |
| [theoria](https://github.com/padamson/theoria) | Component explorer for Leptos |
| [dokime](https://github.com/padamson/dokime) | Component testing for Leptos |
