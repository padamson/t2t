# Pipeline First

```admonish reference title="Reading Assignment"
If you haven't already, re-read [localhost:1313/docs/migrate-to-cd/greenfield/](http://localhost:1313/docs/migrate-to-cd/greenfield/), specifically the "Feature Zero" section. This chapter is Feature Zero.
```

```admonish tip title="Practice Guide Running?"
The reading assignment links point to `localhost:1313`. Make sure your local MinimumCD Practice Guide is running: `cd minimumcd-practice-guide && npm start`. If you haven't cloned it yet, see [Before You Begin](./ch00-before-you-begin.md#set-up-the-minimumcd-practice-guide).
```

You read the greenfield guide. Its core message: the delivery pipeline is the first thing you build. Before the data model, before the UI, before even a "hello world" endpoint.

That's counterintuitive. You're excited to build Scimantic. You want to define the knowledge graph schema, write your first Leptos component, see something in a browser. Instead, you're going to spend this entire chapter on infrastructure: containers, CI/CD, Terraform, security scanning. By the end, you'll have a URL that returns "ok" and nothing else.

Why? Because every decision you make this chapter is trivial to set up in an empty project and expensive to retrofit later. The greenfield guide is explicit: "Every one of these is trivial to add to an empty project and expensive to retrofit into a mature codebase." We take that literally.

This chapter has five phases, each ending with a concrete checkpoint where you run something and see it work. At each checkpoint, we'll revisit the greenfield checklist from Chapter 1 and check off what we completed.

```admonish info title="Why phases here, but not later?"
Phases are a Chapter 2 device, not a book-wide convention. This chapter is unusual: it's "Feature Zero," sequential infrastructure setup before any user-visible feature ships. The phases give us five natural checkpoints — Podman running, server responding, schema generating, pipeline green, production deployed — that each end with the reader running something and seeing it work.

Starting in Chapter 3, the structure shifts. Chapters 3 through 6 each add a layer of the first vertical slice (database, frontend, end-to-end test, REST API), with one linear flow per chapter. Starting in Chapter 7, every chapter is its own user-story slice (Evidence, Hypotheses, Experiments, Results) and the chapter-internal pacing follows the outside-in TDD pattern from the [user stories table](./ch01-design-decisions.md#from-user-stories-to-vertical-slices) in Chapter 1.

If "phase" feels heavy, that's because it is — it's load-bearing for this chapter only.
```

## Phase 1: Local Development Environment

Before you write any code, you need a reproducible development environment. "It works on my machine" is the first thing CD eliminates. By the end of this phase, every reader (regardless of operating system) will have identical tooling and an identical PostgreSQL database.

We'll set up five things:

1. An **empty GitHub repository** for your Scimantic project, cloned locally.
2. The **Rust toolchain** via `rustup`, installed directly on your host.
3. The **Scimantic CLI**, installed from crates.io.
4. A **container runtime** (Podman) to run services locally.
5. A **compose file** that starts PostgreSQL in a container, matching the version we'll deploy to production.

### Create the Repository

