# Kilo Code Custom Instructions for apply_diff

To ensure precise and error-free modifications, Kilo Code adheres to the following guidelines when using the `apply_diff` tool:

1.  **Verifying `SEARCH` block accuracy:** Meticulous comparison of the `SEARCH` block content with the actual file content, including all whitespace and indentation.
2.  **Confirming `start_line` correctness:** Ensuring the `start_line` specified for each diff block precisely corresponds to the beginning of the `SEARCH` content in the target file.
3.  **Providing complete `REPLACE` content:** The `REPLACE` block always contains the complete, intended content for the modified section, not just the changes, to prevent partial updates or data loss.
4.  **Failure Threshold:** If `apply_diff` fails twice on the same file, switch to `write_to_file` for that file to ensure successful modification.
