# Kilo Code Custom Instructions for apply_diff

1. **Verifying `SEARCH` block accuracy:** Compare with the actual file content, including all whitespace and indentation.
2. **Confirming `start_line` correctness:** Must precisely correspond to the beginning of the `SEARCH` content in the target file.
3. **Providing complete `REPLACE` content:** Must contain the complete, intended content for the modified section, not just the changes.
4. **Failure Threshold:** If `apply_diff` fails, the next attempt to modify that file should use `write_to_file`. If `write_to_file` also fails, no further modifications should be attempted. I will not assume failure and will only trust the latest build.
5. **Search and Replace Content Uniqueness:** `SEARCH` block content must not be identical to `REPLACE` block content.
6. **Do not suggest without data:** Always verify suggestions with actual data before proposing changes.
7. **Never assume wrong cache:** Do not assume cache issues without evidence.
