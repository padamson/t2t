---
name: blocker
description: Log a dogfooding blocker — works for both author-maintained and third-party tools. Forks, fixes, contributes upstream.
user-invocable: true
disable-model-invocation: true
argument-hint: "<tool-name> <description of what's missing>"
---

# Log a Dogfooding Blocker

You are helping an author who is simultaneously writing a book (Trunk to Theory)
and developing several tools it depends on. When the book, app,
or toolchain needs a feature or fix that doesn't exist yet, this skill
tracks the blocker and sets up the contribution path.

There are three categories of blocker:

1. **Author-maintained tools** — the author owns the repo and can merge fixes directly.
2. **Third-party tools** — the author forks, fixes, uses the fork, and contributes
   the fix upstream via PR. The fork is used until upstream merges.
3. **New tools** — no existing tool solves the problem; create a new repo from
   `padamson/rust-project-template` and build the tool from scratch.

IMPORTANT: Public issues and PRs in any repo must stand on their own as generic
requests. Do not reference "Trunk to Theory", chapter numbers, or the app
in public repos. Book-specific context goes only in the t2t tracking issue.

## Known tools

### Author-maintained
- `panschema` -> `padamson/panschema` (schema-driven development)
- `playwright-rust` -> `padamson/playwright-rust` (E2E testing)
- `theoria` -> `padamson/theoria` (component explorer for Leptos)
- `dokime` -> `padamson/dokime` (component testing for Leptos)

### Third-party (fork when blocked)
- `mdbook-admonish` -> upstream: `tommilligan/mdbook-admonish`, fork: `padamson/mdbook-admonish`
- `mdbook-quiz` -> upstream: `cognitive-engineering-lab/mdbook-quiz`, fork: `padamson/mdbook-quiz`
- `mdbook-typst-pdf` -> upstream: `KaiserY/mdbook-typst-pdf`, fork: `padamson/mdbook-typst-pdf`
- Any other third-party tool -> determine upstream and fork repos from context

If the tool isn't in either list, ask the user for the GitHub repo path.

## Process

1. **Identify the tool and category.** The first argument should be the tool name.
   If not provided or not recognized, ask. Determine if it's author-maintained or
   third-party using the lists above.

2. **Gather context.** Determine:
   - What chapter or feature triggered the need
   - What the tool should do but can't yet (the missing capability)
   - What workaround, if any, is currently in place
   - Whether an existing upstream issue or PR already addresses this
   - Acceptance criteria: how would you know the feature is done?

   Read relevant files in the t2t and code/app/ repos to build context.
   If the user provided a description as $ARGUMENTS, use that as the starting point
   but ask clarifying questions if needed.

3. **Check for existing upstream work.** Search the upstream repo for related issues and PRs:
   - `gh issue list -R <upstream/repo> --search "<keywords>" --state all`
   - `gh pr list -R <upstream/repo> --search "<keywords>" --state all`

   If an open PR exists that fixes the problem, note it — the plan may involve
   finishing or testing that PR rather than starting from scratch.

4. **Enter plan mode with the proposed actions.** Use the EnterPlanMode tool to present
   the plan for the user to review. The plan differs by category:

   ### For author-maintained tools:

   ```
   # Blocker: [tool-name] — [description]

   ## Category: Author-maintained

   ## Public Issue (to be created in <owner/repo>)
   **Title:** [title]
   **Labels:** enhancement
   **Body:**
   [full issue body as it will appear]

   ## Tracking Issue (to be created in padamson/t2t)
   **Title:** [blocker:<tool-name>] [description]
   **Labels:** dogfood-blocker
   **Body:**
   [full issue body with book context]

   ## dogfood-gaps.md Entry
   [the entry that will be appended]
   ```

   ### For new tools (no existing tool solves the problem):

   ```
   # Blocker: [tool-name] — [description]

   ## Category: New tool

   ## New repo
   **Name:** padamson/<tool-name>
   **Template:** padamson/rust-project-template
   **Create with:** gh repo create padamson/<tool-name> --template padamson/rust-project-template --public --clone

   ## Initial Issue(s) (to be created in the new repo)
   **Title:** [title of the primary feature]
   **Body:**
   [design doc or requirements as they will appear in the new repo]

   ## Tracking Issue (to be created in padamson/t2t)
   **Title:** [blocker:<tool-name>] [description]
   **Labels:** dogfood-blocker
   **Body:**
   [full issue body with book context, link to new repo, acceptance criteria]

   ## dogfood-gaps.md Entry
   [the entry that will be appended]

   ## Post-template setup
   [Checklist items from SETUP.md in the template: Cargo.toml customization,
   GitHub settings, crates.io token, etc.]
   ```

   ### For third-party tools:

   ```
   # Blocker: [tool-name] — [description]

   ## Category: Third-party
   **Upstream repo:** <upstream-owner/repo>
   **Fork repo:** padamson/<repo> (create if needed)
   **Existing upstream PR:** <upstream-owner/repo>#<number> or "None"

   ## Approach
   [One of:
    - "Fork, fix, use fork, contribute PR upstream"
    - "Finish existing upstream PR #N in fork, use fork, contribute upstream"
    - "Test existing upstream PR #N, report results, use PR branch as fork"]

   ## Upstream Issue/PR (create or comment on existing)
   **Title:** [title]
   **Body:**
   [issue body or comment on existing PR]

   ## Tracking Issue (to be created in padamson/t2t)
   **Title:** [blocker:<tool-name>] [description]
   **Labels:** dogfood-blocker
   **Body:**
   [full issue body with book context, fork details, upstream PR link]

   ## dogfood-gaps.md Entry
   [the entry that will be appended]

   ## book.toml / Cargo.toml Changes
   [What config changes are needed to use the fork until upstream merges]
   ```

   After the user reviews and approves (possibly with edits), exit plan mode and
   proceed with the approved actions.

