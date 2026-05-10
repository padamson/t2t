# The REST API

<!-- The reader builds: -->
<!-- - Axum REST handlers with utoipa annotations -->
<!-- - OpenAPI spec and Swagger UI -->
<!-- - Contract tests -->
<!-- - CORS policy configuration -->
<!-- - Input validation at the API boundary -->
<!-- - Read-only SPARQL endpoint for power users and integrations -->

<!-- The reader learns: -->
<!-- - Why the dual-interface architecture matters -->
<!-- - Wiring the service layer to REST handlers (it already exists from Ch3) -->
<!-- - utoipa: #[utoipa::path], ToSchema, OpenApiRouter -->
<!-- - From/Into traits: converting between domain models and API types -->
<!-- - Swagger UI served from the same binary -->
<!-- - Contract testing: API conforms to its OpenAPI spec -->
<!-- - CORS: which origins, why it matters for external consumers (CLI tools, Jupyter integrations) -->
<!-- - Input validation: panschema types enforce shape, explicit validation for business rules -->
<!-- - Pagination: cursor-based for list endpoints, query params, response metadata -->
<!-- - HTTP caching: Cache-Control, ETag headers on read endpoints -->
<!-- - API versioning strategy: /api/v1/ from the start, deprecation headers -->
<!-- - Traits in practice: service layer interface consumed by Leptos and REST handlers -->
<!-- - SPARQL endpoint: exposing a read-only SPARQL endpoint for power users and integrations -->
<!-- - **Workspace extraction (second split):** extract `app-api` for REST request/response -->
<!--   types that need to be shared with external consumers (the scimantic CLI, future -->
<!--   Jupyter integrations). The CLI repo can then `cargo add app-api` (or pin to git) -->
<!--   instead of duplicating types. Builds on the workspace conversion from Ch 3. -->
