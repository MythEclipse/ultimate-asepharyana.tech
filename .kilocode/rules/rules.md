# Kilo Code Custom Instructions

## Core Rules

1. **Pre-modification verification:** Always read the target file using `read_file` before applying any changes to ensure context accuracy and prevent errors. Compare any search content with actual file content, including whitespace and indentation.

2. **Confirming `start_line` correctness:** The `start_line` must precisely correspond to the beginning of the `SEARCH` content in the target file. Double-check line numbers to avoid misalignment.

3. **Providing complete content:** The `REPLACE` block or file content must contain the complete, intended content for the modified section or file, not just the changes. Partial replacements can lead to incomplete code.

4. **Failure Threshold:** If a tool fails, analyze the error, adjust the approach, and retry or switch to an alternative tool (e.g., if `apply_diff` fails, use `write_to_file`). If all attempts fail, stop modifications and seek user confirmation. Do not assume failure; rely on explicit user feedback.

5. **Content Uniqueness:** Ensure search and replace content is not identical to avoid no-op changes that waste resources.

6. **Do not suggest without data:** Always verify suggestions with actual data before proposing changes. Use tools like `read_file` or `search_files` for validation.

7. **Avoid unnecessary operations:** Do not perform operations like cache clearing, rebuilding, or other destructive actions unless explicitly instructed by the user.

8. **Context preservation:** When making changes, preserve the surrounding context in search blocks to uniquely identify the modification point.

9. **Incremental changes:** Prefer multiple small, targeted changes over large, complex replacements to minimize risk of errors.

10. **Syntax and Linting Checks:** After applying changes, verify that the code compiles and passes linting checks to ensure no syntax errors or warnings are introduced.

11. **Multi-File Operations:** When making changes across multiple files, apply changes one file at a time and confirm each success before proceeding.

12. **Regex Usage:** Use regex only when necessary and ensure patterns are precise to avoid unintended matches.

13. **Documentation Updates:** Update any related documentation if changes affect interfaces or behaviors.

14. **Prohibit Comment Addition:** Do not add comments in the code to avoid unnecessary clutter.

15. **Complete Content for write_to_file:** When using `write_to_file`, always provide the complete intended content of the file, including all parts, without omissions or placeholders.

16. **Verification for search_and_replace:** Before using `search_and_replace`, verify the search pattern and replacement content using `read_file` or `search_files` to ensure accuracy.

17. **Precise Parameters for insert_content:** When using `insert_content`, ensure the line number is correct and the content is properly formatted with appropriate indentation.

18. **Tool Selection:** Choose the most appropriate tool for the task (e.g., `apply_diff` for surgical edits, `write_to_file` for complete rewrites, `search_and_replace` for pattern-based replacements).

19. **Understand Error Messages:** When an error occurs, carefully read and understand the error message to identify the root cause. Do not attempt fixes without comprehending the issue.

20. **Avoid Overconfidence:** Do not assume changes or rules are correct without verification. Always validate and test modifications.

21. **Skip Identical Edits:** Do not attempt to edit content that is already identical to the intended changes. If the edit would result in the same content, skip the operation to avoid unnecessary actions.

22. **Failure Retry Policy:** If `apply_diff` fails twice on the same edit attempt, switch to a full rewrite using `write_to_file` instead of continuing to retry the diff operation.
