# The Database

<!-- The reader builds: -->
<!--
   This chapter wires the data layer end-to-end across two databases driven by
   two LinkML schemas. It is the densest chapter in the first vertical slice
   because everything below the service layer lands here.

   Schema sources (introduced in this chapter):
   - LOCAL: app/schema/scimantic-server.yaml — extended from Ch 2 Phase 3 with
     a Session class. Drives PostgreSQL DDL for users + sessions tables.
   - EXTERNAL: padamson/scimantic-schema, consumed via panschema's
     schema-manager (`panschema add scimantic-schema@<version> --source
     github:padamson/scimantic-schema`). NOT a git submodule; panschema
     fetches the schema into its local cache and reads it from there.
     Source of truth for Question and the rest of the scientific entities.
     Drives SHACL shapes loaded into Oxigraph at startup.

   Reader-facing tooling that lands in this chapter:
   - app/panschema.toml — the manifest declaring both schemas as
     dependencies, with [generate.<schema>] sections wiring outputs to
     paths in the workspace
   - app/panschema.lock — the lockfile pinning checksums; committed
   - `panschema fetch && panschema generate` — the canonical command
     pair that reproduces the generated artifacts

   Databases:
   - Managed PostgreSQL on AWS via Terraform (users, sessions, app state)
   - Embedded Oxigraph instance (knowledge graph: Question RDF triples)

   Generated artifacts (from panschema running over both schemas):
   - Rust types for User, Session (from local) and Question (from external)
   - SQL DDL for users + sessions tables (from local)
   - SHACL shapes (from external) loaded into Oxigraph at startup

   App code that lands:
   - Service layer with hexagonal trait ports (UserRepository, QuestionRepository,
     KnowledgeGraph) and adapters (PostgresUserRepository, OxigraphKnowledgeGraph)
   - First unit test against in-memory adapters
-->

<!-- The reader learns: -->
<!-- - Why dual databases: PostgreSQL for app infrastructure (users, sessions);
       Oxigraph for the knowledge graph (Question and its scientific relationships). -->
<!-- - Why PostgreSQL over SQLite (production-like environments from day one) -->
<!-- - Why Oxigraph (Rust-native RDF triple store, SPARQL 1.1, embeddable in the
       Axum binary — no separate process). -->
<!-- - RDF fundamentals: triples, URIs, Turtle format, why knowledge graphs use this model -->

<!-- - The two-schema architecture in practice. The reader writes ONE
       panschema.toml manifest declaring both schemas as dependencies; runs
       `panschema fetch` (downloads scimantic-schema into the local cache,
       writes panschema.lock); runs `panschema generate` (produces Rust types
       + SQL DDL from the local schema, Rust types + SHACL shapes from the
       external schema). Same tool, one invocation, two sources, two
       output destinations. -->
<!-- - Adding scimantic-schema as a panschema dependency (and what
       "versioned schema as a managed dependency" means — analogous to
       cargo dependencies). Reader sees the manifest + lockfile workflow.
       The schema lives in panschema's local cache, not in the t2t repo
       working tree. -->
<!-- - The publishing standard: scimantic-schema's `panschema-publish.toml`
       at the repo root tells panschema what to fetch. Reader peeks at it
       (5 lines) to understand how schema repos identify themselves. -->
<!-- - Extending the local schema (Session added to what was just User in Ch 2 Phase 3).
       Demonstrates schema evolution: edit YAML, run `panschema generate`, types
       and SQL update. The local-path source means no fetch needed; just regenerate. -->

<!-- - SQLx: what it is, how it differs from an ORM, sqlx-cli for migrations.
       The first migration comes from panschema-generated SQL DDL, not hand-written. -->
<!-- - Compile-time query checking with query_as! — the User and Session types in
       query_as! macros come from panschema-generated code, closing the loop. -->
<!-- - Loading SHACL shapes into Oxigraph at startup; the shapes constrain what
       triples can be written. (Shapes from panschema/external schema; triples
       written via service-layer ops from generated Question type.) -->

<!-- - Service layer pattern: domain logic separated from any web framework. -->
<!-- - Hexagonal architecture callout: traits as ports (UserRepository,
       QuestionRepository, KnowledgeGraph), adapters (PostgresUserRepository,
       OxigraphKnowledgeGraph). Name the pattern here after the reader has built
       it — "what we just built has a name." -->
<!-- - **Workspace extraction:** convert app/ from a single crate to a Cargo workspace
       here, motivated by the trait-based service layer. Extract `scimantic-core`
       for the trait definitions and domain types (no framework deps). Keep
       `scimantic-server` as the runnable Axum binary. The hexagonal architecture
       callout is the narrative justification for the workspace split. Update CI
       workflows to be workspace-aware (cargo commands at workspace root). -->

<!-- - Result<T, E> and the ? operator in practice. -->
<!-- - Production error handling: thiserror for domain error types, error
       context/chains, mapping errors to appropriate HTTP status codes. The
       difference between errors for developers (logs) and errors for users
       (responses). -->

<!-- - cargo-nextest: parallel test runner, structured output. -->
<!-- - Writing tests against real databases: fixtures, transaction rollback,
       database isolation. -->
<!-- - Writing tests against the in-memory adapter (no DB required) for fast
       unit-level coverage of the service layer. The hexagonal split makes both
       possible. -->
<!-- - Database operational practices: connection pool config (SQLx),
       EXPLAIN for query performance, adding indexes, N+1 awareness. -->
<!-- - Mutation testing: first cargo mutants run, seeing a surviving mutant,
       writing a test to kill it. The "aha" moment that passing tests don't
       mean the tests are good. -->

<!-- Chapter checkpoint: -->
<!-- - `cargo nextest run` passes. -->
<!-- - PostgreSQL has users + sessions tables created from panschema-generated DDL. -->
<!-- - Oxigraph in-process has SHACL shapes loaded from panschema-generated output. -->
<!-- - Service layer can create a User in PostgreSQL and a Question in Oxigraph
       through the trait-based ports, exercised by a unit test. -->

<!-- Greenfield checklist items checked off: -->
<!--   - PostgreSQL adapter wired with SQLx + compile-time query checking -->
<!--   - Oxigraph adapter wired with SHACL validation at startup -->
<!--   - Hexagonal service layer with trait ports + adapters (no framework deps in core) -->
<!--   - app/ converted to Cargo workspace (scimantic-core + scimantic-server) -->
<!--   - First unit test passes (against in-memory adapter) -->
<!--   - First integration test passes (against real PostgreSQL via SQLx fixtures) -->
<!--   - cargo-mutants run; first surviving mutant identified and killed -->
<!--   - thiserror domain error types in scimantic-core -->
