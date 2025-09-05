# Kilo Code Custom Instructions for apply_diff

These rules ensure accurate and reliable use of the `apply_diff` tool for code modifications.

## Core Rules

1. **Verifying `SEARCH` block accuracy:** Always compare the `SEARCH` block with the actual file content, including all whitespace and indentation. Use the `read_file` tool to confirm before applying changes.

2. **Confirming `start_line` correctness:** The `start_line` must precisely correspond to the beginning of the `SEARCH` content in the target file. Double-check line numbers to avoid misalignment.

3. **Providing complete `REPLACE` content:** The `REPLACE` block must contain the complete, intended content for the modified section, not just the changes. Partial replacements can lead to incomplete code.

4. **Failure Threshold:** If `apply_diff` fails, switch to `write_to_file` for the next attempt. If `write_to_file` also fails, stop modifications and seek user confirmation. Do not assume failure; rely on explicit user feedback.

5. **Search and Replace Content Uniqueness:** Ensure `SEARCH` block content is not identical to `REPLACE` block content to avoid no-op changes that waste resources.

6. **Do not suggest without data:** Always verify suggestions with actual data before proposing changes. Use tools like `read_file` or `search_files` for validation.

7. **Avoid cache clearing:** Do not clear cache (e.g., `cargo clean`) unless explicitly instructed by the user.

8. **Pre-modification verification:** Always read the target file using `read_file` before applying any diff to ensure context accuracy and prevent errors.

9. **Context preservation:** When making changes, preserve the surrounding context in the `SEARCH` block to uniquely identify the modification point.

10. **Incremental changes:** Prefer multiple small, targeted diffs over large, complex replacements to minimize risk of errors.

11. **Syntax and Linting Checks:** After applying changes, verify that the code compiles and passes linting checks to ensure no syntax errors were introduced.

12. **Multi-File Operations:** When making changes across multiple files, apply diffs one file at a time and confirm each success before proceeding.

13. **Regex Usage:** Use regex in SEARCH blocks only when necessary and ensure patterns are precise to avoid unintended matches.

14. **Documentation Updates:** Update any related documentation if changes affect interfaces or behaviors.
