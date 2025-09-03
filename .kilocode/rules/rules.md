# Kilo Code Custom Instructions for apply_diff

1. **Verifying `SEARCH` block accuracy:** `SEARCH` block content must meticulously compare with the actual file content, including all whitespace and indentation.
2. **Confirming `start_line` correctness:** The `start_line` specified for each diff block must precisely correspond to the beginning of the `SEARCH` content in the target file.
3. **Providing complete `REPLACE` content:** The `REPLACE` block must contain the complete, intended content for the modified section, not just the changes.
4. **Failure Threshold:** If `apply_diff` fails on a file, the next attempt to modify that file should use `write_to_file`. If `write_to_file` also fails for that file, no further modifications should be attempted on that specific file for the current task, as both primary methods have been exhausted. I will not assume failure and will only trust the latest build.
5. **Search and Replace Content Uniqueness:** The `SEARCH` block content must not be identical to the `REPLACE` block content.

## Kilo Code: Completion Rules

## Kilo Code: File System Operations

### 1. Reading Files (`read_file`)
- **Bulk Reading:** Read multiple files in a single `read_file` operation.
- **Verification:** Check the result of a `read_file` operation for success.

### 2. Writing and Creating Files (`write_to_file`)
- **Full Content Only:** Provide the *entire* file content when using `write_to_file`.
- **New Files:** `write_to_file` is the preferred tool for creating new files.
- **Overwrite with Caution:** Use `write_to_file` to overwrite an existing file only when a complete rewrite is required.
- **Post-Success Strategy:** After a successful `write_to_file` operation, the next tool use should attempt to be `apply_diff` again for subsequent modifications.

### 3. Surgical Edits (`apply_diff`)
- **Precision is Key:** The `SEARCH` block must be an exact match, including whitespace and newlines.
- **`start_line` Verification:** The `start_line` parameter must be accurate.
- **Multiple Changes:** Use multiple `SEARCH`/`REPLACE` blocks within a single `apply_diff` for related changes.
- **Targeted Edits vs. Rewrites:** Use `apply_diff` for targeted modifications; use `write_to_file` for complete rewrites.

### 4. Listing Files (`list_files`)
- **Recursive with Purpose:** Use `recursive` for deep directory understanding; omit for top-level.
- **Targeted Listing:** Specify a directory path to avoid listing the entire workspace.

### 5. Path Management
- **Relative Paths:** All file paths will be relative to the workspace root.
- **No Assumptions:** Verify file or directory existence with `list_files`.
- **Codebase Knowledge:** Do not assume knowledge of the codebase; use `list_files`, `search_files`, and `list_code_definition_names` to understand it.

## Kilo Code: General Principles

### 1. Clarity and Precision
- **Code with Intent:** All code should be clear, readable, and self-documenting wherever possible. I will not add any comments in code; instead, I will manage tasks using the `update_todo_list` tool. If any comments are found, they should be removed.
- **Precise Tool Usage:** Use tools for their intended purpose with accurate parameters.

### 2. Proactive Context Gathering
- **Understand Before Acting:** Gather sufficient context using `list_files`, `search_files`, and `read_file`.
- **Minimize Assumptions:** Avoid assumptions; use available tools to find missing information.

### 3. Efficiency and Best Practices
- **Follow Conventions:** I will adhere to the established coding standards, linting rules, and architectural patterns of the project.
- **Optimize Operations:** Bundle related operations to maximize efficiency.
- **Clean Code:** Write functional, maintainable, scalable, and understandable code.

### 4. Safety and Verification
- **Non-Destructive First:** Favor non-destructive actions; verify system state before changes. I will not assume failure and will only trust the latest build.
- **Step-by-Step Confirmation:** Wait for explicit user confirmation after each tool use. I will not oversight.
- **Reversibility:** Ensure significant changes can be reverted.

### 5. Adherence to Custom Instructions
- **Prioritize and Follow:** Prioritize and strictly adhere to all custom instructions.
- **Continuous Improvement:** Continuously integrate and learn from new custom instructions.

## Kilo Code: Tool Use Guidelines

### 1. Iterative Process
- **Step-by-Step Execution:** Break down complex tasks into smaller steps.
- **Informed Decisions:** Subsequent tool use will be informed by previous results.

### 2. Confirmation and Verification
- **User Confirmation:** Wait for explicit user confirmation after each tool use.
- **No Conversational Filler:** Responses will be direct and technical, reflecting tool outcome.

### 3. Tool Selection and Parameters
- **Optimal Tool Choice:** Select the most appropriate tool for each sub-task.
- **Precise Parameters:** Provide accurate and complete required parameters.
- **Efficient Tool Chaining:** Consider the most efficient order for tool chaining.

### 4. Contextual Awareness
- **Leverage Environment Details:** Utilize `environment_details` for project context.
- **Codebase Analysis:** Use `list_files`, `search_files`, and `list_code_definition_names` to understand the codebase.

### 5. Output and Reporting
- **Clear Communication:** Responses will be direct, technical, and to the point.
- **Task Completion:** Use `attempt_completion` to present a concise and final result upon task completion.
- **Todo List Management:** Actively use `update_todo_list` to track progress.
