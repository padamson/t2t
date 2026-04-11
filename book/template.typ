// Trunk to Table — custom typst template for print-quality PDF
//
// Uses open-source fonts (installed via brew on macOS, apt on Linux/CI).
// macOS system fonts listed as fallbacks for environments without brew fonts.
// - Body: Charis SIL (open) → Charter (macOS)
// - Headings: Inter (open) → Avenir Next (macOS)
// - Code: JetBrains Mono (open) → Menlo (macOS)

#set text(
  lang: "en",
  font: ("Charis SIL", "Charter"),
  size: 11pt,
)

#set par(
  leading: 0.65em,
  first-line-indent: 0pt,
  justify: true,
)

#show heading: set text(font: ("Inter", "Avenir Next"))

#show link: underline

#show raw: set text(font: ("JetBrains Mono", "Menlo"), size: 9pt)

#show raw.where(block: true): block.with(
  width: 100%,
  fill: luma(245),
  inset: 10pt,
  radius: 4pt,
)

#show quote.where(block: true): block.with(
  width: 100%,
  fill: rgb("#f1f6f9"),
  inset: 10pt,
  radius: 4pt,
)

#set page(
  margin: (top: 2.5cm, bottom: 2.5cm, left: 2.5cm, right: 2.5cm),
  header: context {
    if counter(page).get().first() > 2 [
      #set text(size: 9pt, fill: luma(120))
      MDBOOK_TYPST_PDF_TITLE
    ]
  },
  footer: context {
    if counter(page).get().first() > 1 [
      #align(center)[
        #set text(size: 9pt)
        #counter(page).display()
      ]
    ]
  },
)

// Title page
#v(4cm)
#align(center)[
  #text(size: 28pt, weight: "bold", font: ("Inter", "Avenir Next"))[
    MDBOOK_TYPST_PDF_TITLE
  ]
  #v(1cm)
  #text(size: 14pt, fill: luma(100))[
    Building a Full-Stack Rust Application with Agentic Continuous Delivery
  ]
  #v(2cm)
  #text(size: 12pt)[Paul Adamson]
]

#pagebreak()

// Table of contents
#text(size: 18pt, weight: "bold", font: ("Inter", "Avenir Next"))[Table of Contents]
#v(0.5cm)
#outline(depth: 2, indent: 1em, title: none)

#pagebreak()

/**** MDBOOK_TYPST_PDF_PLACEHOLDER ****/
