---
name: extract-listings
description: Extract code listings from cuisineiq-rust at a chapter tag into listings/chXX/
user-invocable: true
disable-model-invocation: true
argument-hint: "<chapter-number> [tag-name]"
---

# Extract Code Listings

Extract code snapshots from the cuisineiq-rust repo at a specific chapter tag
into the per-chapter `listings/chXX/` directory. These listings are referenced
by chapter markdown files via `{{#include}}` directives with ANCHOR comments.

## Process

1. **Parse the chapter number** from the first argument (e.g., "02" from `$ARGUMENTS`).
   If no tag name is given as the second argument, use `chapter-XX` as the default tag.

2. **Check the submodule.** Verify `code/cuisineiq-rust/` exists and is a git repo.
   If the submodule isn't initialized, run:
   ```
   git submodule update --init code/cuisineiq-rust
   ```

3. **Check the tag exists** in cuisineiq-rust:
   ```
   cd code/cuisineiq-rust && git tag -l "chapter-XX"
   ```
   If it doesn't exist, tell the user and stop.

4. **Checkout the tag** in the submodule:
   ```
   cd code/cuisineiq-rust && git checkout chapter-XX
   ```

5. **Identify files to extract.** Read the chapter scaffold or book plan to determine
   which files are relevant. Look for `{{#include ../listings/chXX/` directives in the
   chapter markdown to know exactly what's needed.

6. **Copy files to listings.** Create `listings/chXX/` and copy the relevant source
   files, preserving directory structure:
   ```
   mkdir -p listings/chXX/src/
   cp code/cuisineiq-rust/app/src/main.rs listings/chXX/src/main.rs
   ```

7. **Verify ANCHOR comments.** For each `{{#include}}` directive that specifies an
   anchor (e.g., `{{#include ../listings/ch02/src/main.rs:health_handler}}`), verify
   that the source file contains matching `// ANCHOR: health_handler` and
   `// ANCHOR_END: health_handler` comments. Report any missing anchors.

8. **Verify includes resolve.** Run `mdbook build` and check for errors related to
   missing includes. Report any that fail.

9. **Reset the submodule** back to its tracking branch:
   ```
   cd code/cuisineiq-rust && git checkout main
   ```

10. **Report.** Show:
    - Files extracted to `listings/chXX/`
    - Any missing anchors
    - Any unresolved `{{#include}}` directives
    - Suggested next step: draft the chapter prose referencing these listings

## Important Notes

- Never modify files in `code/cuisineiq-rust/` — it's a submodule.
- Listings are snapshots, not symlinks. The same file can exist in multiple
  `listings/chXX/` directories with different content (code evolves across chapters).
- ANCHOR comments in the source files are invisible to the compiled code but
  essential for the book's include directives.
- If a chapter references code that doesn't exist yet in cuisineiq-rust,
  that's a signal to go build the code first.