5. **For author-maintained tools — create the public issue:**
   Use `gh issue create -R <owner/repo>` with:
   - Title: concise description of the missing feature
   - Labels: `enhancement`
   - Body: a clean, generic feature request with NO book references:
     ```
     ## Feature request

     [Description of the capability needed]

     ## Use case

     [Generic description — frame as a general user need, not a book need]

     ## Acceptance criteria

     - [ ] [Specific, testable criteria]
     ```

6. **For new tools — create the repo from the template:**

   a. **Create the repo from the template:**
      ```bash
      gh repo create padamson/<tool-name> \
        --template padamson/rust-project-template \
        --description "<short description>" \
        --public --clone
      ```
      Clone target should be `~/src/github-padamson/<tool-name>/` (sibling to t2t).
      If `gh` clones it elsewhere, move it.

   b. **Customize the scaffold:**
      - Update `Cargo.toml`: name, description, repository, authors, categories, keywords
      - Update `README.md` with tool description and usage
      - Update `CLAUDE.md` with tool-specific context

   c. **Configure GitHub settings** (follow SETUP.md in the new repo):
      - Enable code security features (Dependabot, code scanning, secret scanning, push protection, private vulnerability reporting, malware alerts)
      - Set branch protection on main requiring status checks
      - Add `CARGO_REGISTRY_TOKEN` secret if publishing to crates.io

   d. **Create initial design/requirements issue(s)** in the new repo with acceptance criteria
      and any design documents the user approved in plan mode.

   e. **Push initial changes** and verify CI passes.

7. **For third-party tools — set up the fork and contribution path:**

   a. **Fork the upstream repo** if `padamson/<repo>` doesn't exist:
      `gh repo fork <upstream/repo> --clone=false`

   b. **If an existing upstream PR addresses the issue:**
      - Note the PR number and branch
      - The user will work from that PR's changes in their fork
      - Comment on the upstream PR offering to help test or finish the work
        (no book references)

   c. **If no existing PR exists:**
      - Create an issue in the upstream repo (generic, no book references)
      - The user will create the fix in their fork and open a PR upstream

   d. **Configure the project to use the fork** until upstream merges:
      - For mdbook plugins: update `book.toml` or document the `cargo install --git` command
      - For Cargo dependencies: use `[patch]` or git dependency in `Cargo.toml`
      - Add a comment in the config noting the upstream PR to track

8. **Create the tracking issue in t2t.** Use `gh issue create -R padamson/t2t` with:
   - Title: `[blocker:<tool-name>] <description>`
   - Labels: `dogfood-blocker`
   - Body:
     ```
     ## Blocked by

     <owner/repo>#<number> — [issue/PR title]

     ## Category

     [Author-maintained | Third-party (forked) | New tool]

     ## Book context

     - **Chapter:** [chapter number and name]
     - **What we were building:** [feature or section]
     - **Related code:** [file paths in app/ or t2t]

     ## Fork details (third-party only)

     - **Fork repo:** padamson/<repo>
     - **Upstream PR:** <upstream/repo>#<number> or "Will create"
     - **Using fork via:** [how the fork is referenced — git dep, cargo install, etc.]

     ## Workaround

     [What's being done instead, or "Using forked version"]

     ## Resume point

     [What to do when upstream merges — remove fork reference, update to released version]
     ```

9. **Record in dogfood-gaps.md.** Append an entry to `dogfood-gaps.md` in the t2t repo root:
   ```
   ## [date] — [tool-name]: [issue title]
   - **Category:** [Author-maintained | Third-party | New tool]
   - **Repo:** padamson/<tool-name> (for new tools and author-maintained)
   - **Upstream:** <upstream/repo>#<number> (for third-party)
   - **Fork:** padamson/<repo> (if third-party)
   - **Upstream PR:** <upstream/repo>#<number> (if exists)
   - **Tracking issue:** padamson/t2t#<number>
   - **Chapter:** [chapter]
   - **Status:** Open
   - **Workaround:** [description, "Using fork", or "None — tool in development"]
   ```
   Create the file if it doesn't exist. Add a header `# Dogfood Gaps` if creating new.

10. **Report back.** Show the user:
    - All issue/PR URLs (upstream and t2t)
    - Fork URL if created, or new repo URL if new tool
    - Config changes needed to use the fork (third-party) or consume the new tool
    - Summary of what was recorded
    - For third-party: "When upstream merges the PR, run `/resume` to switch back
      to the released version."
    - For author-maintained: "When the issue is resolved, run `/resume` to pick up
      where you left off."
    - For new tools: "When the tool is usable, run `/resume` to integrate it
      into the book workflow."
