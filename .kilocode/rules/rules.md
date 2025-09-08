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

23. **Mandatory SEARCH field:**

    - All search-based operations (`search_and_replace`, `apply_diff`) **must** include a `SEARCH` block.
    - If replacing the entire file, use `SEARCH: .*` and provide the complete file content in `REPLACE`.
    - Never leave `SEARCH` empty.

24. **Whitespace Sensitivity:**

    - `SEARCH` must exactly match the file content, including whitespace, newlines, and indentation.
    - If uncertain, first use `read_file` or `search_files` to extract the exact block.

25. **Atomicity of operations:** Each modification should be atomic and self-contained. Avoid combining unrelated changes in a single operation to simplify rollback if necessary.

26. **Preserve file encoding:** Always maintain the original file encoding (UTF-8, UTF-16, etc.) and line endings (LF/CRLF). Do not introduce inconsistencies.

27. **Backup critical files:** If editing configuration or schema files, create a backup copy before applying modifications to ensure recovery in case of failure.

28. **Test-driven changes:** For code edits, validate modifications against existing tests or create minimal tests to confirm correctness after changes.

29. **Minimal regex scope:** When regex is used in `SEARCH`, scope it narrowly to avoid capturing unintended parts of the file.

30. **Log meaningful failures:** If a modification fails, include the reason and relevant context (file name, line range, attempted `SEARCH`) in the error output for easier debugging.

31. **Avoid destructive replacements:** Never delete large sections of code unless explicitly instructed. Instead, comment on the potential impact and ask for user confirmation.

32. **Respect file permissions:** Ensure the file is writable before attempting modifications. Do not attempt to bypass file permission restrictions.

33. **Do not reorder unless instructed:** Preserve the original order of functions, imports, or configuration entries unless the user explicitly requests reordering.

34. **Consistency across files:** If making edits in multiple files (e.g., renaming a variable), ensure consistency of changes across all occurrences.

35. **Safety over speed:** Prioritize correctness, validation, and safety over making fast changes. Never sacrifice reliability for speed.
