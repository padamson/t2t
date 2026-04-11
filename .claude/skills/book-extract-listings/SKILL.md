---
name: book-extract-listings
description: Verify ANCHOR comments in app/src/ match {{#include}} directives in chapter markdown
user-invocable: true
disable-model-invocation: true
argument-hint: "<chapter-number>"
---

# Verify Code Listings

Verify that ANCHOR comments in `app/src/` match the `{{#include}}` directives
in chapter markdown. In this monorepo, chapters include code directly from
`app/src/` via relative paths. This skill checks that every include resolves
and every referenced anchor exists.

## Process

1. **Parse the chapter number** from the first argument (e.g., "02" from `$ARGUMENTS`).

2. **Find all `{{#include}}` directives** in the chapter file:
   ```
   grep '{{#include' book/src/chXX-*.md
   ```

3. **For each directive**, verify:
   - The referenced file exists (resolve the relative path from `book/src/`)
   - If an anchor is specified (e.g., `:add_item`), verify that matching
     `// ANCHOR: add_item` and `// ANCHOR_END: add_item` comments exist
     in the source file

4. **Report** any:
   - Missing files (include path doesn't resolve)
   - Missing anchors (anchor name not found in the source file)
   - Orphaned anchors (anchors in app/src/ that no chapter references)

5. **Verify the build.** Run `cd book && mdbook build` and check for errors
   related to missing includes.

## Important Notes

- Includes use relative paths from `book/src/`: `{{#include ../../app/src/file.rs:anchor}}`
- ANCHOR comments in source files are invisible to the compiled code
- If a chapter references code that doesn't exist yet in `app/`,
  that's a signal to build the code first
- Run this after modifying app code that chapters reference
