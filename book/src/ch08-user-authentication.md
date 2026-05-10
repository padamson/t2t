# User Authentication

<!-- Teaches: argon2 password hashing, tower-sessions, middleware, protecting routes -->
<!-- Teaches: secrets lifecycle — credential storage, rotation without downtime, per-environment secrets -->

<!-- Local schema additions (app/schema/scimantic-server.yaml):
       - Session: extended/refined from Ch 3's stub for tower-sessions integration
         (id, user_id, expires_at, csrf_token, etc.)
       - ApiToken: for programmatic auth (REST API + CLI consumers)
         (id, user_id, scopes, created_at, last_used_at, hashed_token)

     Demonstrates LinkML enum type for scope values, datetime range, and
     multi-class relationships (User → Session, User → ApiToken). Reader extends
     the schema; panschema regenerates Rust types and SQL DDL; new tables wire
     into the existing data layer without restructuring. -->