On [github.com](https://github.com), click **New repository**. Name it `scimantic` (or whatever you prefer), mark it public or private per your preference, and leave everything else unchecked — no README, no `.gitignore`, no license. We'll add those deliberately as the chapter progresses, each at the moment we have a reason for it.

Clone it locally:

```bash
git clone git@github.com:YOUR-USERNAME/scimantic.git
cd scimantic
```

The `compose.yaml` we create shortly will be the first file in this repository. From this point on, every command in the book assumes you're working inside this directory.

```admonish info
This book expects you to be comfortable with git basics and GitHub. If you need a refresher, see the [Before You Begin](./ch00-before-you-begin.md#what-you-need) section for pointers.
```

### The Rust Toolchain

As mentioned in [Before You Begin](./ch00-before-you-begin.md#what-you-need), you are expected to have some basic knowledge of Rust (*e.g.,* you've read the first half of [*The Rust Programming Language*](https://doc.rust-lang.org/book/)), so you likely have Rust installed via the official toolchain manager, `rustup`. If neither is true, I recommend you go check out [*The Rust Programming Language*](https://doc.rust-lang.org/book/)) and, in the process, install Rust via [rustup](https://rustup.rs/). `rustup` handles multiple Rust versions, lets you pin a project to a specific one, and manages the components we'll use throughout the book (the stable compiler, `rustfmt`, `clippy`, and the WebAssembly target).

After installation, run the following commands in the terminal:

```bash
rustc --version
cargo --version
```

You should see `rustc 1.95.0` and `cargo 1.95.0` or later (they ship together, so the version numbers always match). Chapter 3 will pin the project's MSRV (minimum supported Rust version) so the build fails on older toolchains.

We'll install additional targets and cargo tools (`cargo-leptos`, `sqlx-cli`, `cargo-nextest`, the WebAssembly target, and more) as each chapter needs them.

### The Scimantic CLI

With `cargo` on your path, you can install the Scimantic CLI from [crates.io](https://crates.io/crates/scimantic):

```bash
cargo install scimantic
```

Verify it installed correctly:

```bash
scimantic --version
```

You should see `scimantic 0.1.x`. By the end of the book, you'll use the scimantic CLI to log into a running Scimantic instance, manage data in your knowledge graph, and query the knowledge graph from the command line.

```admonish info
If you are an "early adopter" of this book, the scimantic CLI will have very minimal functionality (*e.g.,* it prints its version and nothing else). I am dogfooding the CLI as I write the book, and features land chapter by chapter as the REST API gains endpoints.
```

```admonish info
`cargo install` downloads the `scimantic` crate from crates.io, compiles it, and places the binary in `~/.cargo/bin/` (which is on your PATH after installing Rust). This is how Rust tools ship. Later chapters use the same command to install `cargo-leptos`, `sqlx-cli`, and other tools.
```

### Why Podman?

You may already have Docker installed. That's fine; everything in this book works with Docker too. We recommend Podman for three reasons:

1. **Open source.** Podman is [Apache-2.0 licensed](https://github.com/containers/podman/blob/main/LICENSE) with no commercial licensing restrictions. Docker Desktop [requires a paid subscription](https://docs.docker.com/subscription/desktop-license/) for companies with more than 250 employees or $10M+ in annual revenue. Podman has no such restriction.
2. **Daemonless.** The [Docker engine](https://docs.docker.com/engine/security/#docker-daemon-attack-surface) runs a persistent background daemon, and the client talks to it over a socket that historically required root. Podman [runs containers directly as the invoking user](https://podman.io/) — no daemon, no root by default, smaller attack surface.
3. **Drop-in compatible.** [`podman compose`](https://docs.podman.io/en/latest/markdown/podman-compose.1.html) reads the same `compose.yaml` files as `docker compose`. Every command in this book works with either tool. If you prefer Docker, substitute `docker` wherever you see `podman`.

Install Podman if you don't have it according to the [Podman Installation Instructions](https://podman.io/docs/installation) for your operating system.

Also, verify it's working according to your OS's instructions. For MacOS, create and start your first Podman machine and verify the installation information:

```bash
podman machine init
podman machine start
podman info
```



### The Compose File

Create `compose.yaml` in the repository root:

```yaml
{{#include listings/compose-yaml-ch02-phase1.yaml}}
```

A few choices in this file are worth pausing on. Hover over any badge inline with the code above to read why we made that choice — `postgres:16` over `latest`, the explicit healthcheck, and the named `pgdata` volume.

```admonish warning
The username and password (`scimantic`/`scimantic`) are for local development only. Production credentials are managed through environment variables and AWS Secrets Manager, never committed to the repository. We'll set that up in Phase 5.
```

Start the database:

```bash
podman compose up -d
```

The `-d` flag runs it in the background. The first time, Podman will pull the PostgreSQL 16 image (~150 MB). Subsequent starts are instant.

Verify it's running:

```bash
podman compose ps
```

You should see the `db` service with status `Up` (and info on how long it's been up). 

Now connect to it:

```bash
psql postgres://scimantic:scimantic@localhost:5432/scimantic -c "SELECT version();"
```

```admonish tip
If you don't have `psql` installed locally, you can use the one inside the container:

    podman compose exec db psql -U scimantic -c "SELECT version();"
```

You should see `PostgreSQL 16.x` in the output. That's the same major version that will run in production on AWS RDS.

Stop the database when you're done:

```bash
podman compose down
```

### Phase 1 Checkpoint

You now have:

- **A GitHub repository** for your Scimantic project, cloned locally.
- **The Rust toolchain** (stable) installed on your host.
- **The Scimantic CLI** installed from crates.io.
- **Podman** installed and running.
- **PostgreSQL 16** running in a container, matching the production version on AWS RDS.

Let's check off the greenfield checklist items we completed:

- [x] GitHub repository created and cloned locally *(done)*
- [x] Rust toolchain installed on the host via `rustup` *(done)*
- [x] Scimantic CLI installed from crates.io *(done)*
- [x] Podman is the container runtime *(done)*
- [x] PostgreSQL runs in a container via `compose.yaml`, matching the production version *(done)*

No code yet, just a reproducible environment where every reader starts from the same place.

Next, we write our first Rust code: a health-check endpoint that proves the server can start.

## Phase 2: Hello-World Endpoint + Build

PostgreSQL is running. The toolchain is installed. Now we write the smallest possible Rust web server: a single endpoint at `/health` that returns the literal string `ok`. By the end of this phase you'll be running `cargo run`, hitting `localhost:3000/health` from another terminal, and seeing both `ok` and a structured log line.

This is deliberately tiny. The point is not what it does — it does almost nothing. The point is what's in place: a Cargo crate with pinned MSRV, an Axum router, a Tokio runtime, structured logging via `tracing`, and a working build. Every later chapter adds to this skeleton.

```admonish info title="Why bare Axum, not cargo-leptos?"
The Leptos team ships a `cargo leptos new` command that scaffolds an entire dual-target (server binary + WASM client) project in one shot. We don't use it yet. There's nothing for the WASM client to do until [The Web Frontend](./ch04-the-web-frontend.md), and scaffolding a UI we won't touch for two chapters obscures the small thing we *are* doing here.

Chapter 4 introduces `cargo-leptos` at the moment of need, when the first Leptos component lands. The conversion is small: add `leptos`/`leptos_axum`/`leptos_meta` dependencies, add `ssr`/`hydrate` features, add the App component. By that point you've already used Axum directly and the dual-target build has a reason to exist.
```

### Create the App Crate

From the repository root, create the `app/` directory with a single Cargo crate inside:

```bash
mkdir -p app/src
```

The `app/` directory is a peer of `book/`, `infra/`, and `schema/`. The repository itself is a monorepo, not a Cargo workspace — `app/`, `book/tools/mdbook-quiz-pdf/`, and any other Cargo crates live as independent projects that happen to share a git repo. (Chapter 3 converts `app/` itself into a Cargo workspace when the trait-based service layer makes the split worthwhile. For now, `app/` is one crate.)

### `app/Cargo.toml`

```toml
{{#include listings/cargo-toml-ch02-phase2.toml}}
```

A few things in this file are worth pausing on. Hover over any badge above to see why we made that choice.

- The crate is named **`scimantic-server`**, not `scimantic`. The unsuffixed `scimantic` name on crates.io belongs to the [Scimantic CLI](https://crates.io/crates/scimantic) you installed in Phase 1; this crate is a different artifact (a deployable web server). The `-server` suffix sets up the workspace-split naming for Chapter 3, where we'll add `scimantic-core` for trait definitions.
- **`edition = "2024"`** uses the latest Rust edition. Edition 2024 stabilizes async closures, `let-else`, and many small ergonomics improvements we'll use later.
- **`publish = false`** is a guard against accidental `cargo publish`. The web app deploys to AWS, not crates.io.
- The dependency set is minimal: Axum (HTTP server), Tokio (async runtime), and `tracing` plus `tracing-subscriber` (structured logging). We add more dependencies as each chapter needs them.

### `app/src/main.rs`

```rust
{{#include listings/main-rs-ch02-phase2.rs}}
```

The whole server is about thirty lines. The callouts walk through the moving parts: how `#[tokio::main]` boots the async runtime, how Axum's router maps a path to a handler, how `#[tracing::instrument]` attaches every log line in `health()` to a per-request span, and why we read the port from an environment variable.

```admonish tip title="Why PORT from the environment?"
Every cloud platform — AWS App Runner, Fly.io, Heroku, Cloud Run — sets a `PORT` environment variable on the container telling the app which port to bind. Hard-coding `3000` works locally; reading `PORT` works locally *and* in production. The default of `3000` matches the cargo-leptos convention we'll adopt in Ch 4, so when the time comes the local URL doesn't change.
```

### `.gitignore`: ignore the `target/` directory

Cargo writes build artifacts to a `target/` directory beside the `Cargo.toml`. In t2t the file already ignores `build/` (mdbook output) and a few other things. Add Rust build artifacts to the rule:

```
target/
```

The bare `target/` (no path prefix) matches at any depth — `app/target/`, `book/tools/mdbook-quiz-pdf/target/`, and any future Cargo crate's build directory. One rule, all crates covered.

### Run It

From `app/`:

```bash
cargo run
```

The first build pulls Axum, Tokio, and `tracing` from crates.io and compiles them; subsequent runs are quick. Once you see `scimantic-server starting`, leave it running and from another terminal:

```bash
curl http://localhost:3000/health
# ok
```

Watch the server terminal: you should see a `health: health check` log line. The `health:` prefix is the span name from `#[tracing::instrument]`; everything inside that handler now reports under that span, which is what makes a structured-logging system useful when you're staring at a thousand requests at once.

```admonish warning title="Port 3000 conflicts with mdbook serve"
If you have `mdbook serve` running for the book, it's holding port 3000. Either stop it, run `mdbook serve --port 4000` to move it aside, or run the server on a different port: `PORT=4000 cargo run` and curl `localhost:4000/health` instead.
```

Stop the server with Ctrl-C.

### Phase 2 Checkpoint

You now have:

- **A Cargo crate at `app/`** named `scimantic-server`, with MSRV 1.95, the Axum + Tokio + `tracing` dependency set, project-wide lints turned on, and `publish = false` blocking accidental release.
- **A working Axum server** with a `/health` endpoint that returns `ok`.
- **Structured logging via `tracing`** showing both startup info and per-request spans.
- **`target/` gitignored** so build artifacts don't pollute the repo.

Greenfield checklist items completed:

- [x] First Rust crate (`scimantic-server`) created at `app/`, MSRV pinned *(done)*
- [x] Axum server builds and runs locally *(done)*
- [x] Structured logging with `tracing` from the first handler *(done)*

What we deliberately *haven't* done yet — saved for the right chapter:

- **CI workflow.** Phase 4 wires `app/` into GitHub Actions alongside the security and supply-chain checks.
- **Pre-commit hooks.** Same — Phase 4.
- **`cargo-leptos` and a UI.** Chapter 4.
- **Workspace structure.** Chapter 3, when the service layer trait-ports earn the split.

Next: author the local LinkML schema for `User`, declare it in `panschema.toml`, and run `panschema fetch && panschema generate` to produce the Rust types you'll consume in Chapter 3.

<!-- ============================================================ -->
<!-- PHASE 3: Schema Foundation -->
<!-- ============================================================ -->

<!-- The reader builds: -->
<!-- - app/schema/scimantic-server.yaml — a tiny LOCAL LinkML schema with a single User class -->
<!--   (id: uri identifier, email: string required). This is the app-state schema, NOT the -->
<!--   scientific ontology — that comes from scimantic-schema in Ch 3 via panschema's schema-manager. -->
<!-- - panschema CLI installed (cargo install panschema) -->
<!-- - app/panschema.toml — first manifest, with a single [schemas] entry (path: source) for the -->
<!--   local schema. Reader's first contact with the manifest format. -->
<!-- - app/panschema.lock — first lockfile, generated by `panschema fetch`. Records the local -->
<!--   schema's checksum (so `verify` can detect "edited but not regenerated"). -->
<!-- - `panschema fetch && panschema generate` runs end-to-end and produces Rust types into -->
<!--   app/src/generated/scimantic_server.rs (or wherever the [generate.<schema>] block points). -->
<!-- - Generated types compile (not yet wired into a handler — that's Ch 3's database work) -->

<!-- The reader learns: -->
<!-- - LinkML basics by AUTHORING: classes, attributes, range types (string, uri, integer) in YAML. -->
<!-- - panschema as a schema package manager (cargo for LinkML): manifest + lockfile + fetch + -->
<!--   generate. Same conceptual model as cargo. v0.1 of the workflow uses just one schema with a -->
<!--   `path:` source — minimum complexity to demonstrate the full workflow. -->
<!-- - The manifest pattern (`panschema.toml`) and the [generate.<schema>] output mapping. -->
<!-- - Why a lockfile even with a local schema: `panschema verify` catches "edited the schema but -->
<!--   forgot to regenerate" — a common mistake. -->
<!-- - The generated code is idiomatic Rust, not opaque output. -->
<!-- - Schema-driven development as a workflow: edit YAML, run `panschema generate`, types update. -->
<!-- - The two-schema architecture is INTRODUCED but only the LOCAL schema is exercised. -->
<!--   Mention scimantic-schema as the canonical scientific ontology that Ch 3 will add as a -->
<!--   second [schemas] entry with the github: source protocol. The reader knows the manifest -->
<!--   pattern by then; Ch 3 just adds one new line and one new source protocol. -->
<!-- - Why the split matters: scientific ontology is a separate workstream with its own design -->
<!--   discipline (BFO/CCO grounding, PROV alignment) — t2t consumes it as a versioned artifact. -->
<!--   App-state schema lives with the app because its design is product-driven, not domain-driven. -->

<!-- Checkpoint: `panschema fetch && panschema generate && cargo build` succeeds with the User -->
<!-- type generated from the local schema -->
<!-- Greenfield checklist items checked off: -->
<!--   - Local LinkML schema (scimantic-server.yaml) authored -->
<!--   - panschema.toml manifest authored; panschema.lock generated -->
<!--   - panschema CLI installed and runs locally (CI integration in Phase 4) -->

<!-- DEFERRED to Ch 3 (because it lands with the database work it serves): -->
<!-- - SHACL shapes generation (those load into Oxigraph at runtime; no Oxigraph yet in Ch 2) -->
<!-- - SQL DDL generation (no PostgreSQL yet; Ch 3 wires it) -->
<!-- - scimantic-schema added as a second [schemas] entry with github: source -->
<!-- - The github: source protocol exercised end-to-end (fetch from a tagged release, lockfile -->
<!--   gains a checksum + revision entry) -->
<!-- - Hexagonal architecture callout (where the trait-based service layer earns its split) -->

<!-- ============================================================ -->
<!-- PHASE 4: CI Pipeline -->
<!-- ============================================================ -->

<!-- The reader builds: -->
<!-- - GitHub Actions workflow: build, rustfmt, clippy, cargo-nextest, cargo-audit, cargo-deny -->
<!-- - cargo-vet: supply chain vetting with trusted audit imports (Mozilla, Google, ISRG) -->
<!-- - cargo-mutants configured (--in-diff on every push, full sweep nightly) -->
<!-- - .pre-commit-config.yaml with hooks mirroring CI (rustfmt, clippy, cargo-audit, detect-private-key) -->
<!-- - prek installed and hooks enabled (cargo install prek && prek install) -->
<!-- - Dependabot configuration for automated dependency update PRs -->
<!-- - GitHub code scanning (SAST) and secret scanning enabled -->
<!-- - Scheduled weekly security workflow (cargo-audit, cargo-deny, cargo-vet) -->
<!-- - SLSA provenance attestation on release artifacts -->
<!-- - Schema generation step in the pipeline (panschema generate + verify) -->

<!-- The reader learns: -->
<!-- - GitHub Actions: workflow files, triggers, jobs, steps -->
<!-- - Pipeline-enforced quality gates: why these are gates, not suggestions -->
<!-- - rustfmt: automatic formatting, pipeline rejects unformatted code -->
<!-- - clippy: Rust linter, catches unidiomatic patterns and potential bugs -->
<!-- - cargo-audit: dependency vulnerability scanning -->
<!-- - cargo-deny: supply chain policy (license compliance, source vetting, duplicate detection) -->
<!-- - cargo-vet: supply chain vetting — importing trusted audits from established organizations, vetting new dependencies -->
<!-- - cargo-mutants: why code coverage isn't enough, --in-diff as pipeline gate, nightly full sweep -->
<!-- - prek: Rust-native pre-commit hooks, reads .pre-commit-config.yaml -->
<!-- - Why local hooks mirror CI: 5-second local feedback vs 10-minute CI failure, keeps pipeline green -->
<!-- - Private key detection in pre-commit hooks: catch accidentally committed secrets before push -->
<!-- - Dependabot: automated dependency update PRs -->
<!-- - GitHub security features: SAST, secret scanning, dependency review -->
<!-- - Scheduled security workflows: weekly scans catch CVEs disclosed between pushes -->
<!-- - SLSA provenance: signing release artifacts so users can verify they came from CI -->

<!-- Checkpoint: push to trunk, pipeline goes green (all gates pass) -->
<!-- Greenfield checklist items checked off: -->
<!--   - GitHub Actions CI pipeline runs on every push to trunk -->
<!--   - All work integrates to trunk at least daily -->
<!--   - cargo-audit scans dependencies for known CVEs on every build -->
<!--   - cargo-deny enforces supply chain policy -->
<!--   - cargo-vet vets dependencies against trusted audit sets -->
<!--   - Pre-commit hooks mirror CI checks via prek (including private key detection) -->
<!--   - Dependabot is configured for automated dependency update PRs -->
<!--   - GitHub code scanning (SAST) and secret scanning are enabled -->
<!--   - Scheduled weekly security workflow runs cargo-audit, cargo-deny, cargo-vet -->
<!--   - Release artifacts include SLSA provenance attestation -->

<!-- ============================================================ -->
<!-- PHASE 5: Infrastructure + Deployment -->
<!-- ============================================================ -->

<!-- The reader builds: -->
<!-- - Terraform configs for AWS: EC2 instance, RDS PostgreSQL, HTTPS/TLS, automated backups -->
<!-- - Staging and production environments from the same Terraform configs -->
<!-- - Deploy hello-world to production through the pipeline -->
<!-- - Secrets management: GitHub Secrets for AWS credentials, .env for local dev -->
<!-- - Local /security-review Claude Code skill and pre-commit hooks -->
<!-- - Rollback tested: deploy, break, rollback, verify -->

<!-- The reader learns: -->
<!-- - Terraform basics: providers, resources, state, HCL syntax -->
<!-- - AWS provisioning: EC2, RDS PostgreSQL, VPC, security groups, HTTPS/TLS, automated backups -->
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
<!--   - Pipeline deploys to a production-like staging environment on AWS -->
<!--   - Pipeline deploys to production on AWS -->
<!--   - Every commit that passes the pipeline is a deployment candidate -->
<!--   - Rollback is tested and works -->
<!--   - Application configuration is externalized -->
<!--   - Artifacts are immutable -->
<!--   - Database backups are automated via Terraform -->
