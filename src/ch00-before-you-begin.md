# Before You Begin

This book teaches full-stack Rust development through the lens of Continuous Delivery. The practices we follow come from the MinimumCD manifesto and its Practice Guide, which covers greenfield project setup, testing strategy, pipeline design, and the Agentic Continuous Delivery (ACD) framework for AI-assisted development.

```admonish tip
You're about to clone a specific commit of the MinimumCD Practice Guide. Pinning to a commit ensures you see the exact content that this book was written against even if the live site changes later.
```

Rather than restate that material here, we'll reference it directly. You'll run a local copy of the MinimumCD Practice Guide alongside this book, reading specific sections as they become relevant and then applying what you've learned to our project, CuisineIQ.

## Set Up the MinimumCD Practice Guide

This book draws from two related projects:

- The **MinimumCD manifesto** defines Continuous Integration, Continuous Delivery, and Trunk-Based Development. It's a short, community-owned page. We'll link to it in the reading assignment at the end of this chapter.
- The **MinimumCD Practice Guide** is [Bryan Finster's](https://www.linkedin.com/in/bryan-finster/) comprehensive guide to CD practices: greenfield project setup, testing strategy, pipeline design, anti-patterns, and the Agentic Continuous Delivery (ACD) framework. This is the reference you'll use throughout the book. You're about to clone it and run it locally.

You'll run the Practice Guide locally so you have a pinned version that matches what this book was written against. Clone it and check out the pinned commit:

```bash
git clone https://github.com/bdfinst/cd-migration.git minimumcd-practice-guide
cd minimumcd-practice-guide
git checkout 1298d7a
```

The site requires Hugo (extended edition), Go, and Node.js. Install all three:

**macOS (Homebrew):**

```bash
brew install hugo go node
```

**Linux:** See the installation pages for [Hugo](https://gohugo.io/installation/linux/) (install the **extended** edition), [Go](https://go.dev/dl/), and [Node.js](https://nodejs.org/en/download/).

**Windows:** See the installation pages for [Hugo](https://gohugo.io/installation/windows/) (install the **extended** edition), [Go](https://go.dev/dl/), and [Node.js](https://nodejs.org/en/download/).

Then install the site's dependencies and start the local server:

```bash
npm install
npm start
```

The Practice Guide is now available at [http://localhost:1313](http://localhost:1313). Keep this running in a terminal while you work through the book.

**If you'd rather not run the site locally,** you can read the same content at [beyond.minimumcd.org](https://beyond.minimumcd.org). Be aware that the live site may have been updated since this book was written. The pinned commit ensures you see exactly the content the book references. If something doesn't match, check the pinned version referenced above.

## How We'll Use the Guide

Throughout this book, you'll see callouts like this:

```admonish reference title="Reading Assignment"
Open [localhost:1313/docs/migrate-to-cd/greenfield/](http://localhost:1313/docs/migrate-to-cd/greenfield/) and read the "Feature Zero" section. Then come back here.
```

These are not optional. The Practice Guide provides the *why* behind the practices we follow. This book provides the *how* and *what* as they apply to a specific Rust stack. Skipping the guide sections means building without understanding the reasoning, and that defeats the purpose.

Each chapter sends you to the Practice Guide for the theory, then brings you back to apply it to CuisineIQ.

## What You Need

**Required:**
- Basic Rust knowledge (you've read the first half of [*The Rust Programming Language*](https://doc.rust-lang.org/book/))
- A terminal and a text editor (VS Code recommended for devcontainer support)
- Git
- A GitHub account
- Podman (or Docker) installed locally

**Installed during the book:**
- The Rust toolchain (via `rustup`)
- `cargo-leptos`, `sqlx-cli`, `cargo-nextest`, `cargo-audit`, `cargo-deny`, `cargo-mutants`
- `panschema` for schema-driven code generation
- Hugo, Go, and Node.js (for the MinimumCD Practice Guide, installed above)

**Not required:**
- Prior web development experience
- Knowledge of HTML, CSS, or JavaScript
- Familiarity with CI/CD, containers, or Infrastructure as Code (all taught inline)

## First Reading Assignment

```admonish reference title="Reading Assignment"
Before starting Chapter 1, read these two pages. Read only the page linked below, not any sub-pages or navigation links.

1. **The MinimumCD Manifesto:** [minimumcd.org](https://minimumcd.org)
   This is a single short page. It defines Continuous Integration, Continuous Delivery, and Trunk-Based Development. These three concepts are the foundation of everything in this book.

2. **CD for Greenfield Projects:** [localhost:1313/docs/migrate-to-cd/greenfield/](http://localhost:1313/docs/migrate-to-cd/greenfield/)
   Read just this one page. It explains why starting with CD is easier than migrating to it, and introduces the "feature zero" concept: the delivery pipeline is the first thing you build.

For all reading assignments in this book, you are only expected to read the specific page linked. We'll direct you to each page as it becomes relevant.
```

Once you've read both, you're ready for Chapter 1.
