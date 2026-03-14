---
name: resume
description: Show outstanding dogfood blockers and help pick up where you left off
user-invocable: true
disable-model-invocation: true
argument-hint: "[optional: issue number or repo name to filter]"
---

# Resume After a Blocker

You are helping an author resume book/app work after a dogfooding blocker.
Blockers can be in author-maintained tools (panschema, playwright-rust, etc.)
or third-party tools (mdbook-admonish, mdbook-quiz, etc.) where the author
has forked and contributed upstream.

## Process

1. **Read the current state.** Read `dogfood-gaps.md` in the t2t repo root. If it doesn't
   exist, tell the user there are no recorded blockers and suggest `/blocker` if they
   need to log one.

2. **Check blocker status.** For each Open entry in dogfood-gaps.md:

   **For author-maintained tools:**
   - Run `gh issue view <number> -R padamson/<repo>` to check if the public issue is closed
   - Run `gh issue view <number> -R padamson/t2t` to read the full book context and resume point
   - Categorize as: Resolved (issue closed), Still open, or Worked around

   **For third-party tools:**
   - Check the upstream PR status: `gh pr view <number> -R <upstream/repo>`
   - Check if a new upstream release includes the fix: `cargo search <crate-name>`
   - Run `gh issue view <number> -R padamson/t2t` for the tracking issue
   - Categorize as: Merged upstream (PR merged), Released (new version available),
     Still open (PR not merged), or Using fork (fork works, upstream pending)

3. **If the user passed an argument**, filter to just that issue number or repo.

4. **Present a status summary:**
   ```
   ## Dogfood Blocker Status

   ### Resolved (ready to resume)
   - panschema#12: "Generate enum types from LinkML" — CLOSED
     Resume: Chapter 3, pick up at service layer types (see t2t#5)

   ### Upstream merged (switch from fork to release)
   - mdbook-admonish: upstream PR tommilligan/mdbook-admonish#234 — MERGED
     New release: v1.21.0 available on crates.io
     Action: `cargo install mdbook-admonish`, remove fork reference from book.toml
     Resume: Re-enable admonitions in chapters (see t2t#8)

   ### Using fork (upstream pending)
   - mdbook-quiz: upstream PR cognitive-engineering-lab/mdbook-quiz#62 — OPEN
     Fork: padamson/mdbook-quiz (installed via cargo install --git)
     Action: None needed — fork is working. Check back later.

   ### Still open
   - playwright-rust#45: "Native select element support" — OPEN
     Workaround: using JS evaluation (ch05, e2e tests)

   ### Worked around (review needed)
   - panschema#8: "FromRow derive on generated types" — OPEN
     Workaround: hand-written FromRow impl in models/grocery_item.rs
   ```

5. **For resolved / merged / released blockers**, help the user resume:
   - Read the t2t tracking issue for the resume point
   - For author-maintained: check what's changed since the issue was filed
   - For third-party with upstream merged:
     - Check if a new crate version is published (`cargo search <name>`)
     - If released: help switch from fork to released version
       (update `cargo install` command, remove git dep / `[patch]` entries,
       update `book.toml` if applicable)
     - If merged but not released: note that the fork is still needed until release
   - Suggest what to do next based on the resume point
   - Offer to update dogfood-gaps.md to mark the entry as Resolved
   - Offer to close the t2t tracking issue

6. **For "worked around" items**, flag them for `/audit-dogfood` review —
   the workaround may have become invisible and the feature request forgotten.

7. **For "using fork" items**, check if the upstream PR needs attention:
   - Has the maintainer requested changes? Help address review feedback.
   - Has the PR gone stale? Suggest a polite ping or offering to help.
   - Is the fork diverging from upstream? Note if a rebase is needed.
