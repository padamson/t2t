# Style Guide

Learned patterns from manuscript reviews. Read by `/pre-commit-review` at the start of every review.

## Attribution Rules

- **MinimumCD quotes** must be blockquoted or explicitly attributed. Known direct quotes:
  - "the delivery pipeline is feature zero"
  - "Every one of these is trivial to add to an empty project and expensive to retrofit into a mature codebase"
  - "Starting with CD is dramatically easier than migrating to it" — must attribute or rephrase, not echo with minor word swaps
- **ACD workflow stages** (the 11-stage workflow) should be framed as coming from the ACD framework, not presented as the author's original formulation
- **"Obey the Testing Goat"** pedagogical approach is acknowledged in the book plan but should be attributed when the parallel is drawn directly

## Recurring LLM Tells

- **Tripled parallel structure** ("Not X. Not Y. Z.") — appeared 3x in ch01 draft. Limit to at most 1 per chapter; vary the rhythm.
- **"Here's" sentence openers** — appeared 2x in ch01. Avoid as paragraph/section openers.
- **Technology bullet lists** where every item follows the same template (Bold name + does X + benefit) with no trade-offs or caveats. Vary the structure: some items lead with capability, others with the tool name. Add at least one honest caveat per list.
- **Grandiose section closers** ("These trends are converging, and this book sits at the intersection.") — cut or replace with something concrete.
- **"This matters because:"** signposting before a bullet list — the bullets speak for themselves.
- **"We're going to do something better"** — dramatic pivots that read as salesy.
- **"Ruthlessly"** and similar inflating adverbs — prefer plain language.
- **Em-dashes are an LLM signal.** Avoid them entirely. They have become so strongly associated with AI-generated text that even legitimate uses trigger reader suspicion. Replace with commas, parentheses, colons, semicolons, or separate sentences. Target: zero per chapter.
- **Short-short declarative pairs** ("That's not slow. That's thorough.") — fine once per chapter, becomes a tell when repeated.

## Voice Notes

- The architecture diagram section (dual-interface design) in ch01 reads well: direct, specific, no filler.
- The table mapping technology choices to CD constraints works better than prose for this kind of content.
- The "Problem with a Single Interface" subsection has a clear, natural flow: states a concrete problem, explains why it's a problem, moves on. Good template for future sections.
- The "Shift-Left Security: Two Layers" section reads well: concrete examples in each direction, no filler, good balance.
- The greenfield checklist with separate Development Environment and Security sections is a clean structure.
- The "Why This Architecture?" section reads naturally: states the problem, explains the design, shows the diagram, lists benefits. Good template for presenting design tradeoffs.
- The "Why Schema-Driven?" section reads well: states a concrete problem, provides cross-ecosystem context (Prisma, Smithy, SeaORM), then presents the solution. Good template for future "why this tool?" sections.
- The "Rust's toolchain isn't simpler" paragraph is an example of honest technical assessment: acknowledges trade-offs rather than overselling. Keep this tone for tool comparisons.
