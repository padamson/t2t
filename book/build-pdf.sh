#!/usr/bin/env bash
# Build HTML + print-quality PDF via mdbook-typst-pdf
set -euo pipefail

MDBOOK_OUTPUT='{"html": {"additional-css": ["./mdbook-admonish.css"]}, "typst-pdf": {"pdf": true, "custom-template": "template.typ"}}' \
  mdbook build

echo "PDF: build/typst-pdf/Trunk to Theory.pdf"
