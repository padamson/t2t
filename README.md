# Trunk to Table

*Building a Full-Stack Rust Application with Agentic Continuous Delivery*

An open-source book and application. The book documents the complete process of building a full-stack Rust web application from scratch, structured around Agentic Continuous Delivery practices from [MinimumCD](https://minimumcd.org).

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
cargo install --path book/tools/mdbook-quiz-pdf
```

`mdbook-quiz` and `mdbook-admonish` are installed from forks until upstream PRs for mdbook 0.5 support are merged ([quiz #62](https://github.com/cognitive-engineering-lab/mdbook-quiz/pull/62), [admonish #235](https://github.com/tommilligan/mdbook-admonish/pull/235)).

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

Tags follow semver: `v0.x.0` for new chapters or significant revisions, `v0.x.y` for small edits. Each tag triggers a GitHub Release with a downloadable PDF. See [CHANGELOG.md](CHANGELOG.md).

## Related repos

| Repo | Role |
|------|------|
| [panschema](https://github.com/padamson/panschema) | Schema-driven development CLI |
| [playwright-rust](https://github.com/padamson/playwright-rust) | E2E testing framework |
| [theoria](https://github.com/padamson/theoria) | Component explorer for Leptos |
| [dokime](https://github.com/padamson/dokime) | Component testing for Leptos |
