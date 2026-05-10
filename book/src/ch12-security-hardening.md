# Security Hardening

<!-- Teaches: CSP (Content Security Policy) headers -->
<!-- Teaches: rate limiting with Tower middleware -->
<!-- Teaches: SBOM generation -->
<!-- Teaches: hardened container images (minimal base, non-root, multi-stage build) -->
<!-- Teaches: secrets rotation — automating credential rotation for database and API keys without downtime -->
<!-- Teaches: database disaster recovery — test the restore process against real data (backups configured in Ch 2) -->

<!-- Local schema additions (app/schema/scimantic-server.yaml):
       - AuditLog: append-only record of security-relevant events (login,
         permission change, API token use, data export, etc.). Includes
         actor (User), action (enum), target (URI), timestamp, ip_address,
         user_agent, request_id.

     LinkML constructs introduced/exercised:
       - Mixin classes (e.g., a `Timestamped` mixin used by AuditLog and
         other entities that need created_at/updated_at)
       - Append-only modeling: schema-level documentation that updates are
         disallowed, even if the storage layer doesn't enforce it
       - Deterministic ID generation patterns (UUIDv7 for time-ordered IDs)
       - Optional: schema versioning — when AuditLog evolves, how the schema
         records the version so old records remain interpretable

     Demonstrates that LinkML scales to compliance-grade modeling without
     reaching for a separate audit framework. -->

