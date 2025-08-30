# Kilo Code: File System Operations

This document provides specific guidelines for all file system operations to ensure that changes are made safely, efficiently, and correctly.

## 1. Reading Files (`read_file`)
- **Bulk Reading:** When multiple files are needed for context, I will read them all in a single `read_file` operation to minimize I/O and improve efficiency.
- **Verification:** I will always check the result of a `read_file` operation to ensure the file was read successfully before proceeding.

## 2. Writing and Creating Files (`write_to_file`)
- **Full Content Only:** When using `write_to_file`, I will always provide the *entire* file content. I will never use it for partial updates.
- **New Files:** `write_to_file` is the preferred tool for creating new files. I will ensure the path is correct and that the content is complete.
- **Overwrite with Caution:** I will only use `write_to_file` to overwrite an existing file when a complete rewrite is the most logical approach.

## 3. Surgical Edits (`apply_diff`)
- **Precision is Key:** The `SEARCH` block must be an exact match of the content in the file, including all whitespace and newlines.
- **`start_line` Verification:** I will double-check that the `start_line` parameter is accurate before sending the request.
- **Multiple Changes:** When making several related changes to a single file, I will use multiple `SEARCH`/`REPLACE` blocks within a single `apply_diff` call.

## 4. Listing Files (`list_files`)
- **Recursive with Purpose:** I will use the `recursive` parameter only when I need a deep understanding of a directory's structure. For a top-level view, I will omit it.
- **Targeted Listing:** I will specify a directory path to avoid listing the entire workspace unless necessary.

## 5. Path Management
- **Relative Paths:** All file paths will be relative to the workspace root.
- **No Assumptions:** I will not assume a file or directory exists. I will use `list_files` to verify its presence if there is any doubt.
