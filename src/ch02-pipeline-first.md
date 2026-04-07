# Pipeline First

<!-- Draft this chapter alongside building cuisineiq-rust -->

<!-- This chapter has five phases, each ending with a concrete checkpoint. -->
<!-- The greenfield checklist items from Ch 1 are checked off in batches at each checkpoint. -->

<!-- ============================================================ -->
<!-- PHASE 1: Local Development Environment -->
<!-- ============================================================ -->

<!-- The reader builds: -->
<!-- - VS Code devcontainer with full Rust toolchain (.devcontainer/devcontainer.json) -->
<!-- - Podman (or Docker) as the container runtime -->
<!-- - Containerized PostgreSQL via compose.yaml, matching the Linode production version -->

<!-- The reader learns: -->
<!-- - Podman: daemonless, rootless, open source (no Docker Desktop licensing) -->
<!-- - Devcontainers: one-click environment setup, reproducible across machines -->
<!-- - compose.yaml: defining services, matching production database version locally -->

<!-- Checkpoint: `podman compose up`, connect to PostgreSQL, confirm it's running -->
<!-- Greenfield checklist items checked off: -->
<!--   - VS Code devcontainer provides a one-click setup with full Rust toolchain -->
<!--   - Podman (or Docker) is the container runtime -->
<!--   - PostgreSQL runs in a container via compose.yaml, matching the production version -->

<!-- ============================================================ -->
<!-- PHASE 2: Hello-World Endpoint + Build -->
<!-- ============================================================ -->

<!-- The reader builds: -->
<!-- - Bare Axum health-check endpoint (/health returns "ok") -->
<!-- - cargo-leptos project structure (server binary + WASM client) -->
<!-- - Structured logging with tracing on the health-check handler -->

<!-- The reader learns: -->
<!-- - Axum basics: async fn handlers, Router, Tokio runtime -->
<!-- - async/await fundamentals: why web servers need async, what .await means -->
<!-- - Structured logging with tracing: spans, events, tracing-subscriber -->
<!--   - Human-readable output for development, JSON output for production -->
<!--   - Why structured logging matters: log aggregation tools parse JSON, not free-text -->
<!-- - cargo-leptos: dual-target compilation (server binary + WASM client), hot reloading -->

<!-- Checkpoint: `cargo leptos serve`, `curl localhost:3000/health` returns "ok" with a log line -->
<!-- Greenfield checklist items checked off: -->
<!--   - cargo leptos build compiles, tests, and packages with a single command -->
<!--   - Structured logging with tracing from the first handler -->

<!-- ============================================================ -->
<!-- PHASE 3: Schema Foundation -->
<!-- ============================================================ -->

<!-- The reader builds: -->
<!-- - Initial LinkML schema for GroceryItem and ItemStatus -->
<!-- - panschema generating Rust types and SQL DDL from the schema -->
<!-- - Generated types compile and are used by the health-check handler -->

<!-- The reader learns: -->
<!-- - LinkML basics: classes, attributes, enums, relationships in YAML -->
<!-- - panschema basics: install CLI, run `panschema generate`, read the generated code -->
<!-- - The generated code is idiomatic Rust, not opaque output -->
<!-- - Schema-driven development: the pipeline will regenerate and verify on every push -->

<!-- Checkpoint: `cargo build` passes with generated types from the schema -->
<!-- Greenfield checklist items checked off: -->
<!--   - LinkML schema is versioned in the repo; panschema generates types and migrations in CI -->

<!-- ============================================================ -->
<!-- PHASE 4: CI Pipeline -->
<!-- ============================================================ -->

<!-- The reader builds: -->
<!-- - GitHub Actions workflow: build, rustfmt, clippy, cargo-nextest, cargo-audit, cargo-deny -->
<!-- - cargo-mutants configured (--in-diff on every push, full sweep nightly) -->
<!-- - .pre-commit-config.yaml with hooks mirroring CI (rustfmt, clippy, cargo-audit) -->
<!-- - prek installed and hooks enabled (cargo install prek && prek install) -->
<!-- - Dependabot configuration for automated dependency update PRs -->
<!-- - GitHub code scanning (SAST) and secret scanning enabled -->
<!-- - Schema generation step in the pipeline (panschema generate + verify) -->

