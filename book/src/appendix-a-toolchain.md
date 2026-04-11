# Appendix A: Our Toolchain

<!-- This appendix describes the tools built or extended during the writing of this book. -->

## minimumcd-mcp

While writing this book, we used a Rust-based MCP (Model Context Protocol) server to give our AI coding assistant full access to the MinimumCD Practice Guide content. When drafting a chapter, the assistant could query specific ACD concepts, cross-reference greenfield guidance, and verify that our practices aligned with the framework, all without leaving the editor.

The server indexes the markdown content from a local clone of the Practice Guide and exposes it as searchable context via the MCP protocol. It runs as a stdio transport, integrating with Claude Code, VS Code, and other MCP-compatible tools.

If you're using an AI coding assistant while working through this book (and we recommend it), you can install minimumcd-mcp from crates.io:

```bash
cargo install minimumcd-mcp
```

Then add it to your Claude Code configuration (or equivalent) pointing at the same Practice Guide clone you set up in "Before You Begin." Your assistant will have the same contextual access to MinimumCD and ACD content that we had while writing.

See the [minimumcd-mcp repository](https://github.com/padamson/minimumcd-mcp) for setup instructions.

## Other Tools Built for This Book

Several tools introduced in this book were built by the author to fill gaps in the Rust ecosystem:

- **[panschema](https://github.com/padamson/panschema)** generates Rust types, SQL DDL, SHACL shapes, and JSON Schema from LinkML models. Introduced in Chapter 2, used throughout.
- **[playwright-rust](https://github.com/padamson/playwright-rust)** provides Rust language bindings for Microsoft Playwright. Used for E2E and security testing from Chapter 5.
- **[theoria](https://github.com/padamson/theoria)** is a Rust-native component explorer for Leptos. Introduced in Chapter 4.
- **[dokime](https://github.com/padamson/dokime)** is a Rust-native component testing framework for Leptos. Introduced in Chapter 4.

Each of these was developed alongside the book and the Scimantic application. When a tool needed a new feature to support a chapter's content, we paused, built the feature, and came back. That process is part of the story this book tells about what it means to build with Rust today: sometimes the tool you need doesn't exist yet, and you build it.
