# Trunk to Theory

## Book Planning Document

**Title:** *Trunk to Theory: Building a Full-Stack Rust Application with Agentic Continuous Delivery*

**Short form (if needed):** *Trunk to Theory: Full-Stack Rust with Agentic CD*

**What this is:** A dogfooding exercise for a complete Rust-based toolchain for full-stack web apps. The author builds and maintains some of the key tools in this stack — [playwright-rust](https://github.com/padamson/playwright-rust) (E2E testing), [panschema](https://github.com/padamson/panschema) (schema-driven development), [theoria](https://github.com/padamson/theoria) (component explorer for Leptos), and [dokime](https://github.com/padamson/dokime) (component testing for Leptos). The book and Trunk to Theory are the proving ground: build a real product with these tools, find the gaps, fix them.

**Publishing model:** The book and Trunk to Theory are open source. The full interactive web version is freely available via mdbook. A print/ebook edition through No Starch Press (following the *Rust Programming Language* model) is a possible spinoff but not the primary goal.

**Manuscript Format:** Markdown via mdbook (Rust-based). Code snippets are pulled from the application source via mdbook `{{#include}}` directives with ANCHOR comments, following the approach used by *The Rust Programming Language* book. Admonitions via mdbook-admonish. Code callouts and print-quality PDF output are planned via custom mdbook preprocessors/backends (Rust). The entire book toolchain is Rust-based, consistent with the book's thesis.

---

## Progress

Statuses: `scaffold` → `outlining` → `drafting` → `draft` → `review` → `final`

| Ch | Title | Status | t2t tag | Code scope | Depends on | Notes |
|----|-------|--------|--------------------|------------|------------|-------|
| 00 | Before You Begin | scaffold | — | — | — | No code; reading assignments + local MinimumCD setup |
| 01 | Design Decisions | draft | `chapter-01` | CLAUDE.md, design docs | — | Prose complete, needs review |
| 02 | Pipeline First | scaffold | `chapter-02` | Devcontainer, compose.yaml, Axum health endpoint, LinkML schema, panschema, GH Actions, Terraform, tracing setup | — | First code chapter; heaviest setup |
| 03 | The Database | scaffold | `chapter-03` | PostgreSQL via Terraform, Oxigraph setup, questions migration, service layer, thiserror, first unit test, first cargo-mutants run | ch02 tagged | |
| 04 | The Web Frontend | scaffold | `chapter-04` | Leptos components (Button, Input, Card, Layout), Tailwind v4 theme, theoria, dokime, questions page, accessibility | ch03 tagged | |
| 05 | Completing the Slice | scaffold | `chapter-05` | Add-question form, server action, playwright-rust E2E + DAST, mutation testing across slice | ch04 tagged | First full ACD workflow cycle |
| 06 | The REST API | scaffold | `chapter-06` | Axum REST handlers, utoipa, Swagger UI, contract tests, CORS, pagination, caching headers, API versioning strategy | ch05 tagged | Completes the first vertical slice |
| 07 | Evidence | scaffold | `chapter-07` | Evidence entity, linking evidence to questions, SPARQL queries | ch06 tagged | |
| 08 | User Authentication | scaffold | `chapter-08` | argon2, tower-sessions, middleware, route protection, secrets lifecycle | ch07 tagged | |
| 09 | Hypotheses | scaffold | `chapter-09` | Hypotheses entity, forming hypotheses from evidence, knowledge graph traversal | ch08 tagged | |
| 10 | Experiments | scaffold | `chapter-10` | Experiment entity, designing experiments to test hypotheses, complex forms | ch09 tagged | |
| 11 | Results & Analysis | scaffold | `chapter-11` | Results entity, bridging Oxigraph and PostgreSQL, criterion benchmarks | ch10 tagged | |
| 12 | Security Hardening | scaffold | `chapter-12` | CSP, rate limiting, SBOM, hardened containers, secrets rotation, DB backup/DR | ch11 tagged | |
| 13 | Feature Flags | scaffold | `chapter-13` | Feature flag infrastructure, deployment/release decoupling | ch12 tagged | |
| 14 | Progressive Rollout | scaffold | `chapter-14` | Canary deployments, observability for canary comparison, load testing | ch13 tagged | |
| A | Appendix A: Toolchain | scaffold | — | — | — | Reference appendix |

### Versioning

Tags follow semver: `v0.x.0` for new chapters or significant revisions, `v0.x.y` for small edits. `v1.0.0` is the complete first edition. Version numbers increment sequentially — they are not tied to chapter numbers. A revision to Ch 02 triggered by Ch 05 work is just the next minor bump, not a Ch 02-specific version. `CHANGELOG.md` records what each version contains. Every tag triggers a GitHub Release with a downloadable PDF.

---

## Title Rationale

"Trunk to Theory" riffs on trunk-based development — the CD practice at the heart of the book — and connects it to the scientific domain. The complete, transparent pipeline from trunk (where all code integrates) to a validated theory (the product of the scientific workflow). It ties the CD philosophy (trunk) to the knowledge management domain (theory) in three words. It's short, memorable, and tells you what the book is about before you open it.

The title is inspired by *Test-Driven Development with Python* (Harry Percival), popularly known as "Obey the Testing Goat" — a book whose catchy identity gave it a personality and a philosophy baked into a single phrase.

The subtitle does the discoverability work, hitting search keywords: full-stack, Rust, agentic, continuous delivery.

---

## 1. Book Concept

### The Pitch

A step-by-step guide to building a full-stack Rust web application from scratch, structured around Agentic Continuous Delivery (ACD) practices from [MinimumCD](https://minimumcd.org). The reader builds Trunk to Theory — a scientific knowledge management platform based on scimantic concepts — as a real, deployable product with Infrastructure as Code on Linode. The app helps researchers pose Questions, gather Evidence, form Hypotheses, design Experiments, and record Results, all managed through a knowledge graph backed by Oxigraph and PostgreSQL.

### What Makes This Book Different

1. **Pipeline-first pedagogy.** Following the MinimumCD greenfield guide, the delivery pipeline is "feature zero." The first thing the reader builds is the CI/CD pipeline and a hello-world deployment to production on Linode. Every subsequent chapter flows features through this pipeline.

2. **Agentic CD framing.** The book is structured around the [ACD workflow](https://beyond.minimumcd.org/docs/agentic-cd/) from MinimumCD — intent descriptions, behavioral specifications as code, agent-assisted specification, test generation, implementation, and pipeline verification. This gives the book a narrative arc about *how* you build, not just *what* you build.

3. **Real product, not a toy example.** Trunk to Theory is simultaneously the book's teaching vehicle and an open-source product. Readers follow along with a codebase that is deployed, maintained, and used by real researchers.

4. **Schema-driven development with dual databases.** The data model is a versioned LinkML schema — an architecture artifact in the ACD sense. panschema generates Rust types, SQL DDL, SHACL shapes, and JSON Schema from that single source. The schema is the contract between every layer of the stack — Oxigraph (knowledge graph) and PostgreSQL (app state) — and the pipeline enforces consistency.

5. **"It's all Rust" full-stack story.** Frontend, backend, API, database queries, knowledge graph, E2E tests, schema tooling, and build tooling — all Rust. The only non-Rust pieces are Tailwind CSS for styling, Terraform HCL for infrastructure, and LinkML YAML for schema definitions.

6. **No existing book covers this combination.** Existing Rust web books (*Rust Web Development* by Gruber, *Rust Servers, Services, and Apps* by Eshwarla) cover server-side only and don't address full-stack with IaC, CI/CD, schema-driven development, knowledge graphs, or the ACD methodology.

### Scope Boundaries

**In scope for the book:**
- Full-stack Rust web application (Leptos + Axum)
- OpenAPI-compliant REST API (for external consumers: CLI tools, Jupyter integrations, etc.)
- Schema-driven development with LinkML and panschema (Rust types, SQL migrations, SHACL shapes, JSON Schema generated from a single source)
- Dual database: Oxigraph (Rust-native RDF triple store) for the knowledge graph + PostgreSQL for app state (users, sessions)
- Infrastructure as Code deployment to Linode (staging + production)
- CI/CD pipeline with GitHub Actions following MinimumCD practices
- E2E testing with playwright-rust
- Agentic CD workflow throughout

**Out of scope for the book (but part of the broader product):**
- Native mobile apps (iOS/Android)
- VS Code extension for knowledge management
- Subscription/payment processing
- Advanced scaling and optimization
- OWL reasoning (no Rust-native OWL reasoner exists)

### Why This Domain

Scientific knowledge management is an ideal teaching vehicle because:

- **Distinctive.** No other Rust book (or web development book) builds a knowledge management platform. The domain is inherently interesting and exercises parts of the stack that toy examples never touch.
- **Deeply exercises the stack.** The scimantic workflow (Questions → Evidence → Hypotheses → Experiments → Results) requires graph queries, ontology modeling, dual databases, complex relationships, and rich UI — far more than simple CRUD.
- **The author is a scientist.** Paul is building tools he actually needs for his own research. This is genuine dogfooding, not a contrived example.
- **Natural vertical slices.** Each scimantic entity maps to a feature slice that can be independently built and deployed (pose a question, link evidence, form a hypothesis, etc.).
- **Ruthlessly scoped MVP.** Questions CRUD + knowledge graph browsing. Nothing more for the first slice.

### Target Audience

Developers who know basic Rust — they've read the first half of *The Rust Programming Language* ("The Book"), they can write a function, they understand ownership conceptually — and want to learn *everything else* through building a real product. Not just Leptos, not just Axum, not just "how to write a REST API in Rust." Everything: async Rust, full-stack web architecture, schema-driven data modeling, database access, knowledge graphs, WASM compilation, E2E testing, Infrastructure as Code, CI/CD pipelines, and the Agentic CD workflow. All of it taught through the act of building and shipping Trunk to Theory.

This is a broader audience than existing Rust web books, which assume the reader already knows web development patterns and focus narrowly on server-side Rust.

### A Debt to the Testing Goat

This book owes a direct debt to Harry Percival's *Test-Driven Development with Python* — known to its readers as "Obey the Testing Goat." That book did something rare: it taught an entire web development stack (Django, HTML, CSS, JavaScript, deployment, CI) to readers who knew Python but nothing else, entirely through the process of building a to-do list application, one tested step at a time. It took Percival six chapters to get a minimal working feature deployed. That wasn't a flaw — it was the point. Every concept was introduced at the moment of need, in context, with a reason to exist. By the end, readers hadn't just built an app; they'd internalized a *way of working*.

*Trunk to Theory* follows the same philosophy for a different era. Where Percival taught TDD-first Python web development, this book teaches ACD-first Rust full-stack development. The reader should understand what they're signing up for: this is not a book where you skip to Chapter 8 and learn how to write a Leptos component. It's a book where you start at Chapter 1, build incrementally, and every chapter depends on the one before it. The first vertical slice — "a researcher can pose a question in their knowledge base" — takes roughly six chapters. That's not slow. That's thorough. By the end of those six chapters, the reader will have touched every layer of the stack and completed a full cycle of the ACD workflow. Every subsequent feature builds faster because the scaffolding — both in the codebase and in the reader's understanding — is already in place.

Like the Testing Goat, this book is a cover-to-cover journey. Come with patience, a terminal, and basic Rust. Leave with a deployed product and a way of working you'll use for the rest of your career.

---

## 2. Teaching Philosophy: At the Moment of Need

### Core Principle

Everything is taught inline, in context, at the moment the reader needs it. There are no dedicated "Rust Fundamentals," "Toolchain Overview," or "Testing Basics" chapters. The moment you add one of those, you've blown the page budget and broken the narrative flow. The ACD workflow is the spine of the book. Rust idioms, toolchain knowledge, and playwright-rust are taught as they arise naturally within that workflow.

This approach is more than a structural choice — it's philosophically aligned with the ACD methodology itself. The MinimumCD ACD anti-patterns catalog explicitly identifies rubber-stamping AI-generated code without understanding as a dysfunction. The reader must understand what they're implementing. But understanding comes from encountering a real problem and learning the concept that solves it, not from reading about the concept in the abstract and hoping to remember it later.

**The rule of thumb:** Explain enough that the reader can *modify* the code confidently, not enough that they could reimplement the tool from scratch. The book's job is to get them building and shipping — and to make sure they understand what they're shipping. Readers who want to go deeper on any topic will have the foundation to do so with dedicated resources.

### Idiomatic Rust — Taught Inline as Patterns Arise

Rust's language features are introduced at the exact moment the reader encounters a real problem they solve:

| When the reader... | They learn about... |
|---|---|
| Writes their first Leptos component with signals | Ownership, `move` closures, why `Copy` + `'static` signals matter |
| Writes their first fallible service layer function | `Result<T, E>`, the `?` operator, error propagation |
| Defines the service layer trait consumed by both Leptos and REST handlers | Traits, trait objects vs. generics, dependency injection in Rust |
| Models question states (open, investigating, resolved) | Enums, pattern matching, exhaustiveness checking |
| Sees Rust types generated by panschema from the LinkML schema | Derive macros, code generation, why generated code is idiomatic Rust |
| Converts between domain models and API response types | `From`/`Into` traits, the newtype pattern |
| Writes their first SQLx query with `query_as!` | Procedural macros, compile-time verification, what happens at build time vs. runtime |
| Writes their first SPARQL query against Oxigraph | Graph query patterns, RDF triples, how knowledge graphs differ from relational databases |
| Encounters a lifetime annotation for the first time | Lifetimes (briefly), why Rust tracks them, and when to reach for owned types instead |
| Shares database pool state across handlers | `Arc`, `Clone`, interior mutability patterns |
| Writes concurrent E2E tests | `async`/`await`, the Tokio runtime, `Future` basics |

Each of these gets a concise explanation — a well-placed sidebar, callout box, or "Why does Rust make me do this?" aside — right where the reader encounters the pattern. No forward references to "we'll cover this in Chapter 12." No abstract explanations divorced from working code.

### The Toolchain — Taught at the Moment of Use

Every tool in the Rust development workflow is introduced when the reader first uses it, with enough context to understand what it does and why it exists:

| Tool | When introduced | What the reader learns |
|---|---|---|
| `rustup`, `cargo` | Project setup | Rust's toolchain manager and build system, the role of `Cargo.toml` |
| `rustfmt` | Pipeline configuration (Chapter 2) | Automatic code formatting, why the pipeline enforces it |
| `clippy` | Pipeline configuration (Chapter 2) | Rust's linter, common idiom violations, why the pipeline enforces it |
| `cargo-leptos` | First Leptos project scaffold | Dual-target compilation (server binary + WASM client), hot reloading |
| `cargo-nextest` | First unit test | Faster test runner, parallel execution, structured output |
| `cargo-audit` | Pipeline security scanning | Dependency vulnerability detection, why this runs in CI |
| `cargo-vet` | Pipeline configuration (Chapter 2) | Supply chain vetting: importing trusted audits, verifying dependency provenance, exempting new crates |
| `prek` | Pipeline configuration (Chapter 2) | Rust-native pre-commit hooks. Local hooks mirror CI checks for fast feedback. Reads `.pre-commit-config.yaml` (industry standard) |
| `cargo-mutants` | First unit test (Chapter 3) | Mutation testing: proving tests catch real bugs, not just exercise code. Incremental (`--in-diff`) on every push; full sweep on a schedule |
| `tracing` | Health-check endpoint (Chapter 2) | Structured logging and observability: spans, events, log levels, JSON output for production. The runtime complement to compile-time verification |
| `thiserror` | First fallible service function (Chapter 3) | Domain-specific error types with context. Production error handling beyond `Result<T, E>` syntax |
| `criterion` | REST API performance (Chapter 6+) | Benchmarking: response time budgets, identifying bottlenecks, preventing performance regressions in CI |
| `wasm-bindgen` | First client-side interactivity | What WASM is, how Rust compiles to it, the browser-Rust boundary |
| `panschema` | Data model definition (Chapter 2) | Schema-driven development: LinkML YAML as the single source of truth, generating Rust types, SQL DDL, SHACL shapes, and JSON Schema |
| `sqlx-cli` | First database migration | Migration management, offline mode for compile-time query checking |
| `oxigraph` | Knowledge graph setup (Chapter 3) | Rust-native RDF triple store, SPARQL 1.1 queries, how graph databases complement relational databases |
| `sophia_rs` | RDF data handling | RDF toolkit for Rust: parsing, serialization, nanopublications, RDFS reasoning |
| `wasm-pack` / `trunk` | Build process explanation | How the WASM build pipeline works under `cargo-leptos` |

The reader is never asked to install and learn a tool before they have a reason to use it.

### playwright-rust — A Dedicated Section, Not a Dedicated Chapter

playwright-rust gets more real estate than other individual tools because (a) the author is the maintainer, (b) it's a newer part of the Rust ecosystem that readers won't already know, and (c) it's the capstone of the "it's all Rust" testing story. However, it's taught within the context of the E2E testing chapter, not as a standalone.

**What the reader learns about playwright-rust:**

- **Architecture:** Rust API → JSON-RPC over stdio → Playwright server (Node.js) → browser native protocols → Chromium/Firefox/WebKit. Why this architecture gives full feature parity with Playwright's JS/Python/Java implementations.
- **Practical E2E tests:** Writing meaningful tests against Trunk to Theory — pose a research question, link evidence, verify persistence across page reload, test the OpenAPI endpoints. Not toy examples.
- **Pipeline integration:** How playwright-rust tests run as a stage in the GitHub Actions pipeline, including browser installation in CI and handling headless vs. headed execution.
- **The "it's all Rust" payoff:** This is the moment the reader sees the full picture — a Rust web app, tested by a Rust E2E framework, verified by a Rust-based pipeline, deployed by IaC. The testing tool is written in the same language as the application.

### Testing Philosophy: Code-First Behavioral Specs

Behavioral specifications are Rust test functions. Doc tests are executable specifications. Descriptive test function names communicate behavior. The compiler catches categories of bugs that other stacks need runtime testing to find. BDD is the practice; Rust's native testing tools are the implementation.

The testing pyramid for Trunk to Theory:

| Layer | Tool | What it tests | ACD Artifact |
|---|---|---|---|
| Doc tests | `cargo test --doc` | API examples compile and run; docs can't drift from code | Living documentation |
| Unit tests | cargo-nextest | Service layer, validation, schema-generated types | Acceptance criteria |
| Component tests | dokime | Leptos rendering, signal reactivity, event handling (no browser) | User-facing behavior |
| Integration tests | cargo-nextest + SQLx fixtures | Service layer against real PostgreSQL | Acceptance criteria |
| Graph tests | cargo-nextest + Oxigraph test store | SPARQL queries, SHACL validation, knowledge graph integrity | Acceptance criteria |
| Contract tests | cargo-nextest | REST API conforms to OpenAPI spec | Acceptance criteria |
| Security E2E (DAST) | playwright-rust | Injection, XSS, auth bypass against running app | System constraints |
| E2E tests | playwright-rust | Full user flows in a real browser | User-facing behavior |
| Visual regression | theoria + playwright-rust | Component screenshots match baseline | User-facing behavior |
| Mutation testing | cargo-mutants | Tests catch real bugs, not just exercise code. Incremental (`--in-diff`) on every push; full sweep on schedule | Test quality |

Each layer maps to an ACD artifact. The pipeline runs all layers on every commit. No separate specification language is needed because the specifications *are* the tests.

### What This Book Does *Not* Teach

Being honest about prerequisites while being generous about what's taught in context:

- **Not a Rust-from-zero book.** The reader should have read at least the first half of *The Rust Programming Language* ("The Book") — chapters 1 through 10. They should be comfortable with variables, functions, structs, enums, basic pattern matching, and the conceptual idea of ownership. Everything beyond that — async, traits in practice, closures, error handling patterns, lifetimes, macros — is taught through building Trunk to Theory.
- **Not a toolchain reference.** Each tool gets enough explanation to use it effectively in this project. Readers who want exhaustive `clippy` lint configuration or advanced `cargo` workspace patterns are directed to official documentation.
- **Not a complete Playwright tutorial.** The reader learns playwright-rust in the context of testing Trunk to Theory. They don't learn every Playwright API — they learn the subset needed for effective E2E testing of a full-stack web app.
- **Not a Terraform deep-dive.** The reader learns enough Terraform to provision staging and production environments on Linode. They don't learn Terraform module design patterns or multi-cloud strategies.
- **Not a semantic web textbook.** The reader learns enough about RDF, SPARQL, and SHACL to build a working knowledge graph with Oxigraph. They don't learn OWL reasoning, federated SPARQL, or the full semantic web stack.
- **Not a book you can skip around in.** Like *Obey the Testing Goat*, each chapter builds on the previous one. The codebase grows incrementally. The reader who jumps to Chapter 10 will be lost. This is a cover-to-cover book.

### MinimumCD as a Local Reference

The book does not restate MinimumCD or ACD content. Instead, readers clone a pinned commit of the [MinimumCD Practice Guide](https://github.com/bdfinst/cd-migration) and run it locally with Hugo. The book directs readers to specific pages at key moments ("Read first: open localhost:1313/docs/agentic-cd/specification/first-class-artifacts/ and read the Intent Description section"), then brings them back to apply what they read to Trunk to Theory. This approach:

- **Avoids close paraphrasing.** The book references the source directly rather than restating it.
- **Gives editorial control.** Pinning to a specific commit means the reader sees exactly what the book expects, even if the live site changes.
- **Keeps the book focused.** CD theory lives in the Practice Guide; the book focuses on Rust implementation.
- **Creates a reading rhythm.** Theory (Practice Guide) then practice (this book), alternating throughout.

The "Before You Begin" chapter walks readers through cloning and running the local site.

**For authoring:** The book is written with the help of `minimumcd-mcp`, a Rust-based MCP server that indexes the Practice Guide content and provides it as context to the AI coding assistant. This tool is mentioned in Appendix A and available on crates.io, but is not a reader prerequisite.

---

## 3. The Trunk to Theory Product

### Domain

A scientific knowledge management platform based on scimantic concepts. The domain is ideal for a teaching vehicle because:

- **Distinctive.** No other programming book builds a scientific knowledge management platform. The domain is inherently interesting and sets the book apart from the sea of to-do lists, blogs, and e-commerce tutorials.
- **Deeply exercises the stack.** The scimantic workflow (Questions → Evidence → Hypotheses → Experiments → Results) requires graph queries, ontology modeling, dual databases, complex entity relationships, and rich interactive UI — far more than simple CRUD.
- **The author is a scientist.** This is genuine dogfooding. Paul is building research tools he actually needs, not a contrived teaching example.
- **Natural vertical slices.** Each scimantic entity maps to a feature slice that can be independently built and deployed: pose a question, link evidence, form a hypothesis, design an experiment, record results.
- **Ruthlessly scoped MVP.** Questions CRUD + knowledge graph browsing for the first vertical slice. Nothing more for the book's opening chapters.

### Architecture: Dual-Interface, Dual-Database Design

Two critical design decisions shape the architecture:

**1. Dual database.** Scientific knowledge management requires two complementary data stores:
- **Oxigraph** (Rust-native RDF triple store) for the knowledge graph: questions, evidence, hypotheses, experiments, results, and the relationships between them. RDF/SPARQL is the right model for knowledge graphs — provenance chains, ontology relationships, and flexible schema evolution.
- **PostgreSQL** for app infrastructure: users, sessions, access control, audit logs. Relational data that doesn't benefit from graph modeling.

panschema generates outputs for both: SHACL shapes for Oxigraph validation, SQL DDL for PostgreSQL migrations.

**2. Dual interface.** Trunk to Theory needs to serve both a web frontend (for desktop/mobile browsers) and external consumers (CLI tools, Jupyter notebook integrations, research automation scripts) via a REST API.

Leptos server functions are designed for Leptos-to-Leptos communication and are not standard REST endpoints. External consumers need an OpenAPI-compliant REST API. However, Leptos runs *on top of* Axum — they share the same server process, Tokio runtime, and router. This enables a single-binary, dual-interface architecture:

```
┌──────────────────────────────────────────────────────┐
│                 Single Axum Server                    │
│                                                      │
│  ┌────────────────────┐  ┌────────────────────────┐  │
│  │  Leptos Routes     │  │  REST API Routes       │  │
│  │  (SSR + WASM)      │  │  (utoipa OpenAPI)      │  │
│  │  server functions  │  │  /api/v1/*             │  │
│  └─────────┬──────────┘  └──────────┬─────────────┘  │
│            │                        │                │
│            ▼                        ▼                │
│  ┌────────────────────────────────────────────────┐  │
│  │           Shared Service Layer                 │  │
│  │     (domain logic, validation, auth)           │  │
│  └──────────┬──────────────────┬─────────────────┘  │
│             │                  │                     │
│             ▼                  ▼                     │
│  ┌──────────────────┐  ┌──────────────────────┐     │
│  │  SQLx +           │  │  Oxigraph            │     │
│  │  PostgreSQL       │  │  (RDF/SPARQL)        │     │
│  │  (app state)      │  │  (knowledge graph)   │     │
│  └──────────────────┘  └──────────────────────┘     │
└──────────────────────────────────────────────────────┘
```

**Why this matters:**

- The shared service layer enforces clean separation between domain logic and presentation.
- Both interfaces are guaranteed by Rust's type system to agree on data shapes.
- Domain logic is tested once; both interfaces consume it.
- The REST API gets OpenAPI documentation generated at compile time from the same Rust types the Leptos frontend uses.
- The service layer abstracts the dual database — callers don't need to know whether a query hits PostgreSQL or Oxigraph.

### Narrative Arc for the Book

The dual-interface, dual-database design creates a natural teaching progression:

1. Start with PostgreSQL for app infrastructure and Oxigraph for the knowledge graph — introduced together in Chapter 3.
2. Build Leptos server functions to get the web app working fast (simplest path to a vertical slice).
3. In a later chapter, extract the service layer and add the REST API with utoipa.
4. Readers learn *why* the separation matters by experiencing the need for it organically.
5. Later chapters bridge the two databases as the scientific workflow demands cross-cutting queries.

---

## 4. Technology Stack

### Core Framework: Leptos 0.8 + Axum

**Leptos** is the most actively developed full-stack Rust web framework. Key features for this book:

- **Full-stack isomorphic:** Same code runs on server (SSR) and client (WASM with hydration).
- **`#[server]` functions:** Write database queries and UI components in the same file. The framework handles serialization across the client/server boundary.
- **Fine-grained reactivity:** No virtual DOM. Signal changes update individual DOM nodes.
- **Runs on Axum:** Not a separate server — Leptos integrates directly with Axum's router and middleware.
- **RSX templating:** HTML-like syntax in Rust macros. The compiler catches template errors at compile time.
- **`cargo-leptos` build tool:** Handles compiling both server binary and WASM client, with hot reloading.

**Axum** is the de facto standard Rust web backend framework, developed by the Tokio team:

- Native async with Tokio runtime.
- Type-safe extractors (State, Json, Path, Query).
- Middleware via Tower.
- Serves as the foundation for both Leptos routes and REST API routes.

### Databases: PostgreSQL + Oxigraph

**PostgreSQL** for app infrastructure:
- **SQLx:** Async, compile-time checked SQL queries. No ORM layer — raw SQL is more teachable and the abstraction isn't needed for the MVP scope.
- **PostgreSQL:** Production-grade relational database for users, sessions, access control, audit logs. Chosen over SQLite because the MinimumCD greenfield guide mandates production-like environments from day one. Using Linode's Managed PostgreSQL service via Terraform.
- **Migrations:** SQLx's built-in migration system.

**Oxigraph** for the knowledge graph:
- **Oxigraph:** Rust-native RDF triple store with full SPARQL 1.1 support. Embeddable (runs in-process, no external server needed) with RocksDB storage for persistence. Supports Turtle, N-Triples, and JSON-LD serialization formats.
- **sophia_rs:** Rust RDF toolkit for parsing, serialization, nanopublications, and RDFS reasoning. Complements Oxigraph for data manipulation.
- **SHACL validation:** panschema generates SHACL shapes from the LinkML schema. These validate data entering the knowledge graph, enforcing ontology constraints at the application level.

**Why dual databases:** RDF/SPARQL is the right model for knowledge graphs — provenance chains, ontology relationships, and flexible schema evolution. PostgreSQL is the right model for app infrastructure. Using both lets each database do what it does best, and panschema generates the appropriate artifacts for each.

### Schema-Driven Development: panschema + LinkML

- **LinkML:** A YAML-based modeling language for defining data structures. The Trunk to Theory domain model is defined in the `scimantic-ontology` repo as a versioned LinkML schema. In ACD terms, this is architecture represented as a delivery artifact — versioned, machine-readable, and enforced by the pipeline.
- **panschema:** The author's own Rust CLI tool. The universal Rust data modeling tool, handling vocabularies, application data models, and ontologies. Reads LinkML schemas and generates:
  - **Rust structs** with `serde::Serialize`, `serde::Deserialize`, `sqlx::FromRow`, and `utoipa::ToSchema` derives.
  - **SQL DDL** for SQLx migrations (PostgreSQL app tables).
  - **SHACL shapes** for Oxigraph validation (knowledge graph constraints).
  - **JSON Schema** for API contract validation.
  - **Visualizations** for documentation and ontology exploration.
- **Ontology as a separate repo:** The `padamson/scimantic-ontology` repo contains the LinkML schema. The t2t app depends on it as a versioned artifact. This separates the ontology (which may be used by multiple tools) from the application.
- **Why this matters for ACD:** The ACD framework requires consistency between intent, tests, implementation, and architecture. The LinkML schema *is* the architectural intent for the data model. panschema enforces consistency by generating the implementation artifacts from it. When the schema changes, the pipeline regenerates types, migrations, SHACL shapes, and any inconsistency breaks the build.

### OpenAPI: utoipa + utoipa-axum + utoipa-swagger-ui

- **utoipa:** Code-first, compile-time generated OpenAPI documentation from proc macro annotations.
- **utoipa-axum:** `OpenApiRouter` extends Axum's router, simultaneously registering handlers and generating the OpenAPI spec.
- **utoipa-swagger-ui:** Serves Swagger UI from the same binary for API exploration and documentation.

This provides the OpenAPI-compliant REST API that external consumers (CLI tools, Jupyter integrations, research automation scripts) can consume. The OpenAPI spec is generated from the same Rust types used by the Leptos frontend.

### Styling and UI: Tailwind CSS v4 + Component-Driven Development

- **Tailwind CSS v4:** The Leptos ecosystem has standardized around Tailwind. The v4 standalone CLI is written in Rust (Lightning CSS), requiring no Node.js runtime. Utility-first CSS with a Trunk to Theory-specific configuration (color palette, typography scale, spacing tokens). No third-party CSS framework (DaisyUI or similar) — component styles are Leptos components composing Tailwind utilities directly.
- **Component-driven development:** The UI is built from composable Leptos components (Button, Input, Card, Layout, etc.) introduced in Chapter 4 and reused in every subsequent chapter. Each component encapsulates its Tailwind utility classes, accepts typed props, and renders consistently everywhere. New features compose from existing components.
- **theoria:** The author's own Rust-native component catalog for Leptos (dogfooded in this book). Provides a dedicated route for browsing and testing UI components in isolation with configurable props. Essential for a pure-Tailwind approach where components are built from scratch rather than consumed from a CSS framework. Introduced in Chapter 4 alongside the first Leptos components.
- **dokime:** The author's own Rust-native component testing framework for Leptos (dogfooded in this book). Verifies rendering, signal reactivity, and event handling for every component theoria catalogs, without a full browser. Introduced alongside theoria in Chapter 4.
- **Trunk to Theory theme:** Color palette, typography, and spacing defined in Tailwind's configuration. Component styles composed from these tokens inside Leptos components. The theme is intentionally minimal but well-structured, showing the reader how to build and expand a design system from Tailwind primitives.

### Authentication

For MVP chapters: session-based auth with `argon2` (password hashing) and `tower-sessions` (session management). OAuth can be layered on later for the broader product but is out of scope for the book.

### Infrastructure as Code: Terraform + Linode (Akamai Cloud)

- **Terraform:** Linode Terraform Provider v3.0.0 (released June 2025). Mature, well-documented, simpler than Pulumi for a book context. HCL is teachable in a single chapter.
- **Linode/Akamai Cloud:** Simpler and cheaper than AWS/GCP for a book's purposes. Readers can follow along without surprise cloud bills. Resources provisioned via Terraform:
  - Linode compute instance(s)
  - Managed PostgreSQL database
  - NodeBalancer (load balancer)
  - Staging and production environments

### CI/CD: GitHub Actions

Maps cleanly to the ACD workflow:
- Trunk-based development with automated tests on every push.
- Single path to production — the pipeline is the only way to deploy.
- Pipeline decides releasability; its verdict is definitive.
- All feature work stops when the main pipeline is red.

### E2E Testing: playwright-rust

The author's own crate. Rust language bindings for Microsoft Playwright. Provides cross-browser E2E testing (Chromium, Firefox, WebKit) from Rust. This is a genuine demonstration that the Rust web ecosystem is maturing — you can test a Rust web app with a Rust testing tool.

Using playwright-rust for E2E tests as a stage in the CI/CD pipeline is also a compelling bit of dogfooding that ties the narrative together.

### Containers: Podman + Devcontainer

- **Podman** over Docker Desktop: Fully open source, daemonless, rootless by default (better security posture), and `podman compose` is a drop-in replacement for `docker compose`. No licensing concerns for readers at companies over 250 employees. Docker remains compatible for readers who prefer it.
- **`compose.yaml` with PostgreSQL + Oxigraph:** The local development databases run in containers, matching the same PostgreSQL version as Linode's Managed PostgreSQL in staging and production. Oxigraph runs as an embedded store in development (in-process, no container needed) but the compose file includes it for integration testing. This eliminates environment divergence from day one, consistent with the MinimumCD principle of production-like environments.
- **VS Code Devcontainer (`.devcontainer/devcontainer.json`):** The recommended setup path for readers. Open the repo in VS Code, click "Reopen in Container," and get a fully configured environment: Rust toolchain, cargo-leptos, sqlx-cli, panschema, playwright-rust dependencies, Oxigraph, and PostgreSQL as a service. No "install these 12 things before Chapter 2" section. Readers who prefer manual setup can still follow along, but the devcontainer is the happy path.
- **Podman Desktop:** Provides the GUI experience for container management. VS Code's container tooling works with both Podman and Docker.

### Build Tooling: cargo-leptos

Handles:
- Compiling server binary and WASM client from the same codebase.
- Hot reloading during development.
- CSS minification and WASM optimization.
- Coordinating the dual-target build (server + client).

### Stack Summary Table

| Layer | Technology | Role |
|---|---|---|
| Full-stack web framework | Leptos 0.8 + Axum | SSR, hydration, server functions, routing |
| REST API | Axum handlers + utoipa | OpenAPI-compliant API for external consumers (CLI, Jupyter, etc.) |
| Schema modeling | panschema + LinkML | Single-source data model → Rust types, SQL DDL, SHACL shapes, JSON Schema, visualizations |
| Knowledge graph | Oxigraph + sophia_rs | Rust-native RDF triple store, SPARQL 1.1 queries, knowledge graph persistence |
| App database | SQLx + PostgreSQL | Async, compile-time verified queries for app state (users, sessions) |
| Styling + UI | Tailwind CSS v4 | Utility-first CSS, Rust-native standalone CLI (no Node.js), Trunk to Theory theme |
| Component explorer | theoria | Rust-native component catalog (author's project, dogfooded here) |
| Component testing | dokime | Rust-native component testing framework for Leptos (author's project, dogfooded here) |
| Authentication | argon2 + tower-sessions | Password hashing + session management |
| Containers | Podman + compose.yaml | Local dev databases, devcontainer, production images |
| Dev environment | VS Code Devcontainer | One-click setup with full toolchain + PostgreSQL + Oxigraph |
| IaC | Terraform + Linode Provider v3 | Infrastructure provisioning (staging + prod) |
| CI/CD | GitHub Actions | Pipeline automation, trunk-based workflow |
| E2E Testing | playwright-rust | Cross-browser end-to-end tests (functional + DAST) |
| Build tool | cargo-leptos | Dual-target compilation, hot reload |
| API documentation | utoipa-swagger-ui | Swagger UI served from the application binary |
| Dependency security | cargo-audit + Dependabot | CVE scanning + automated dependency update PRs |
| Supply chain policy | cargo-deny | License compliance, crate source vetting, duplicate detection |
| Supply chain vetting | cargo-vet | Trusted audit imports (Mozilla, Google, ISRG), dependency provenance verification |
| Mutation testing | cargo-mutants | Test quality verification: incremental (`--in-diff`) on every push, full sweep on schedule |
| Observability | tracing + tracing-subscriber | Structured logging, spans, events, JSON output for production log aggregation |
| Error handling | thiserror | Domain-specific error types with context, error chains, clean propagation across service boundaries |
| Benchmarking | criterion | Response time budgets, performance regression detection in CI |
| SAST | GitHub Advanced Security | Static analysis, secret scanning, dependency review |
| Pre-commit hooks | prek | Rust-native hook runner, mirrors CI checks locally for fast feedback, reads `.pre-commit-config.yaml` |
| Local security review | Claude Code `/security-review` skill | AI-assisted code review for injection, auth bypass, XSS (human-in-the-loop) |

---

## 5. MinimumCD and Agentic CD Alignment

### Core MinimumCD Principles Governing the Book

From the [MinimumCD manifesto](https://minimumcd.org):

**Continuous Delivery minimum activities:**
- Use Continuous Integration.
- The application pipeline is the only way to deploy to any environment.
- The pipeline decides the releasability of changes; its verdict is definitive.
- Artifacts created by the pipeline always meet the organization's definition of deployable.
- Immutable artifacts (no human changes after commit).
- All feature work stops when the main pipeline is red.
- Production-like test environment.
- Rollback on-demand.
- Application configuration deploys with artifact.

**Continuous Integration minimum activities:**
- Trunk-based development.
- Work integrates to the trunk at a minimum daily.
- Work has automated testing before merge to trunk.
- Work is tested with other work automatically on merge.
- All feature work stops when the main build is red.
- New work does not break delivered work.

**Trunk-Based Development minimum activities:**
- All changes integrate into the trunk.
- If branches are used, they originate from trunk, re-integrate to trunk, and are short-lived.

### Greenfield CD: The Book's Structural Foundation

The [MinimumCD Greenfield CD guide](https://beyond.minimumcd.org/docs/migrate-to-cd/greenfield/) provides the structural foundation for the book:

**"Pipeline first"** — Before writing application code, set up the delivery pipeline. The pipeline is feature zero. The first commit includes a build script, CI configuration, and deployment mechanism.

**"Deploy 'hello world' to production"** — The first deployment happens before the first feature. This proves the entire path works: build, test, package, deploy, verify. The greenfield guide is explicit: "The goal is to prove the full path works end-to-end."

**"Feature zero validations"** — Code formatting (rustfmt), linting (clippy), type checking (Rust compiler — already stricter than most linting toolchains), test framework (cargo-nextest), security scanning (cargo-audit). "Every one of these is trivial to add to an empty project and expensive to retrofit into a mature codebase."

**"Trunk-based development from the start"** — There is no reason to start with long-lived branches.

**"Test architecture from the start"** — Unit tests, integration tests, component tests, contract tests, all established before features are built.

**"Small, vertical slices from the start"** — Decompose features into independently deployable increments.

### Agentic CD Extensions

The [ACD framework](https://beyond.minimumcd.org/docs/agentic-cd/) extends MinimumCD for AI agent-assisted development:

- Explicit, human-owned intent exists for every change.
- Intent and architecture are represented as delivery artifacts.
- All delivery artifacts are versioned and delivered together with the change.
- Intended behavior is represented independently of implementation (behavioral test functions).
- Consistency between intent, tests, implementation, and architecture is enforced.
- Agent-generated changes must comply with all documented constraints.
- While the pipeline is red, agents may only generate changes restoring pipeline health.

**Schema-driven development as ACD in practice:** The LinkML schema is an architecture artifact in the ACD sense — it represents the data model intent independently of implementation. panschema enforces consistency by generating Rust types, SQL migrations, SHACL shapes, and JSON Schema from that single source. When an agent or human modifies the schema, the pipeline regenerates all downstream artifacts and verifies consistency. This is the ACD principle of "consistency between intent, tests, implementation, and architecture is enforced" made concrete for data modeling. The scimantic ontology, maintained in its own repo as a versioned artifact, is a particularly strong example: it is the scientific domain's architecture, consumed by the application as a dependency.

**ACD Workflow (maps to chapter progression):**

| Stage | Human | Agent | Pipeline |
|---|---|---|---|
| Intent Description | Draft problem statement | Find ambiguity, suggest edge cases | |
| User-Facing Behavior | Define and approve behavioral test specs | Generate test drafts, find gaps | |
| Feature Description | Set constraints and boundaries | Suggest architectural considerations | |
| Acceptance Criteria | Define thresholds | Draft non-functional criteria | |
| Test Generation | | Generate test code from behavioral specs | |
| Implementation | | Generate production code | |
| Pipeline Verification | | | Run all tests; all scenarios must pass |
| Deployment | | | Deploy through same pipeline |

### Key Anti-Patterns the Book Explicitly Avoids

From the MinimumCD anti-patterns catalog:

- **"We'll set up the pipeline later"** — Pipeline feels like overhead when there's little code. The book addresses this head-on: the pipeline shapes the code, not the other way around.
- **"Deferring CD Until After the Rewrite"** — CD is built in from day one.
- **"Testing only at the end"** — Test architecture is established before the first feature.
- **"Long-lived feature branches"** — Trunk-based development from commit one.
- **"No Infrastructure as Code"** — Terraform configs are in the repo from the first commit.
- **"Snowflake environments"** — Staging and production are provisioned from the same Terraform configs.

---

## 6. Chapter Progression: The First Vertical Slice

Like *Obey the Testing Goat*, the first complete feature — "a researcher can pose a question in their knowledge base" — takes several chapters. This is not a flaw. Each chapter teaches a bounded set of new concepts through a concrete step in building the app. By the end of Chapter 6, the reader has touched every layer of the stack and completed a full cycle of the ACD workflow. Every subsequent feature builds faster because the scaffolding — both in the codebase and in the reader's understanding — is in place.

### Chapter 1: Design Decisions

The introductory chapter walks through all design decisions following MinimumCD greenfield guidance. No code yet — just the "why" behind everything that follows. Three layers of decisions:

**6.1.1 Why Pipeline-First, and What Does Ours Look Like?**

Opening argument: Starting with CD is dramatically easier than migrating to it. The pipeline is feature zero.

- Introduce MinimumCD principles (CI, CD, TBD) as the constraints governing the entire book.
- Explain that Chapter 2 will *not* start with `cargo leptos new`. It will start with a GitHub Actions workflow, Terraform configs for Linode, and deploying a health-check endpoint to production.
- Walk through "feature zero validations" mapped to the Rust toolchain:
  - **Formatting:** `rustfmt` (enforced by pipeline)
  - **Linting:** `clippy` (enforced by pipeline)
  - **Type checking:** Rust's compiler (already stricter than most languages' entire linting toolchains — a selling point)
  - **Test framework:** `cargo-nextest` (configured from commit one)
  - **Security scanning:** `cargo-audit` (dependency vulnerability scanning)
  - **Supply chain policy:** `cargo-deny` (license compliance, source vetting, duplicate detection)
  - **Secrets management:** No secrets in code; environment variables, GitHub Secrets, `.env` in `.gitignore`
  - **Automated dependency updates:** Dependabot configured from day one
  - **GitHub security features:** Code scanning (SAST), secret scanning, dependency review
- Introduce shift-left security as a two-layer model:
  - **Local (human-in-the-loop):** AI-assisted security review via Claude Code skills and pre-commit hooks. Catches application-logic vulnerabilities (injection, auth bypass, XSS) that static tools miss.
  - **Pipeline (deterministic, auditable):** cargo-audit, cargo-deny, Dependabot, GitHub Advanced Security. No API dependency; runs the same every time.
- Introduce the ACD extensions: intent descriptions, behavioral specifications as code, agent delivery contracts.

**6.1.2 Why Schema-Driven? The Data Model as an Architecture Artifact**

- The ACD framework requires architecture to be represented as versioned delivery artifacts. For data modeling, this means the schema should be the source of truth — not the Rust code, not the SQL, not the OpenAPI spec, not the SPARQL queries.
- Introduce LinkML as the schema language: YAML-based, human-readable, tool-friendly. Define classes, attributes, enums, and relationships in one place.
- Introduce the scimantic ontology as a separate versioned artifact (`padamson/scimantic-ontology`). The application depends on it — this is ACD's "architecture as delivery artifact" principle in action.
- Introduce panschema as the tool that enforces consistency: from a single LinkML schema, generate Rust structs (with serde, sqlx, and utoipa derives), SQL DDL for migrations, SHACL shapes for knowledge graph validation, and JSON Schema for contract validation.
- Contrast with the typical approach: hand-maintaining separate struct layers (API DTOs, domain models, DB row types, graph shapes) connected by manual implementations. That works, but consistency depends on developer discipline. Schema-driven development makes the pipeline enforce it.
- Frame this as a natural extension of the same philosophy behind compile-time query checking (SQLx) and compile-time OpenAPI generation (utoipa) — the theme is: catch inconsistencies at build time, not in production.

**6.1.3 Why This Architecture? The Dual-Interface, Dual-Database Design**

- Start from Trunk to Theory's requirements: web app for browsers + REST API for external consumers + knowledge graph for scientific data + relational database for app state.
- Explain why Leptos server functions alone are insufficient (not standard REST endpoints).
- Explain why a single database is insufficient: relational databases are poor at graph traversal; graph databases are poor at transactional app state.
- Walk through the architecture diagram: Leptos routes + REST API routes → shared service layer → Oxigraph (knowledge graph) + PostgreSQL (app state).
- Explain why the shared service layer matters: domain logic tested once, type system guarantees data shape agreement, database choice abstracted from callers.
- Map each technology choice to CD constraints:
  - **PostgreSQL over SQLite:** Production-like environments from day one.
  - **Oxigraph:** Rust-native, embeddable, no external service dependency for the knowledge graph.
  - **SQLx:** Compile-time query verification means the pipeline catches database contract violations before deployment.
  - **Terraform + Linode:** Everything-as-code; infrastructure lives in the same repo and flows through the same pipeline.
  - **utoipa:** OpenAPI spec generated at compile time from the same Rust types — the API contract is enforced by the compiler.
  - **playwright-rust:** E2E testing in Rust, demonstrating ecosystem maturity.

**6.1.4 Why Rust, and Why Now?**

- The Rust web ecosystem has crossed a threshold:
  - Leptos provides the full-stack story (SSR, hydration, server functions).
  - Axum is the de facto standard backend.
  - The testing ecosystem now includes E2E browser automation (playwright-rust).
  - utoipa provides code-first OpenAPI generation.
  - SQLx provides compile-time database query checking.
  - Oxigraph provides a Rust-native knowledge graph with full SPARQL 1.1 support.
- Rust's type system and compiler do at build time what other stacks need runtime testing and linting to catch — the pipeline's deterministic test suite is *stronger* with less effort.
- AI coding assistants are making Rust accessible to more developers. TDD is experiencing a renaissance as the optimal way to work with AI agents. These trends are converging.

**6.1.5 Chapter Close: Greenfield Checklist**

Adapted from the MinimumCD greenfield checklist to the specific stack. Serves as both a preview of what the reader will build and a de facto table of contents for the book.

Pipeline Basics:
- [ ] GitHub Actions CI pipeline runs on every push to trunk
- [ ] `cargo leptos build` compiles, tests, and packages with a single command
- [ ] First unit test exists and passes
- [ ] All work integrates to trunk at least daily
- [ ] Deployment to staging is automated via Terraform + GitHub Actions
- [ ] Structured logging with `tracing` from the first handler

Quality Gates:
- [ ] Test architecture established (unit with cargo-nextest, integration with SQLx test fixtures, graph tests with Oxigraph test store, E2E with playwright-rust)
- [ ] Mutation testing verifies test suite catches regressions (cargo-mutants `--in-diff` on every push; full sweep on schedule)
- [ ] External dependencies use test doubles in the deterministic test suite
- [ ] Contract tests exist for the OpenAPI REST API
- [ ] SHACL validation tests verify knowledge graph constraints
- [ ] Domain-specific error types with context (thiserror)
- [ ] Accessibility: WCAG 2.1 AA compliance, semantic HTML, ARIA, keyboard navigation
- [ ] API endpoints are paginated with cursor-based pagination
- [ ] HTTP caching headers on read endpoints
- [ ] Pipeline deploys to a production-like staging environment on Linode
- [ ] Rollback is tested and works
- [ ] Application configuration is externalized (environment variables, not baked into the binary)
- [ ] Artifacts are immutable (single binary built once, deployed to staging and production)

Security:
- [ ] Secrets rotation is automated for database and API credentials
- [ ] Database backups are automated via Terraform
- [ ] Database restore process is tested against real data

Production Readiness:
- [ ] Pipeline deploys to production on Linode
- [ ] Every commit that passes the pipeline is a deployment candidate
- [ ] Deployment is a routine, low-risk event
- [ ] Performance benchmarks run in CI (criterion); regressions block the pipeline
- [ ] Load testing establishes baseline capacity and failure modes
- [ ] Observability: tracing spans and metrics support canary comparison
- [ ] Feature flags decouple deployment from release
- [ ] DORA metrics are tracked

### Chapter 2: Pipeline First

This chapter has five phases, each ending with a concrete checkpoint where the reader runs something and sees it work. Greenfield checklist items are checked off in batches at each checkpoint.

**Phase 1: Local Development Environment.** Devcontainer, Podman, compose.yaml with PostgreSQL matching the production version. *Checkpoint: `podman compose up`, PostgreSQL is running.*

**Phase 2: Hello-World Endpoint + Build.** Axum health-check endpoint, `cargo-leptos` project structure, structured logging with `tracing`. *Checkpoint: `curl localhost:3000/health` returns "ok" with a log line.*

**Phase 3: Schema Foundation.** LinkML schema from `scimantic-ontology` for Question and QuestionStatus, `panschema generate` produces Rust types, SQL DDL, and SHACL shapes. *Checkpoint: `cargo build` passes with generated types.*

**Phase 4: CI Pipeline.** GitHub Actions workflow with rustfmt, clippy, cargo-nextest, cargo-audit, cargo-deny, cargo-vet, cargo-mutants (`--in-diff` on every push, full sweep nightly). Pre-commit hooks via prek mirroring CI (including private key detection). Dependabot, GitHub code scanning, secret scanning. Scheduled weekly security workflow. SLSA provenance attestation on releases. Schema generation step verifies consistency. *Checkpoint: push to trunk, pipeline goes green.*

**Phase 5: Infrastructure + Deployment.** Terraform configs for Linode (staging + production), HTTPS/TLS, secrets management, `/security-review` skill, pre-commit hooks, immutable artifacts, externalized configuration, rollback tested. *Checkpoint: hit the production URL over HTTPS, see "ok."*

**The reader learns:**
- Podman, devcontainers, compose.yaml: one-click environment setup, production-matching database locally
- Axum basics: async fn handlers, Router, Tokio runtime. `async`/`await` fundamentals.
- Structured logging with `tracing`: spans, events, `tracing-subscriber` for human-readable (dev) and JSON (prod) output
- LinkML basics: classes, attributes, enums in YAML. panschema: generating Rust types, SQL DDL, and SHACL shapes from the schema. The scimantic ontology as a versioned dependency.
- GitHub Actions: workflow files, triggers, jobs, steps. Pipeline-enforced quality gates.
- `rustfmt`, `clippy`, `cargo-audit`, `cargo-deny`, `cargo-vet`, `cargo-mutants` as pipeline gates
- `prek`: Rust-native pre-commit hooks mirroring CI, including private key detection
- Dependabot + GitHub security features (SAST, secret scanning)
- Scheduled weekly security workflow (cargo-audit, cargo-deny, cargo-vet)
- SLSA provenance attestation on release artifacts
- Terraform basics: providers, resources, state. Linode provisioning with HTTPS/TLS.
- Secrets management: GitHub Secrets, `.env`, never committing secrets
- Local security tooling: `/security-review` skill and pre-commit hooks
- Immutable artifacts, externalized configuration, rollback verification

**The reader ends with:** A URL they can hit (over HTTPS) that returns "ok," deployed through the full pipeline. The scimantic ontology schema is a versioned dependency, panschema is integrated into the build, `cargo-mutants --in-diff` runs as a pipeline gate, a nightly full mutation sweep is configured, Dependabot and GitHub security scanning are active, and every MinimumCD pipeline basic is in place.

### Chapter 3: The Database

**The reader builds:** A managed PostgreSQL database on Linode (via Terraform) for app state, an embedded Oxigraph store for the knowledge graph, the `questions` table (from the SQL DDL generated by panschema in Chapter 2), SHACL shapes for validating Question entities in the knowledge graph, a service layer function to create a question in both stores, and the first unit test.

**The reader learns:**
- Why PostgreSQL over SQLite — the MinimumCD principle of production-like environments from day one.
- Why dual databases — relational for app state, graph for knowledge. When to use which.
- SQLx: what it is, how it differs from an ORM, how `sqlx-cli` manages migrations. The initial migration comes from panschema's generated SQL DDL — the reader sees how the schema artifact flows into the database.
- Oxigraph: what it is, how RDF triples work, how SPARQL queries differ from SQL. Setting up an embedded store with RocksDB persistence. The reader writes their first SPARQL query.
- SHACL validation: how panschema-generated shapes constrain data entering the knowledge graph. The reader sees validation errors when data violates ontology constraints.
- Compile-time query checking with `query_as!` — what procedural macros are, what happens at build time vs. runtime, why this matters for pipeline confidence. The types used in `query_as!` are the same types panschema generated from LinkML.
- The service layer pattern: domain logic separated from any web framework, testable in isolation. Service functions operate on the generated domain types and abstract the dual database.
- `Result<T, E>` and the `?` operator in practice — writing fallible functions that propagate errors cleanly.
- Production error handling with `thiserror`: defining domain-specific error types (`ServiceError`, `DatabaseError`, `GraphError`), adding error context and chains, mapping errors to appropriate HTTP status codes. The difference between errors for developers (logs) and errors for users (responses).
- `cargo-nextest`: how it differs from `cargo test`, how to run specific tests, how to read output.
- Writing tests against real databases: test fixtures, transaction rollback, database isolation. Testing Oxigraph queries with an in-memory test store.
- Database operational practices: connection pool configuration and sizing with SQLx, understanding `EXPLAIN` output for query performance, adding indexes to support query patterns, N+1 query awareness. These are introduced briefly here and expanded as the data model grows in later chapters.
- Mutation testing in practice: running `cargo mutants` against the service layer after the first unit test passes. The reader sees their first surviving mutant — a function whose return value can be changed without any test failing — and writes a test to kill it. This is the "aha" moment: passing tests don't mean the tests are good.

**The reader ends with:** A passing test in CI that proves they can write to and read from both a production-like PostgreSQL database and an Oxigraph knowledge graph, using types, migrations, and SHACL shapes that trace back to the scimantic ontology. The service layer has domain-specific error types with context. The mutation testing pipeline gate (`--in-diff`) catches any untested behavior in the change. No UI yet.

### Chapter 4: The Web Frontend

**The reader builds:** A set of base Leptos components (Button, Input, Card, Layout), a Trunk to Theory theme from Tailwind v4 primitives, a component catalog route via theoria, and a questions page that renders research questions from the knowledge graph via a `#[server]` function.

**The reader learns:**
- What Leptos is: SSR, hydration, WASM compilation. How `cargo-leptos` coordinates the dual-target build (server binary + WASM client).
- Reactive signals: `create_signal`, getters and setters, fine-grained reactivity. Why Leptos doesn't use a virtual DOM.
- RSX syntax: HTML-like templates inside Rust macros, how Rust's type system catches template errors at compile time.
- Component-driven development: building composable UI components from the start. Base components (Button, Input, Card, Layout) are introduced here and reused in every subsequent chapter.
- Tailwind CSS v4: setting up the Trunk to Theory theme (color palette, typography, spacing tokens) in Tailwind's configuration. Composing Tailwind utilities inside Leptos components. Using the standalone CLI (Rust-native, no Node.js). Why component styles live in Rust code, not in CSS class names from a framework.
- theoria: setting up the component catalog, registering components with configurable props, using it as a development and documentation tool. The reader sees how isolated component development works when building a design system from Tailwind primitives.
- dokime: writing component-level tests that verify rendering, signal reactivity, and event handling without a full browser. Testing every prop combination for each component theoria catalogs.
- Accessibility from the start: semantic HTML structure in Leptos `view!` macros, ARIA roles and labels on custom components, keyboard navigation and focus management. WCAG 2.1 AA compliance as a design constraint, not an afterthought. The reader builds accessible components from the ground up rather than relying on a CSS framework's defaults. Accessibility-focused E2E tests with playwright-rust (keyboard-only navigation, screen reader label verification) are added to the testing suite.
- `#[server]` functions: how they cross the client/server boundary, how serialization works, why you can write a database query and a UI component in the same file.
- Ownership in the context of closures: why `move ||` appears everywhere in Leptos components, what it means, why Rust requires it.
- `wasm-bindgen` (briefly): what WASM is, how Rust compiles to it, the boundary between Rust and the browser's JavaScript runtime.

**The reader ends with:** A themed web page with a component library, deployed to production, that displays research questions fetched from the knowledge graph. The component explorer is accessible at a dev route. Read-only for now, no way to add questions yet.

### Chapter 5: Completing the Slice

**The reader builds:** A form to pose new research questions, form submission handling, persistence to both Oxigraph and PostgreSQL, and the first E2E test with playwright-rust.

**The reader learns:**
- Leptos forms and actions: `<ActionForm>`, `create_server_action`, handling form submission as a server function.
- Optimistic UI updates: how Leptos can update the UI before the server responds, and how to handle failures.
- **playwright-rust in depth:** The architecture (Rust API → JSON-RPC over stdio → Playwright server → browsers). Installing browsers in CI. Writing a meaningful E2E test: load the page, type a research question, click submit, verify it appears in the knowledge base. Assertions, waiting strategies, selectors.
- How E2E tests fit into the pipeline: the testing stage in GitHub Actions, headless browser execution, failure reporting.
- **Security-focused E2E tests (DAST):** Writing playwright-rust tests that probe the running application for vulnerabilities: attempting SPARQL injection via form inputs, testing for XSS in rendered output, verifying that unauthenticated requests are rejected. These run against the staging deployment as a pipeline stage.
- The complete ACD workflow executed for the first time: intent description → behavioral test specs → implementation → pipeline verification → deployment. This chapter makes the process explicit and names each stage.

- Mutation testing across the full slice: running `cargo mutants` against the complete vertical slice (service layer, server functions, form handling). The reader sees how different test layers (unit, integration, graph, E2E) kill different categories of mutants. Surviving mutants guide where to strengthen tests.

**The reader ends with:** A *deployed, tested, working feature*. A researcher can pose a question in their knowledge base via a web browser, and both functional and security E2E tests prove it works. Mutation testing confirms the test suite catches real regressions across the full stack. The first complete trip through the ACD workflow.

### Chapter 6: The REST API

**The reader builds:** Axum REST handlers with utoipa annotations, the OpenAPI spec, Swagger UI, and a contract test.

**The reader learns:**
- Why the dual-interface architecture matters — the reader has been using Leptos server functions, and now they see why that's not enough for external consumers like CLI tools and Jupyter integrations.
- Extracting the service layer: it already exists (built in Chapter 3), so this chapter is about wiring it to a second set of handlers, not rewriting domain logic.
- utoipa: `#[utoipa::path]`, `ToSchema` derive macro, `OpenApiRouter`. How OpenAPI docs are generated at compile time from the same Rust types the Leptos frontend uses.
- `From`/`Into` traits: converting between domain models and API response/request types.
- Swagger UI: serving it from the same binary, using it to explore and test the API.
- Contract testing: writing tests that verify the API conforms to its OpenAPI spec.
- Traits in practice: how the service layer defines an interface that both Leptos server functions and REST handlers consume.
- **CORS policy:** Configuring Cross-Origin Resource Sharing for the REST API. Which origins are allowed, why this matters for external consumers and browser security.
- **Input validation at the API boundary:** Validating request payloads at the handler level before they reach the service layer. Panschema-generated types enforce shape; explicit validation enforces business rules (string length, allowed characters, valid URIs for knowledge graph entities, etc.).
- **Pagination:** Cursor-based pagination for list endpoints. Why offset-based pagination breaks under concurrent writes. Query parameters, response metadata (`next_cursor`, `has_more`), and the Leptos frontend consuming paginated responses.
- **HTTP caching basics:** `Cache-Control` and `ETag` headers on read endpoints. Setting up caching that works correctly with the dual-interface design.
- **API versioning strategy:** Why `/api/v1/` exists from the start, when to introduce `/api/v2/`, deprecation headers and client communication. Briefly introduced here; becomes concrete when a later chapter changes the API shape.

**The reader ends with:** The same "pose a research question" feature accessible via both the web UI (Leptos) and a REST endpoint (documented with Swagger UI), with CORS configured, input validation enforced, list endpoints paginated, and caching headers in place. The dual-interface architecture is real and working.

### Chapters 7+: Building the Scientific Workflow

With the full stack and ACD workflow in place, subsequent features move faster. Each introduces the next entity in the scimantic scientific workflow, along with new technical concepts. The pipeline, architecture, and testing patterns are established.

- **Chapter 7: Evidence** — The second scimantic entity. Teaches linking evidence to questions in the knowledge graph, SPARQL queries for graph traversal, the Oxigraph query API from Rust. The reader builds evidence creation and the first cross-entity relationship. State transitions for questions (open → investigating when evidence is linked).
- **Chapter 8: User Authentication** — Teaches `argon2` password hashing, `tower-sessions`, middleware, protecting routes. Introduces secrets lifecycle: credential storage, rotation without downtime, different secrets per environment. Questions and evidence are now owned by authenticated users.
- **Chapter 9: Hypotheses** — The third scimantic entity. Teaches forming hypotheses from evidence, more complex knowledge graph traversal, and multi-entity UI views. The reader builds a hypothesis that references multiple pieces of evidence and explores graph query patterns for "which evidence supports this hypothesis?"
- **Chapter 10: Experiments** — The fourth scimantic entity. Teaches designing experiments to test hypotheses, complex forms with structured data (protocols, variables, expected outcomes), and richer Leptos fine-grained reactivity via an experiment designer with live preview. Database practices expand: query performance with graph joins, index strategy for complex SPARQL patterns.
- **Chapter 11: Results & Analysis** — The fifth scimantic entity, completing the scientific workflow. Teaches recording results, bridging Oxigraph and PostgreSQL (results reference knowledge graph entities but store quantitative data in PostgreSQL), and cross-database queries. Performance benchmarking with `criterion`: response time budgets for graph traversal endpoints, preventing performance regressions in CI.
- **Chapter 12: Security Hardening** — Teaches CSP (Content Security Policy) headers, rate limiting with Tower middleware, SBOM generation, hardened container images for deployment. Secrets rotation: automating credential rotation for database and API keys without downtime. Database disaster recovery: testing the restore process against real data (backups configured in Ch 2 via Terraform).
- **Chapter 13: Feature Flags** — Teaches decoupling deployment from release (MinimumCD optimization phase).
- **Chapter 14: Progressive Rollout** — Teaches canary deployments, monitoring, rollback in practice. Observability as a prerequisite: using `tracing` spans and metrics to compare canary vs. stable traffic. Load testing: establishing baseline capacity, stress testing failure modes, understanding system limits before rolling out to 100%.

The exact chapter breakdown for 7+ will be determined after the first six chapters are drafted and the page budget is clearer.

---

## 7. Repository Strategy

| Repo | Visibility | Purpose |
|---|---|---|
| `t2t` | Public | Monorepo: book manuscript (mdbook/Markdown), planning docs, figures, and the Leptos + Axum application code. Simultaneously the open source product and the code readers follow along with. |
| `scimantic-ontology` | Public | LinkML schema for the scimantic domain (Questions, Evidence, Hypotheses, Experiments, Results). Versioned artifact consumed by t2t as a dependency. |
| `panschema` | Public | Universal Rust data modeling tool. Generates Rust types, SQL DDL, SHACL shapes, JSON Schema, and visualizations from LinkML. (Author's project, dogfooded in the book.) |
| `theoria` | Public | Rust-native component explorer for Leptos (author's project, dogfooded in the book). |
| `dokime` | Public | Rust-native component testing framework for Leptos (author's project, dogfooded in the book). |
| `playwright-rust` | Public | Rust language bindings for Microsoft Playwright (author's project, dogfooded in the book). |
| `t2t-commercial` | Private | Thin commercial layer only: subscription integration, billing logic, production secrets templates. Depends on `t2t` as its core — a deployment wrapper, not a fork. |

### Why `t2t` Stands Alone

The repo should stand on its own as an open source project. People who've never heard of *Trunk to Theory* should be able to find it, understand what it is, and use it. The book drives people *to* the repo, but the repo shouldn't look like it only exists for the book. That's the difference between a real open source project and a "companion code" repo that nobody touches after publication.

### Chapter Tags (Reader Experience)

Following the *Obey the Testing Goat* model, each chapter's ending state is a git tag. The reader can check out any tag to see the codebase at that point:

```
main              ← the current production-ready state (always ahead of the book)
tags:
  chapter-01      ← end state after Chapter 1 (design docs, CLAUDE.md)
  chapter-02      ← end state after Chapter 2 (pipeline + hello world deployed)
  chapter-03      ← end state after Chapter 3 (dual database + service layer + first test)
  chapter-04      ← end state after Chapter 4 (Leptos frontend, read-only)
  chapter-05      ← end state after Chapter 5 (add question form + E2E test)
  chapter-06      ← end state after Chapter 6 (REST API + utoipa + Swagger)
  ...
```

Tags are immutable snapshots of `main` at specific points in the book's development history — not branches. This is consistent with the MinimumCD principle of immutable artifacts, and with the trunk-based development the book teaches.

`main` is the living, evolving product. It may be ahead of what the book covers, because Trunk to Theory continues to develop after publication.

### README Integration

The `t2t` README includes a section like:

```markdown
## Following Along with Trunk to Theory

This codebase is the companion to [Trunk to Theory](link).
Each chapter's ending state is tagged:

    git checkout chapter-02  # Pipeline + hello world
    git checkout chapter-05  # First complete feature with E2E tests

To follow along from scratch, start at Chapter 1 of the book with an empty repo.
To jump in at a specific chapter, check out the corresponding tag.
```

### Repository Layout (`t2t`)

Monorepo with three top-level directories, mirroring the ACD principle of everything-as-code and versioned together:

```
t2t/
├── app/                          # Leptos + Axum application
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs               # Axum server setup, route mounting
│   │   ├── app.rs                # Leptos root component
│   │   ├── components/           # Leptos UI components
│   │   ├── pages/                # Leptos page components (routes)
│   │   ├── server/               # Server functions (Leptos #[server])
│   │   ├── api/                  # REST API handlers (utoipa-annotated)
│   │   ├── services/             # Shared service layer (domain logic)
│   │   ├── models/               # Data types (shared by Leptos + API)
│   │   ├── db/                   # SQLx queries and migrations (PostgreSQL)
│   │   └── graph/                # Oxigraph queries and SHACL validation
│   ├── migrations/               # SQLx database migrations
│   ├── tests/                    # Integration and E2E tests
│   │   ├── integration/
│   │   ├── graph/                # Oxigraph/SPARQL tests
│   │   └── e2e/                  # playwright-rust E2E tests
│   ├── style/                    # Tailwind CSS
│   └── Cargo.lock
├── book/                         # mdbook manuscript
│   ├── book.toml                 # mdbook configuration
│   ├── src/
│   │   ├── SUMMARY.md            # Table of contents
│   │   ├── ch01-design-decisions.md
│   │   ├── ch02-pipeline-first.md
│   │   ├── ch03-the-database.md
│   │   ├── ch04-the-web-frontend.md
│   │   ├── ch05-completing-the-slice.md
│   │   ├── ch06-the-rest-api.md
│   │   └── ...
│   ├── images/                   # Figures and diagrams
│   └── book-plan.md              # This planning document
├── infra/                        # Terraform IaC
│   ├── main.tf
│   ├── variables.tf
│   ├── outputs.tf
│   ├── environments/
│   │   ├── staging.tfvars
│   │   └── production.tfvars
│   └── modules/
│       ├── compute/              # Linode instances
│       ├── database/             # Managed PostgreSQL
│       └── networking/           # NodeBalancer, DNS
├── schema/                       # Local schema artifacts (generated from scimantic-ontology)
│   ├── t2t.yaml                  # App-specific schema extensions
│   └── generated/                # panschema output (Rust types, SQL DDL, SHACL shapes, JSON Schema)
├── .devcontainer/
│   └── devcontainer.json         # VS Code devcontainer (full toolchain + PostgreSQL + Oxigraph)
├── compose.yaml                  # Podman/Docker Compose (PostgreSQL for local dev)
├── Containerfile                 # Production image (multi-stage, hardened in ch12)
├── .github/
│   └── workflows/
│       ├── ci.yml                # Build, test, lint on every push
│       ├── deploy-staging.yml    # Deploy to staging on trunk merge
│       └── deploy-production.yml # Deploy to production (manual gate or auto)
├── .claude/                      # ACD agent configuration + skills
│   └── skills/                   # /security-review, ACD workflow skills
├── .agent/                       # ACD workflow definitions
│   └── workflows/
├── CLAUDE.md                     # Project context for AI agents
├── README.md
└── docs/
    └── intent/                   # ACD intent descriptions (only prose artifacts)
```

### Manuscript Tooling (mdbook — Rust-Based)

The `book/` directory within the monorepo uses mdbook (Rust) for all chapter content. The entire book toolchain is Rust-based, consistent with the book's thesis. Key tooling decisions:

- **Code inclusion:** Code snippets are included directly from `app/src/` via mdbook's `{{#include ../../app/src/file.rs:anchor_name}}` directive. In the Rust source files, anchors are placed inside comments (`// ANCHOR: name` / `// ANCHOR_END: name`). This follows the approach used by *The Rust Programming Language* book. No separate `listings/` directory — the monorepo structure makes direct inclusion possible.
- **Chapter tags as snapshots:** Each chapter's code state is captured by a git tag on the monorepo. The reader can check out any tag to see both the manuscript and the application code at that point in the book's progression.
- **Output formats:** `mdbook build` produces both HTML (the primary, freely accessible version) and print-quality PDF via mdbook-typst-pdf (Rust-based, using the Typst typesetting engine). PDF is generated on every push in CI and attached to GitHub Releases on tag. EPUB via mdbook-epub or Pandoc conversion if needed.
- **Admonitions:** mdbook-admonish plugin (using fork `padamson/mdbook-admonish` until upstream merges PR #235 — tracked in `dogfood-gaps.md`).
- **Quizzes:** mdbook-quiz plugin (using fork `padamson/mdbook-quiz` until upstream merges PR #62 — tracked in `dogfood-gaps.md`).
- **Code callouts:** Planned as a custom mdbook preprocessor (Rust). Until then, use numbered lists after code blocks keyed to line numbers.
- **Code testing:** `mdbook test` compiles and runs Rust code blocks, verifying all examples work. This is a major advantage over non-Rust toolchains.
- **Dogfooding:** When mdbook or its plugins lack features, the approach is to fork/fix/contribute rather than switch to non-Rust alternatives. Gaps are tracked via `/blocker` and `/audit-dogfood` skills.

### Writing Workflow

During the pre-proposal and writing phases, the book and application are developed in parallel within the monorepo. Each chapter draft corresponds to a sequence of commits. When a chapter is finished:

1. The monorepo is tagged (e.g., `chapter-03`).
2. Chapter Markdown in `book/src/` references application source via `{{#include ../../app/src/...}}` with ANCHOR comments marking the includable regions.
3. `mdbook test` verifies all code listings compile and run.
4. The `/book-extract-listings` skill verifies all `{{#include}}` directives resolve to valid anchors.

This approach ensures the book always references live, tested application code.

---

## 8. Development Approach

### Pre-Proposal Phase (Current)

1. **Set up the pipeline first.** Create the `t2t` monorepo. GitHub Actions workflow + Terraform configs for Linode. Deploy a health-check endpoint to production before writing any application code.
2. **Build the first vertical slice.** "A researcher can pose a question in their knowledge base." Leptos frontend + Oxigraph knowledge graph + PostgreSQL app state, deployed through the pipeline.
3. **Write the first ACD cycle.** Intent description → behavioral test specs → implementation → pipeline verification → deployment. This becomes the template for every subsequent chapter.
4. **Draft Chapters 1-2.** Chapter 1 (design decisions, as outlined above) and Chapter 2 (pipeline setup and hello-world deployment). Tag the monorepo at each chapter boundary.

### Writing Phase

Each chapter follows the ACD workflow: introduce a feature as an intent description, write behavioral test specs, implement through the pipeline, deploy. The manuscript and application codebase evolve in lockstep within the monorepo. Each completed chapter results in a new tag.

---

## 9. The Broader Trunk to Theory Product (Beyond the Book)

The book covers the web application and REST API. The broader product includes:

- **CLI tool** (Rust, consuming the REST API built in the book) for researchers who prefer terminal workflows
- **Jupyter integration** (consuming the REST API) for notebook-based research workflows
- **VS Code extension** (future) for IDE-integrated knowledge management
- **Subscription management** (Stripe integration) for hosted version
- **Open-source core** — The Trunk to Theory codebase serves as a real-world open-source example of a full-stack Rust app with a dual-database architecture and knowledge graph

The CLI tool, Jupyter integration, VS Code extension, and subscription features are not covered in the book but are enabled by the architecture decisions made in the book (specifically, the OpenAPI-compliant REST API and the well-separated service layer).

---

## 10. Key References

### MinimumCD
- **MinimumCD Manifesto:** https://minimumcd.org
- **Greenfield CD Guide:** https://beyond.minimumcd.org/docs/migrate-to-cd/greenfield/
- **Agentic CD:** https://beyond.minimumcd.org/docs/agentic-cd/
- **ACD Getting Started:** https://beyond.minimumcd.org/docs/agentic-cd/getting-started/
- **Agent Delivery Contract:** https://beyond.minimumcd.org/docs/agentic-cd/specification/first-class-artifacts/
- **CD Testing:** https://beyond.minimumcd.org/docs/testing/

### Rust Ecosystem
- **Leptos:** https://www.leptos.dev / https://github.com/leptos-rs/leptos
- **Leptos Book:** https://book.leptos.dev
- **Leptos + Axum Starter:** https://github.com/leptos-rs/start-axum
- **Axum:** https://github.com/tokio-rs/axum
- **SQLx:** https://github.com/launchbadge/sqlx
- **Oxigraph:** https://github.com/oxigraph/oxigraph
- **sophia_rs:** https://github.com/pchampin/sophia_rs
- **utoipa:** https://github.com/juhaku/utoipa
- **utoipa-axum:** https://docs.rs/utoipa-axum
- **playwright-rust:** https://github.com/padamson/playwright-rust
- **cargo-leptos:** https://github.com/leptos-rs/cargo-leptos
- **cargo-nextest:** https://nexte.st
- **panschema:** https://github.com/padamson/panschema
- **scimantic-ontology:** https://github.com/padamson/scimantic-ontology
- **theoria:** https://github.com/padamson/theoria
- **dokime:** https://github.com/padamson/dokime
- **Tailwind CSS v4:** https://tailwindcss.com

### Book Toolchain
- **mdbook:** https://rust-lang.github.io/mdBook/
- **mdbook-admonish:** https://github.com/tommilligan/mdbook-admonish
- **mdbook-typst-pdf:** https://github.com/KaiserY/mdbook-typst-pdf (print-quality PDF via Typst; used in CI and releases)
- **The Rust Programming Language (book source):** https://github.com/rust-lang/book (reference for listings pattern)
- **minimumcd-mcp:** https://github.com/padamson/minimumcd-mcp (Rust MCP server for MinimumCD Practice Guide context; authoring tool, mentioned in Appendix A)

### Infrastructure
- **Linode Terraform Provider v3:** https://www.akamai.com/blog/developers/linode-terraform-provider-v3-0-0
- **Linode Terraform Docs:** https://www.linode.com/docs/guides/how-to-build-your-infrastructure-using-terraform-and-linode/
- **Linode Managed PostgreSQL with Terraform:** https://www.linode.com/docs/guides/managed-postgresql-databases-on-akamai-cloud-with-terraform/

### Semantic Web / Knowledge Graphs
- **RDF Primer:** https://www.w3.org/TR/rdf11-primer/
- **SPARQL 1.1 Query Language:** https://www.w3.org/TR/sparql11-query/
- **SHACL (Shapes Constraint Language):** https://www.w3.org/TR/shacl/
- **LinkML:** https://linkml.io

### Existing Rust Web Books (Competitive Landscape)
- **Rust Web Development** (Bastian Gruber) — Server-side only, uses Warp, no frontend/IaC/CD
- **Rust Servers, Services, and Apps** (Prabhu Eshwarla) — Uses Actix, no IaC/CD, no ACD methodology

---

## 11. Open Questions

- [ ] **Leptos version stability:** Leptos 0.8 is current with a path to 1.0. API has stabilized significantly but minor breaking changes still occur. Monitor for stability during writing.
- [ ] **Licensing model:** Determine the boundary between open-source Trunk to Theory core and the commercial subscription product.
- [ ] **Linode cost for readers:** Estimate the Linode bill a reader would incur following along with the book. Keep it minimal.
- [ ] **ACD tooling maturity:** The ACD section of MinimumCD is relatively new. Stay aligned with updates as the framework evolves.
- [ ] **Oxigraph production deployment:** Oxigraph is embeddable (in-process) for the book. Determine whether a separate Oxigraph server is needed for production scale, and whether that changes the Terraform/deployment story.
- [ ] **SHACL validation performance:** Measure the overhead of SHACL validation on every write to the knowledge graph. Determine whether validation should be synchronous or deferred.
- [ ] **scimantic-ontology versioning:** Define the contract between the ontology repo and the t2t app. How does a breaking ontology change flow through the pipeline?
