#!/bin/bash
# Warns when committing changes to src/*.md without running /pre-commit-review.
# Also checks that {{#include}} directives in staged .md files resolve.

STAGED_MD=$(git diff --cached --name-only 2>/dev/null | grep "^book/src/.*\.md$" || true)

if [ -z "$STAGED_MD" ]; then
  exit 0
fi

WARNINGS=""

# Check if /pre-commit-review was likely run in this session
# (We can't know for sure, but we can remind)
WARNINGS="${WARNINGS}Manuscript files staged for commit: ${STAGED_MD}\n"
WARNINGS="${WARNINGS}Reminder: run /book-pre-commit-review before committing manuscript changes.\n"

# Check that {{#include}} directives resolve
for file in $STAGED_MD; do
  if [ -f "$file" ]; then
    while IFS= read -r line; do
      # Extract the include path
      include_path=$(echo "$line" | sed -n 's/.*{{#include \(.*\)}}.*/\1/p')
      if [ -n "$include_path" ]; then
        dir=$(dirname "$file")
        resolved="$dir/$include_path"
        # Strip any :anchor_name suffix
        resolved=$(echo "$resolved" | sed 's/:.*$//')
        if [ ! -f "$resolved" ]; then
          WARNINGS="${WARNINGS}WARNING: Unresolved include in $file: {{#include $include_path}}\n"
        fi
      fi
    done < <(grep '{{#include' "$file" || true)
  fi
done

if [ -n "$WARNINGS" ]; then
  echo -e "$WARNINGS" >&2
fi

exit 0
