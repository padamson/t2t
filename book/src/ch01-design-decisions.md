# Prerequisites

```admonish warning title="Prerequisites"
Make sure you've completed [Before You Begin](./ch00-before-you-begin.md), including the two reading assignments: the [MinimumCD manifesto](https://minimumcd.org) and the [Greenfield CD guide](http://localhost:1313/docs/migrate-to-cd/greenfield/). This chapter builds directly on that material.
```

Did you actually read them? Let's find out with the following quiz.

{{#quiz ./ch01-minimumcd-quiz.toml}}

If any of those quiz answers surprised you, go back and re-read the manifesto and greenfield guide before continuing.

# Design Decisions

You're about to build a full-stack web application in Rust. Before you write a single line of code, you're going to make every major design decision and understand why each one matters.

This isn't busywork or a box-checking exercise. The design decisions we make up front are the load-bearing walls of everything that follows. Get them right, and subsequent work flows naturally. Get them wrong, and you'll spend all of your time fighting your own architecture.

You read the [greenfield CD guide](http://localhost:1313/docs/migrate-to-cd/greenfield/) in your first reading assignment. Its core message: understand *why* before building *what*. Most software projects make their hardest-to-reverse decisions in their first week, when they understand the least. We're going to be deliberate instead.

By the end of this chapter, you'll know:

- Why we build the delivery pipeline before the application
- Why a YAML schema file is the single source of truth for every data type in the system
- Why the application serves two interfaces from a single binary
- Why the UI is built from composable, themed components from day one
- Why Rust, and why *now*

No code yet. Just decisions. [Pipeline First](./ch02-pipeline-first.md) is where we start to build.

## Why Pipeline-First?

If you are new to CD, we will ask you to do something very counterintuitive in the [next chapter](./ch02-pipeline-first.md). You won't start by running `cargo leptos new`. You'll write a GitHub Actions workflow, Terraform configurations for AWS, and deploy a health-check endpoint to production. The application comes later. The pipeline comes first.

This is the core MinimumCD greenfield principle: **the delivery pipeline is feature zero.** It's not something you "set up later when there's enough code to deploy." The pipeline shapes the code, not the other way around.

Why? Because building CD into a project from the start costs almost nothing. Retrofitting it later can take months. Every team that says "we'll add CI/CD after the prototype" ends up with a prototype that's allergic to automation. The test suite assumes a specific directory structure. The deployment requires SSH and a checklist. The database migrations need to be run by hand.

As the [greenfield guide](http://localhost:1313/docs/migrate-to-cd/greenfield/) puts it: "Every one of these is trivial to add to an empty project and expensive to retrofit into a mature codebase." We take that literally.

### Feature Zero Validations

The pipeline enforces quality gates from commit one. Ours maps to the Rust toolchain like this:

**Formatting: `rustfmt`.** Automatic code formatting, enforced by the pipeline. Not a suggestion, a gate. If your code isn't formatted, it doesn't merge. This eliminates an entire category of code review friction and keeps the codebase consistent as it grows.

**Linting: `clippy`.** Rust's linter catches common mistakes, unidiomatic patterns, and potential bugs. Like `rustfmt`, it's a pipeline gate, not an optional tool.

**Type checking: the Rust compiler.** This is a selling point specific to our stack. Rust's compiler is already stricter than most languages' *entire* linting toolchains. If your code compiles, you've eliminated null pointer exceptions, data races, use-after-free bugs, and a host of other errors that other languages catch (if at all) at runtime. The pipeline's type-checking gate is the Rust compiler itself, and it's doing more work than you'd get from adding three or four tools to a Python or JavaScript project.

**Test framework: `cargo-nextest`.** Faster than `cargo test`, with parallel execution and structured output. Configured from commit one so the test infrastructure is ready before we write our first test.

**Security scanning: `cargo-audit`.** Dependency vulnerability scanning. When a CVE is published against a crate in your dependency tree, the pipeline catches it before you deploy. Add it on day one and it costs nothing. Add it on day one hundred and you're triaging a backlog of vulnerable transitive dependencies that have been in production for months.

**Supply chain policy: `cargo-deny`.** License compliance, crate source vetting, and duplicate dependency detection. This is the difference between "we use open source" and "we know exactly what open source we use, where it comes from, and what licenses we're agreeing to."

**Supply chain vetting: `cargo-vet`.** `cargo-deny` checks policy; `cargo-vet` checks provenance. It imports trusted audit sets from organizations like Mozilla, Google, and ISRG, then verifies that every dependency in your tree has been audited by someone you trust. When you add a new crate, `cargo-vet` tells you whether it's been reviewed and by whom. New, unvetted dependencies require explicit exemption.

**Mutation testing: `cargo-mutants`.** Your tests prove the code works, but do they actually catch bugs? `cargo-mutants` systematically modifies your code, replacing `+` with `-`, deleting function bodies, changing return values, and making other "mutations" while checking whether your tests notice. A surviving mutant is a line of code you can break without any test failing thus revealing a gap in your safety net. The pipeline runs `cargo mutants --in-diff` on every push, mutation-testing only the changed code. A full mutation sweep runs nightly to catch accumulated gaps.

**Automated dependency updates: Dependabot.** Configured at project creation. When a dependency publishes a security fix, you get a PR within hours. No human has to remember to check.

**GitHub security features.** Code scanning (SAST) and secret scanning enabled on the repository. These are free, built into GitHub, and catch categories of mistakes that careful coding alone won't prevent: accidentally committed API keys, known vulnerability patterns in code, insecure dependency configurations.

### Local Hooks Mirror CI

Every check the pipeline runs, you can run locally before you push. The tool that makes this practical is [prek](https://github.com/j178/prek), a Rust-native pre-commit hook runner. You define your checks once in `.pre-commit-config.yaml`, and the same file drives both local hooks (`prek run`) and the CI pipeline step.

Why bother? Because a 5-second local check is better than a 10-minute CI failure. If rustfmt or clippy catches something before you push, the pipeline stays green. "All feature work stops when the pipeline is red" is a lot easier to follow when the pipeline is rarely red. The hooks aren't a gate that replaces CI. CI is still the authority. The hooks are a fast feedback loop that keeps you from wasting CI time on problems you could have caught at your desk.

`prek` reads the same `.pre-commit-config.yaml` format used by the Python pre-commit framework (the industry standard), but it's written in Rust, installs with `cargo install prek`, and runs hooks significantly faster. No Python runtime required.

### Shift-Left Security: Two Layers

Security in this project works at two levels. Locally, while you're writing code, an AI-assisted security review (a Claude Code skill) scans your changes for application-logic vulnerabilities: injection patterns, missing authentication checks, XSS risks, hardcoded secrets. You're in the loop. You see every finding and decide how to respond.

In the pipeline, deterministic tools do the same work independently: cargo-audit, cargo-deny, Dependabot, GitHub code scanning. These don't depend on an API call. They run the same way every time. They're auditable.

The two layers complement each other. The local review catches things static tools miss (like a raw SQL string that *looks* parameterized but isn't). The pipeline catches things you might overlook during development (like a transitive dependency with a known CVE). Neither layer alone is sufficient.

### The Testing Pyramid

Every test in Scimantic is Rust code. Behavioral specifications are test functions: a test named `user_can_pose_research_question()` is both the spec and the verification. When the agent reads it, it knows what to implement. When the pipeline runs it, it knows whether the implementation is correct.

The pyramid has nine layers, each serving a different purpose and running at a different speed:

| Layer | Tool | What it tests |
|-------|------|---------------|
| **Doc tests** | `cargo test --doc` | API examples in documentation compile and run. If the API changes, the docs break. Spec drift is impossible. |
| **Unit tests** | cargo-nextest | Service layer logic, validation rules, schema-generated types, pure functions. Fast, no external dependencies. |
| **Component tests** | dokime | Leptos component rendering, signal reactivity, event handling. No browser required. |
| **Integration tests** | cargo-nextest + SQLx fixtures | Service layer against a real PostgreSQL database. Catches query bugs, migration issues, transaction edge cases. |
| **Contract tests** | cargo-nextest | REST API responses conform to the OpenAPI spec. Catches contract drift between the API and its documentation. |
| **Security E2E (DAST)** | playwright-rust | Probes the running application for injection, XSS, and auth bypass vulnerabilities. Runs against staging. |
| **E2E tests** | playwright-rust | Full user flows in a real browser. Multi-step interactions, page navigation, data persistence across reloads. |
| **Visual regression** | theoria + playwright-rust | Component screenshots diffed against baselines. Catches rendering regressions without a dedicated visual testing tool. |
| **Mutation testing** | cargo-mutants | Tests catch real bugs, not just exercise code paths. Mutants that survive reveal untested behavior. Runs incrementally (`--in-diff`) on every push; full sweep nightly. |

The fast layers (doc tests, unit tests, component tests) run in milliseconds and give immediate feedback during development. The slow layers (integration, contract, security, E2E, visual regression) run in seconds and are pipeline gates. Mutation testing sits alongside the pipeline: incremental on every push, full sweep nightly. Together they cover the full stack from individual functions to deployed user flows.

### The ACD Workflow

```admonish reference title="Reading Assignment"
Read these two pages only (as always, don't follow any other links on the site):

1. [localhost:1313/docs/agentic-cd/](http://localhost:1313/docs/agentic-cd/) — the ACD overview page
2. [localhost:1313/docs/agentic-cd/specification/first-class-artifacts/](http://localhost:1313/docs/agentic-cd/specification/first-class-artifacts/) — the full artifact definitions

Come back here when you're done.
```

Did you read both pages? Let's check.

{{#quiz ./ch01-acd-quiz.toml}}

Agentic Continuous Delivery, the framework you just read about, is the workflow that structures this entire book. ACD extends Continuous Delivery with [eight constraints](http://localhost:1313/docs/agentic-cd/) and a set of [delivery artifacts](http://localhost:1313/docs/agentic-cd/specification/first-class-artifacts/) that anchor human-agent collaboration. The [ACD workflow](http://localhost:1313/docs/agentic-cd/) defines eleven stages:

1. **Intent Description.** A human drafts a problem statement and hypothesis. An agent finds ambiguity and suggests edge cases.
2. **User-Facing Behavior.** The human defines and approves BDD scenarios. The agent generates scenario drafts and finds gaps.
3. **Feature Description.** The human sets constraints and architectural boundaries. The agent suggests architectural considerations and integration points.
4. **Acceptance Criteria.** The human defines thresholds and evaluation design. The agent drafts non-functional criteria and checks cross-artifact consistency.
5. **Specification Validation.** Before implementation begins, the agent reviews all four specification artifacts for conflicts, gaps, and ambiguity. The human gates entry to implementation.
6. **Test Generation.** The agent generates test code from the BDD scenarios, feature description, and acceptance criteria.
7. **Test Validation.** The human reviews the generated tests. Over time, expert validation agents progressively replace human review.
8. **Implementation.** The agent generates production code within a small-batch session per scenario.
9. **Pipeline Verification.** The pipeline runs all tests. All scenarios implemented so far must pass.
10. **Code Review.** The human reviews the implementation. Over time, expert validation agents progressively replace human review.
11. **Deployment.** The pipeline deploys through the same path as every other change.

The key ACD principles governing every chapter:

- **Explicit, human-owned intent exists for every change.** The human defines the *why*. The agent helps figure out the *how*.
- **Intent and architecture are represented as delivery artifacts.** It exists in the repo under version control and is machine-readable. It is not in the developer's head or in a separate wiki.
- **Consistency between intent, tests, implementation, and architecture is enforced.** The pipeline verifies this. It's not a matter of discipline.
- **While the pipeline is red, agents may only generate changes restoring pipeline health.** No new features until the build is green.

You'll execute the full ACD workflow for the first time in [Completing the Slice](./ch05-completing-the-slice.md), when we build the "pose a research question" feature end-to-end. But the pipeline and constraints are in place from [Pipeline First](./ch02-pipeline-first.md).

## Why Schema-Driven?

In a typical Rust web application, you maintain the data model in three places:

1. **Rust structs**, with `serde::Serialize`, `serde::Deserialize`, `sqlx::FromRow`, and `utoipa::ToSchema` derives stacked on top.
2. **SQL migrations**, `CREATE TABLE` statements that must agree with the Rust types.
3. **JSON Schema or OpenAPI definitions**, the API contract for external consumers.

These three representations describe the same thing. When you change one, you must change the others. If you forget, nothing catches the inconsistency until a test fails (if you're lucky) or a production user hits a 500 error (if you're not).

The typical Rust approach is to maintain separate struct layers (API DTOs, domain models, database row types) connected by `From`/`Into` trait implementations. This works. It's idiomatic. And it depends entirely on developer discipline to stay consistent.

Other ecosystems have partially solved this. Prisma gives TypeScript developers a single `.prisma` schema that generates types, a query client, and versioned migrations. AWS's Smithy generates Rust types and API specs from a single `.smithy` model, but it has no database awareness. SeaORM 2.0 lets you sync Rust entity structs to the database at runtime, but it doesn't produce versioned migration files or OpenAPI schemas. No existing tool bridges all three layers for a Rust web application.

### The Schema as an Architecture Artifact

The ACD framework requires that architecture be represented as versioned delivery artifacts. For data modeling, this means the schema should be the source of truth, not the Rust code, not the SQL, not the OpenAPI spec.

Scimantic's domain model has two homes. The scientific ontology — the classes, relationships, and constraints that define what a Question, Evidence, Hypothesis, Experiment, and Result *are* — lives in the [scimantic-ontology](https://github.com/padamson/scimantic-ontology) repo as a versioned [LinkML](https://linkml.io/) schema. The application data model (users, sessions, app configuration) lives in the main repo. Both are YAML-based, and both flow through the same tool.

Here's what a core entity looks like in LinkML:

```yaml
classes:
  Question:
    attributes:
      id:
        range: uri
        identifier: true
      text:
        range: string
        required: true
      status:
        range: QuestionStatus
        required: true
      domain:
        range: string

enums:
  QuestionStatus:
    permissible_values:
      open: {}
      investigating: {}
      resolved: {}
```

From these schemas, [panschema](https://github.com/padamson/panschema) generates:

- **Rust structs** with the appropriate `serde`, `sqlx`, and `utoipa` derives.
- **SQL DDL** for database migrations (PostgreSQL tables for app state).
- **JSON Schema** for API contract validation.
- **SHACL shapes** for validating RDF data in the knowledge graph.

When the schema changes, the pipeline regenerates all downstream artifacts and verifies consistency. If the generated Rust types don't match the SQL migrations, the build breaks. If the SHACL shapes don't match the ontology, the build breaks. If the JSON Schema doesn't match the API response types, the build breaks. Inconsistency is caught at build time, not in production.

### Why Not Just Hand-Write Everything?

You could. Many excellent Rust applications do. But consider the trajectory of this book's philosophy:

- **SQLx** already does this for queries: it checks column names and types at compile time. Nobody hand-verifies `SELECT` results against struct fields.
- **utoipa** does the same for API contracts: your OpenAPI spec is generated from Rust types, not maintained in a separate file.
- **panschema** extends this to the data model itself. The Rust types, SQL DDL, SHACL shapes, and JSON Schema are all generated from one LinkML definition. The pipeline verifies consistency across all of them.

The theme is the same at every layer: **catch inconsistencies at build time, not in production.** Schema-driven development is a natural extension of the same philosophy that makes SQLx and utoipa compelling. It's Rust's build-time verification story applied to the data model itself.

## Why This Architecture?

Scimantic needs to serve two kinds of clients:

1. **A web frontend** for desktop and mobile browsers, server-rendered HTML with client-side interactivity.
2. **A REST API** for external consumers — CLI tools, Jupyter notebook integrations, data pipeline scripts, and other research tooling that interacts with the knowledge graph programmatically.

This dual requirement drives the architecture.

### The Problem with a Single Interface

Leptos, the full-stack Rust framework we're using, provides `#[server]` functions. These let you write code that runs on the server but call it transparently from the client. They're elegant for the web frontend: you write a function that queries the database, and Leptos handles serializing the result across the client/server boundary.

But Leptos server functions are not REST endpoints. They use Leptos's own serialization protocol. A Python script running a SPARQL query can't call them. Neither can any HTTP client that isn't a Leptos frontend.

We need standard, OpenAPI-documented REST endpoints alongside the Leptos frontend. And both need to agree on data shapes, validation rules, and business logic.

### The Single-Binary, Dual-Interface Design

Leptos runs on top of Axum; they share the same server process, Tokio runtime, and router. This means we can serve both interfaces from a single binary:

```text
┌─────────────────────────────────────────────────┐
│               Single Axum Server                │
│                                                 │
│  ┌────────────────────┐  ┌───────────────────┐  │
│  │  Leptos Routes     │  │  REST API Routes  │  │
│  │  (SSR + WASM)      │  │  (utoipa OpenAPI) │  │
│  │  server functions  │  │  /api/v1/*        │  │
│  └─────────┬──────────┘  └─────────┬─────────┘  │
│            │                       │            │
│            ▼                       ▼            │
│  ┌───────────────────────────────────────────┐  │
│  │          Shared Service Layer             │  │
│  │    (domain logic, validation, auth)       │  │
│  └──────────┬────────────────────┬───────────┘  │
│             ▼                    ▼              │
│  ┌────────────────────┐  ┌───────────────────┐  │
│  │  SQLx + PostgreSQL │  │     Oxigraph      │  │
│  │   (app state)      │  │ (knowledge graph) │  │
│  └────────────────────┘  └───────────────────┘  │
└─────────────────────────────────────────────────┘
```

The key is the **shared service layer**. All domain logic (validation, authorization, business rules) lives in one place. Both the Leptos server functions and the REST API handlers call into the same service layer. Neither interface implements business logic directly.

Below the service layer, two databases serve different purposes. **PostgreSQL** stores application state: users, sessions, configuration, and any relational data that doesn't belong in a graph. **Oxigraph**, a Rust-native RDF triple store with full SPARQL 1.1 support, stores the knowledge graph: questions, evidence, hypotheses, experiments, results, and all the relationships between them. The service layer abstracts this split. A request to "show me all evidence linked to this question" queries Oxigraph via SPARQL. A request to "update the current user's profile" queries PostgreSQL via SQLx. The caller doesn't know or care which database answered.

- **Domain logic is tested once.** You don't write separate tests for the web frontend's "pose a question" logic and the API's "pose a question" logic. They call the same function.
- **Both interfaces are guaranteed to agree on data shapes.** The Rust type system enforces this: if the service layer returns a `Question`, both interfaces get the same `Question`. No drift.
- **The REST API gets OpenAPI documentation generated at compile time** from the same Rust types the Leptos frontend uses. The API contract is enforced by the compiler.
- **The dual database is invisible to consumers.** Whether data lives in PostgreSQL or Oxigraph is a service-layer concern. The API returns the same JSON regardless of which store answered the query.

### Technology Choices as CD Constraints

Every technology choice in this book maps back to a Continuous Delivery constraint:

| Choice | CD Constraint |
|--------|---------------|
| **PostgreSQL** over SQLite | MinimumCD requires production-like environments from day one. SQLite in dev, Postgres in prod is the kind of divergence that hides bugs. |
| **Oxigraph** (Rust-native RDF store) | The knowledge graph runs in-process, embeds via RocksDB, and supports full SPARQL 1.1. No Java/Python triple store to manage separately. Same binary, same deployment, same pipeline. |
| **SQLx** (compile-time checked queries) | The pipeline catches database contract violations before deployment. A typo in a column name breaks the build, not the user's session. |
| **Terraform + AWS** | Everything-as-code. Infrastructure lives in the same repo and flows through the same pipeline. No snowflake servers, no manual provisioning. |
| **utoipa** (compile-time OpenAPI) | The API contract is generated from Rust types, not maintained separately. Contract drift is impossible. |
| **playwright-rust** | E2E testing in Rust. The testing tool is written in the same language as the application. The entire testing story (unit, integration, E2E) is Rust. |
| **cargo-mutants** | Tests must demonstrate they catch regressions. Code coverage measures what runs; mutation testing measures what's *verified*. Incremental on every push; full sweep on a schedule. |
| **panschema + LinkML** | The data model is a versioned architecture artifact. The pipeline generates and verifies all downstream representations: Rust types, SQL DDL, SHACL shapes, JSON Schema. |
| **scimantic-ontology** (separate repo) | The domain ontology is a versioned artifact with its own release cycle. The app depends on a pinned version. Ontology changes flow through the pipeline like any other dependency update. |
| **cargo-deny** | Supply chain policy as code. License compliance, crate source vetting, and duplicate detection are pipeline gates, not afterthoughts. |
| **cargo-vet** | Supply chain vetting with trusted audit imports. Every dependency verified by someone you trust (Mozilla, Google, ISRG). New, unvetted dependencies require explicit exemption. |
| **Dependabot + GitHub Security** | Automated dependency updates and static analysis. Security scanning that doesn't depend on developer memory. |
| **prek** (pre-commit hooks) | Local hooks mirror CI checks. Catch formatting, lint, and security issues in seconds before pushing, keeping the pipeline green. Rust-native, reads the industry-standard `.pre-commit-config.yaml`. |
| **Podman + compose.yaml** | The local PostgreSQL runs in a container matching the production version. No environment divergence. |
| **Tailwind CSS v4** | Utility-first CSS with a Rust-native standalone CLI. Component styles are Leptos components composing Tailwind utilities. No third-party CSS framework, no Node.js dependency. The Scimantic theme is a versioned artifact. |
| **theoria** | A component catalog where you browse, configure, and document every UI component in isolation. Ensures components are reusable and well-documented as the project grows. |
| **dokime** | Fast component-level testing without a full browser. Verifies rendering and signal reactivity for every component theoria catalogs. Catches regressions without the overhead of E2E tests. |
| **tracing** | Structured logging from day one. The pipeline verifies code is correct at build time; `tracing` shows what's happening at runtime. You can't do canary deployments without observability to compare. |
| **thiserror** | Domain-specific error types with context. Errors carry enough information to diagnose problems from logs without reproducing them locally. |

## Why Component-Driven?

Scimantic has several distinct UI surfaces: question boards where researchers pose and track questions, evidence timelines that link literature and observations to questions, hypothesis trees that visualize how evidence supports or refutes competing explanations, and experiment trackers that follow methodology from design through results. Without a deliberate approach to UI, every chapter would reinvent button styles, form layouts, and spacing. The result would be a codebase where no two pages look the same.

We build the UI from composable Leptos components from the start. A Button component introduced in [The Web Frontend](./ch04-the-web-frontend.md) is the same Button used in [Results & Analysis](./ch11-results-and-analysis.md). A Card component that displays a research question also displays a piece of evidence or an experiment summary. The components are small, reusable, and tested in isolation.

[Tailwind CSS v4](https://tailwindcss.com) provides the styling foundation. Its standalone CLI is written in Rust (using Lightning CSS), so it requires no Node.js runtime. Tailwind scans your Leptos `view!` macros for class names and generates only the CSS you actually use. We define a Scimantic theme (color palette, typography scale, spacing tokens) in Tailwind's configuration, then compose those utilities inside Leptos components. A `Button` component isn't a CSS class name; it's a Rust function that encapsulates a specific combination of Tailwind utilities, accepts typed props (`variant`, `size`, `disabled`), and renders consistently everywhere it's used.

This approach keeps the entire build chain Rust-native and eliminates a third-party CSS framework from the dependency tree. The component styles are *yours*, defined in your codebase, tested by your pipeline.

For component development, we use [theoria](https://github.com/padamson/theoria) (Greek: theoria, "a journey to witness a spectacle"), a Rust-native component catalog. It provides a dedicated route where you can browse every component, configure its props, and see the rendered output in isolation. When you build a `Card` component that will be used on both the question board and the evidence timeline, theoria lets you develop and refine it independently before it touches either page. You'll see it introduced in [The Web Frontend](./ch04-the-web-frontend.md) and used throughout the book.

For component testing, we use [dokime](https://github.com/padamson/dokime) (Greek: dokime, "proof by fire"), a Rust-native component testing framework. It verifies rendering, signal reactivity, and event handling for every component theoria catalogs, without booting a full browser. A `Button` with three variants and two sizes has six prop combinations; dokime tests them all in milliseconds. Together with standard `#[test]` functions for domain logic and playwright-rust for full E2E flows, dokime covers the middle of the testing pyramid that would otherwise require slow browser-based tests.

```admonish tip
theoria and dokime are being built alongside this book. If you're reading an early draft, their repos may be empty or incomplete (or non-existent!). They'll be ready by [The Web Frontend](./ch04-the-web-frontend.md), where we introduce them.
```

## Why Rust, and Why Now?

Two years ago, recommending Rust for a full-stack web application would have required caveats. The frameworks were young. The ecosystem had gaps. You could build a backend, sure, but "full-stack" meant reaching for JavaScript or TypeScript on the frontend.

That's no longer true:

- **Leptos** provides the full-stack story: server-side rendering, WebAssembly hydration, and `#[server]` functions that bridge the client/server boundary. It's pre-1.0 and the API is still settling, but the core model is stable and the community is active.
- **Axum** is the de facto standard backend framework, built by the Tokio team. Its extractor pattern takes getting used to, but once you learn it, routing and request handling are concise. It's the foundation everything else runs on.
- **SQLx** provides compile-time checked database queries. Your SQL is verified against the real database schema during `cargo build`, not at runtime. A mistyped column name or a type mismatch between your struct and the table stops the build before you can deploy it.
- **Oxigraph** provides an embeddable, Rust-native RDF triple store with full SPARQL 1.1 support. It runs in-process, stores data via RocksDB, and handles Turtle, N-Triples, and JSON-LD serialization. No Java runtime, no separate server process. The knowledge graph is part of the binary.
- **utoipa** generates OpenAPI specifications at compile time from your Rust types. The API documentation is always correct because it's generated from the same code that serves the API.
- Cross-browser E2E testing is possible from Rust now, via **playwright-rust**. Chromium, Firefox, WebKit, driven from `#[test]` functions. No JavaScript test runner required.
- **panschema** generates Rust types, SQL, SHACL shapes, and JSON Schema from a single LinkML model. It's early-stage software; we'll be extending it as we go, and you'll see that process firsthand.

But the ecosystem is only half the story. The other half is what Rust's type system and compiler do for your delivery pipeline.

Rust's toolchain isn't simpler than other stacks. You still need a formatter, a linter, a test runner, security scanners, and mutation testing. What's different is the foundation those tools build on. Null pointer exceptions, data races, type coercion surprises, missing error handling: Rust's compiler rejects all of these before your code reaches the test suite. Your linter isn't hunting for null checks you forgot. Your tests aren't catching type mismatches that slipped through. The compiler has already eliminated those categories, so every tool in the pipeline is working on a stronger base. The result is a more robust pipeline with faster feedback: when a test fails, it's testing your *logic*, not catching a bug the compiler should have caught.

And then there's the convergence that makes this book timely: **AI coding assistants are making Rust accessible to more developers, and test-driven development is experiencing a renaissance as the optimal way to work with AI agents.** The ACD workflow, where humans define intent and agents generate implementation under pipeline supervision, plays to Rust's strengths. A strict compiler gives the agent a tighter feedback loop. Code that doesn't satisfy the types is rejected instantly at build time, not discovered later in review.

## What You're Going to Build

Scimantic is a scientific knowledge management platform — think [Indico](https://getindico.io/) (CERN's conference management system) but for the scientific workflow itself. It's open-source software that researchers can self-host, and it's also offered as a paid hosted service for individuals and organizations who'd rather not run their own infrastructure.

We chose this domain because:

- **It's distinctive.** There is no existing "build a scientific knowledge graph in Rust" book. The domain exercises capabilities (ontologies, RDF, SPARQL, dual databases, graph traversal) that a generic CRUD app never touches. If you finish this book, you will have built something no tutorial has covered before.
- **It deeply exercises the stack.** A knowledge graph demands complex relationships, graph queries, ontology validation, and dual-database coordination. Every layer of the architecture earns its keep. The schema-driven approach isn't a nice-to-have; it's essential when your data model is an ontology.
- **It decomposes into natural vertical slices.** Each user story maps to a thin, independently deployable increment — exactly what CD demands.
- **The author is a scientist.** This isn't a contrived teaching example. It's a tool the author will use in their own research. That alignment between dogfooding and career means the domain gets the depth it deserves.
- **It's a real product.** By the end of this book, Scimantic is deployed, usable, and serving real researchers. The open-source core is the codebase readers build. A thin commercial layer (hosting, billing) sits on top but is outside the book's scope.

### The Product Vision

Scimantic helps researchers manage the full lifecycle of scientific inquiry. The core workflow follows the scientific method:

1. **Pose a question.** What do you want to know?
2. **Gather evidence.** What data, literature, and observations bear on the question?
3. **Form a hypothesis.** Given the evidence, what testable prediction can you make?
4. **Design an experiment.** How will you test the hypothesis?
5. **Record results.** What did the experiment show, and does it support or refute the hypothesis?

Each step produces an entity in a knowledge graph. The relationships between entities — which evidence supports which hypothesis, which experiment tests which hypothesis, which results confirm or contradict — are first-class data, stored as RDF triples and queryable via SPARQL. Over time, a researcher's Scimantic instance becomes a navigable history of their scientific thinking.

### User Stories

These are the user stories we'll build through the course of the book. Each story is a vertical slice: independently deployable, testable end-to-end, and valuable on its own.

**Questions (Part I — The First Vertical Slice)**

- *As a researcher, I want to pose a research question so that I can begin a structured inquiry.*
- *As a researcher, I want to tag questions with scientific domains so that I can organize my work.*
- *As a researcher, I want to track a question's status (open → investigating → resolved) so that I can see which inquiries are active.*

**Evidence (Ch 7)**

- *As a researcher, I want to link a literature reference to a question so that I can track what I've read.*
- *As a researcher, I want to link a dataset or observation to a question so that empirical data is connected to the inquiry it informs.*
- *As a researcher, I want to see all evidence for a question so that I can assess whether I have enough to form a hypothesis.*

**User Authentication (Ch 8)**

- *As a researcher, I want to create an account and log in so that my work is private and persistent.*
- *As a researcher, I want to control who can view or edit my questions and evidence so that I can collaborate selectively.*

**Hypotheses (Ch 9)**

- *As a researcher, I want to form a hypothesis linked to supporting evidence so that my reasoning is traceable.*
- *As a researcher, I want to see the evidence chain behind any hypothesis so that I can evaluate its foundation.*
- *As a researcher, I want to collaborate on hypotheses with other researchers in real time.*

**Experiments (Ch 10)**

- *As a researcher, I want to design an experiment linked to a hypothesis so that I can plan how to test it.*
- *As a researcher, I want to record methodology, variables, and expected outcomes so that the experiment is reproducible.*
- *As a researcher, I want to track an experiment's status (planned → in-progress → completed) so that I know what's running.*

**Results & Analysis (Ch 11)**

- *As a researcher, I want to record experimental results and link them to the hypothesis they tested so that the outcome is part of the knowledge graph.*
- *As a researcher, I want to mark whether results support or refute a hypothesis so that the graph captures verdicts.*
- *As a researcher, I want to trace the full path from question to result so that I can see the complete arc of an inquiry.*

### From User Stories to Vertical Slices

Each group of user stories maps to a chapter, and each chapter delivers a deployable increment:

| Chapter | Slice | User stories | What ships |
|---------|-------|-------------|------------|
| Ch 2–6 | **Questions** | Pose, tag, track status | Full stack: pipeline, database, UI, REST API, E2E tests |
| Ch 7 | **Evidence** | Link literature/data, view per-question | Second entity, first graph relationships, SPARQL queries |
| Ch 8 | **Authentication** | Sign up, log in, access control | Session auth, route protection, authorization |
| Ch 9 | **Hypotheses** | Form, trace evidence chain, collaborate | Multi-hop graph queries, real-time updates |
| Ch 10 | **Experiments** | Design, track methodology, status workflow | Complex forms, state machines, cross-entity queries |
| Ch 11 | **Results** | Record, link to hypothesis, full traversal | Both databases bridged, aggregation, benchmarks |
| Ch 12–14 | **Production hardening** | — | Security, feature flags, progressive rollout |

The first vertical slice — "a researcher can pose a question" — takes six chapters. That's not slow; that's thorough. Those six chapters build the entire stack: pipeline, dual database, web frontend, E2E tests, REST API. Every subsequent slice builds faster because the scaffolding is in place.

### The Knowledge Graph

Two databases serve different purposes:

- **Oxigraph** stores the scientific entities and all relationships between them as RDF triples, queryable via SPARQL. This is the knowledge graph — the thing that makes Scimantic more than a CRUD app.
- **PostgreSQL** stores application state: users, sessions, and configuration. This is the infrastructure that makes Scimantic a product.

The dual-database architecture is invisible to the user; the service layer handles routing queries to the right store.

```admonish info title="Architecture Decision Records"
Some teams document decisions like the ones in this chapter using Architecture Decision Records (ADRs). These are short markdown files with a Status, Context, Decision, and Consequences section, stored in the repo under `docs/adr/`. It's a widely adopted practice (ThoughtWorks has recommended it since 2016) and the format is useful: six months later, nobody remembers *why* SQLite was rejected, and the ADR captures that. The risk is that ADRs rot. Teams write them enthusiastically for a month, then stop, and stale ADRs mislead more than they help. In this book, the chapters themselves serve as our decision record. Every technology choice has its rationale right here in the narrative. If you adopt ADRs in your own projects, keep them short, update them when decisions change, and delete them when they're no longer relevant.
```

## The Greenfield Checklist

Here's what we're going to build, adapted from the [greenfield checklist](http://localhost:1313/docs/migrate-to-cd/greenfield/) to our specific stack. This serves as both a preview and a roadmap. The chapter annotation on each item tells you when we'll get there. At the end of every chapter, we'll revisit this list and check off what we completed.

### Development Environment

- [ ] GitHub repository created and cloned locally *(Ch 2)*
- [ ] Rust toolchain installed on the host via `rustup` *(Ch 2)*
- [ ] Scimantic CLI installed from crates.io via `cargo install scimantic` *(Ch 2)*
- [ ] Podman is the container runtime *(Ch 2)*
- [ ] PostgreSQL runs in a container via `compose.yaml`, matching the production version *(Ch 2)*
- [ ] Oxigraph runs embedded in the application binary (no separate container needed) *(Ch 2)*

### Pipeline Basics

- [ ] GitHub Actions CI pipeline runs on every push to trunk *(Ch 2)*
- [ ] `cargo leptos build` compiles, tests, and packages with a single command *(Ch 2)*
- [ ] All work integrates to trunk at least daily *(Ch 2)*
- [ ] Deployment to staging is automated via Terraform + GitHub Actions *(Ch 2)*
- [ ] LinkML schema is versioned in the repo; panschema generates types, migrations, and SHACL shapes in CI *(Ch 2)*
- [ ] scimantic-ontology is a pinned dependency; ontology updates flow through the pipeline *(Ch 2)*
- [ ] Structured logging with `tracing` from the first handler *(Ch 2)*
- [ ] Pre-commit hooks mirror CI checks via `prek` *(Ch 2)*
- [ ] First unit test exists and passes *(Ch 3)*

### Security

- [ ] `cargo-audit` scans dependencies for known CVEs on every build *(Ch 2)*
- [ ] `cargo-deny` enforces supply chain policy (licenses, sources, duplicates) *(Ch 2)*
- [ ] `cargo-vet` vets dependencies against trusted audit sets *(Ch 2)*
- [ ] Scheduled weekly security workflow runs cargo-audit, cargo-deny, cargo-vet *(Ch 2)*
- [ ] Release artifacts include SLSA provenance attestation *(Ch 2)*
- [ ] Dependabot is configured for automated dependency update PRs *(Ch 2)*
- [ ] GitHub code scanning (SAST) and secret scanning are enabled *(Ch 2)*
- [ ] Secrets are managed via environment variables and GitHub Secrets, never committed *(Ch 2)*
- [ ] HTTPS/TLS is configured on all deployed environments *(Ch 2)*
- [ ] Local `/security-review` skill and pre-commit hooks are set up for the application repo *(Ch 2)*
- [ ] Security-focused E2E tests (DAST) run against staging after each deployment *(Ch 5)*
- [ ] Secrets rotation is automated for database and API credentials *(Ch 12)*

### Quality Gates

- [ ] Pipeline deploys to a production-like staging environment on AWS *(Ch 2)*
- [ ] Rollback is tested and works *(Ch 2)*
- [ ] Application configuration is externalized (environment variables, not baked into the binary) *(Ch 2)*
- [ ] Artifacts are immutable (single binary built once, deployed to staging and production) *(Ch 2)*
- [ ] Doc tests verify all public API examples *(Ch 3)*
- [ ] Unit tests cover service layer and domain logic (cargo-nextest) *(Ch 3)*
- [ ] Integration tests run against real PostgreSQL (SQLx test fixtures) *(Ch 3)*
- [ ] Mutation testing verifies test suite catches regressions (cargo-mutants `--in-diff` on every push; full sweep nightly) *(Ch 3)*
- [ ] External dependencies use test doubles in the deterministic test suite *(Ch 3)*
- [ ] Domain-specific error types with context (not raw strings) *(Ch 3)*
- [ ] Component tests verify Leptos rendering and reactivity (dokime) *(Ch 4)*
- [ ] Accessibility: WCAG 2.1 AA compliance, semantic HTML, ARIA, keyboard navigation *(Ch 4)*
- [ ] E2E tests verify full user flows in a real browser (playwright-rust) *(Ch 5)*
- [ ] Security E2E tests (DAST) probe staging for vulnerabilities (playwright-rust) *(Ch 5)*
- [ ] Contract tests verify REST API conforms to OpenAPI spec *(Ch 6)*
- [ ] CORS policy is configured for the REST API *(Ch 6)*
- [ ] Input validation enforced at the API boundary *(Ch 6)*
- [ ] API endpoints are paginated with cursor-based pagination *(Ch 6)*
- [ ] HTTP caching headers on read endpoints *(Ch 6)*
- [ ] SPARQL queries against Oxigraph are tested with known graph fixtures *(Ch 7)*

### Production Readiness

- [ ] Pipeline deploys to production on AWS *(Ch 2)*
- [ ] Every commit that passes the pipeline is a deployment candidate *(Ch 2)*
- [ ] Deployment is a routine, low-risk event *(Ch 5)*
- [ ] Performance benchmarks run in CI (criterion); regressions block the pipeline *(Ch 11)*
- [ ] Database backups are automated via Terraform *(Ch 2)*
- [ ] CSP headers and rate limiting are configured *(Ch 12)*
- [ ] Database restore process is tested against real data *(Ch 12)*
- [ ] Oxigraph backup and restore is automated alongside PostgreSQL *(Ch 12)*
- [ ] Feature flags decouple deployment from release *(Ch 13)*
- [ ] Load testing establishes baseline capacity and failure modes *(Ch 14)*
- [ ] Observability: tracing spans and metrics support canary comparison *(Ch 14)*
- [ ] DORA metrics are tracked *(Ch 14)*

Every checkbox is something we will build, in order, through the course of this book. At the end of each chapter, we'll come back to this list and check off the items we completed. By the final chapter, they'll all be done.

Next up: we build the pipeline.