<!-- The reader learns: -->
<!-- - GitHub Actions: workflow files, triggers, jobs, steps -->
<!-- - Pipeline-enforced quality gates: why these are gates, not suggestions -->
<!-- - rustfmt: automatic formatting, pipeline rejects unformatted code -->
<!-- - clippy: Rust linter, catches unidiomatic patterns and potential bugs -->
<!-- - cargo-audit: dependency vulnerability scanning -->
<!-- - cargo-deny: supply chain policy (license compliance, source vetting, duplicate detection) -->
<!-- - cargo-mutants: why code coverage isn't enough, --in-diff as pipeline gate, nightly full sweep -->
<!-- - prek: Rust-native pre-commit hooks, reads .pre-commit-config.yaml -->
<!-- - Why local hooks mirror CI: 5-second local feedback vs 10-minute CI failure, keeps pipeline green -->
<!-- - Dependabot: automated dependency update PRs -->
<!-- - GitHub security features: SAST, secret scanning, dependency review -->

<!-- Checkpoint: push to trunk, pipeline goes green (all gates pass) -->
<!-- Greenfield checklist items checked off: -->
<!--   - GitHub Actions CI pipeline runs on every push to trunk -->
<!--   - All work integrates to trunk at least daily -->
<!--   - cargo-audit scans dependencies for known CVEs on every build -->
<!--   - cargo-deny enforces supply chain policy -->
<!--   - Pre-commit hooks mirror CI checks via prek -->
<!--   - Dependabot is configured for automated dependency update PRs -->
<!--   - GitHub code scanning (SAST) and secret scanning are enabled -->

<!-- ============================================================ -->
<!-- PHASE 5: Infrastructure + Deployment -->
<!-- ============================================================ -->

<!-- The reader builds: -->
<!-- - Terraform configs for Linode: compute instance, managed PostgreSQL, HTTPS/TLS, automated backups -->
<!-- - Staging and production environments from the same Terraform configs -->
<!-- - Deploy hello-world to production through the pipeline -->
<!-- - Secrets management: GitHub Secrets for Linode API tokens, .env for local dev -->
<!-- - Local /security-review Claude Code skill and pre-commit hooks -->
<!-- - Rollback tested: deploy, break, rollback, verify -->

<!-- The reader learns: -->
<!-- - Terraform basics: providers, resources, state, HCL syntax -->
<!-- - Linode provisioning: compute instances, managed PostgreSQL, HTTPS/TLS, automated backups -->
<!-- - Why staging and production use the same Terraform configs (no snowflake environments) -->
<!-- - Secrets management: environment variables, GitHub Secrets, .env in .gitignore -->
<!-- - HTTPS/TLS from day one (production-like environments) -->
<!-- - Local security tooling: /security-review skill, pre-commit hooks -->
<!-- - Immutable artifacts: single binary built once, deployed to both environments -->
<!-- - Externalized configuration: environment variables, not baked into the binary -->
<!-- - Rollback: deploy, verify, break intentionally, rollback, verify again -->

<!-- Checkpoint: hit the production URL over HTTPS, see "ok" -->
<!-- Greenfield checklist items checked off: -->
<!--   - Secrets are managed via environment variables and GitHub Secrets, never committed -->
<!--   - HTTPS/TLS is configured on all deployed environments -->
<!--   - Local /security-review skill and pre-commit hooks are set up -->
<!--   - Deployment to staging is automated via Terraform + GitHub Actions -->
<!--   - Pipeline deploys to a production-like staging environment on Linode -->
<!--   - Pipeline deploys to production on Linode -->
<!--   - Every commit that passes the pipeline is a deployment candidate -->
<!--   - Rollback is tested and works -->
<!--   - Application configuration is externalized -->
<!--   - Artifacts are immutable -->
<!--   - Database backups are automated via Terraform -->
