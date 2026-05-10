# Hypotheses

<!-- The reader builds: -->
<!-- - Hypothesis entity: testable statements derived from accumulated evidence -->
<!-- - Hypothesis-Evidence and Hypothesis-Question relationships in the knowledge graph -->
<!-- - Hypothesis tree visualization (Leptos fine-grained reactivity) -->
<!-- - Collaborative hypothesis editing with real-time updates -->

<!-- The reader learns: -->
<!-- - Complex knowledge graph modeling: multi-hop relationships (Question → Evidence → Hypothesis) -->
<!-- - SPARQL path queries: traversing relationship chains -->
<!-- - Multi-user data modeling and authorization logic -->
<!-- - Real-time updates via server-sent events or WebSockets -->
<!-- - Leptos fine-grained reactivity via the hypothesis tree with live updates as the user types -->
<!-- - Graph visualization in the browser: rendering relationship trees from SPARQL results -->

<!-- Local schema additions (app/schema/scimantic-server.yaml):
       - Organization: multi-tenancy unit (id, name, slug, created_at)
       - Membership: User ↔ Organization with role (owner, editor, viewer)
       - Optionally: Invitation (pending Membership)

     Demonstrates LinkML *across* multiple classes with relationships,
     a permissions enum (role values), and the difference between modeling
     authorization in the local schema (these are app-infrastructure concerns)
     vs in the scientific schema (which has no notion of "tenant" — it's about
     scientific entities). The chapter is also where multi-tenant data isolation
     becomes a real concern in PostgreSQL (organization_id columns, query
     scoping) and in Oxigraph (named-graph-per-organization).

     Hypothesis itself comes from scimantic-schema (external), not the local
     schema. The chapter teaches authorization on top of scientific entities,
     not authoring of those entities. -->

