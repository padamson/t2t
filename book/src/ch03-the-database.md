# The Database

<!-- The reader builds: -->
<!-- - Managed PostgreSQL on Linode via Terraform (app state: users, sessions) -->
<!-- - Embedded Oxigraph instance for the knowledge graph -->
<!-- - questions table in PostgreSQL from panschema-generated SQL DDL -->
<!-- - SHACL shapes loaded into Oxigraph from panschema-generated output -->
<!-- - Service layer function to create a question using generated Rust types -->
<!-- - First unit test -->

<!-- The reader learns: -->
<!-- - Why PostgreSQL over SQLite (production-like environments from day one) -->
<!-- - Why dual databases: PostgreSQL for app infrastructure, Oxigraph for the knowledge graph -->
<!-- - Oxigraph: what it is (Rust-native RDF triple store), SPARQL basics, embedding vs. standalone -->
<!-- - RDF fundamentals: triples, URIs, Turtle format, why knowledge graphs use this model -->
<!-- - SQLx: what it is, how it differs from an ORM, sqlx-cli for migrations -->
<!-- - Compile-time query checking with query_as! -->
<!-- - Service layer pattern: domain logic separated from any web framework -->
<!-- - How panschema-generated types flow into both database layers -->
<!-- - Result<T, E> and the ? operator in practice -->
<!-- - Production error handling: thiserror for domain error types, error context/chains, mapping errors to HTTP status codes -->
<!-- - cargo-nextest: parallel test runner, structured output -->
<!-- - Writing tests against real databases: fixtures, transaction rollback, database isolation -->
<!-- - Database operational practices: connection pool config, EXPLAIN, indexes, N+1 awareness -->
<!-- - Mutation testing: first cargo mutants run, seeing a surviving mutant, writing a test to kill it -->
