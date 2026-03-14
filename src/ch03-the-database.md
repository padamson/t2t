# The Database

<!-- The reader builds: -->
<!-- - Managed PostgreSQL on Linode via Terraform -->
<!-- - grocery_items table from panschema-generated SQL DDL -->
<!-- - Service layer function to insert an item using generated Rust types -->
<!-- - First unit test -->

<!-- The reader learns: -->
<!-- - Why PostgreSQL over SQLite (production-like environments from day one) -->
<!-- - SQLx: what it is, how it differs from an ORM, sqlx-cli for migrations -->
<!-- - Compile-time query checking with query_as! -->
<!-- - Service layer pattern: domain logic separated from any web framework -->
<!-- - How panschema-generated types flow into the database layer -->
<!-- - Result<T, E> and the ? operator in practice -->
<!-- - Production error handling: thiserror for domain error types, error context/chains, mapping errors to HTTP status codes -->
<!-- - cargo-nextest: parallel test runner, structured output -->
<!-- - Writing tests against a real database: fixtures, transaction rollback -->
<!-- - Database operational practices: connection pool config, EXPLAIN, indexes, N+1 awareness -->
<!-- - Mutation testing: first cargo mutants run, seeing a surviving mutant, writing a test to kill it -->
