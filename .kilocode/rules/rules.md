# Kilo Code Custom Instructions for apply_diff

To ensure precise and error-free modifications, Kilo Code adheres to the following guidelines when using the `apply_diff` tool:

1. **Verifying `SEARCH` block accuracy:** Strict comparison with actual file content, including whitespace and indentation. No assumptions.  
2. **Confirming `start_line` correctness:** Must match exactly with the `SEARCH` block start.  
3. **Providing complete `REPLACE` content:** Always full replacement, not partial edits.  
4. **Failure Handling:** If `apply_diff` fails once, switch to `write_to_file`.  
5. **Uniqueness:** `SEARCH` must not be identical to `REPLACE`.  

---

# Kilo Code: Completion Rules

- On task completion, use `attempt_completion` to provide a concise final result.  
- No filler, no assumptions, no speculative commentary.  

---

# Kilo Code: File System Operations

## 1. Reading Files (`read_file`)
- Read multiple files in one call when needed.  
- Verify results before acting.  

## 2. Writing and Creating Files (`write_to_file`)
- Always provide complete file content.  
- Use for new files or full rewrites only.  

## 3. Surgical Edits (`apply_diff`)
- `SEARCH` must match file content exactly.  
- Ensure correct `start_line`.  
- Use multiple blocks in one call when appropriate.  
- Prefer `apply_diff` for small edits, `write_to_file` for large rewrites.  

## 4. Listing Files (`list_files`)
- Use recursion only if necessary.  
- Target specific directories.  

## 5. Path Management
- Always use relative paths.  
- Verify existence with `list_files` if unsure.  

---

# Kilo Code: General Principles

## 1. Clarity and Precision
- Code must be clean and self-explanatory.  
- Only top-level comments allowed.  

## 2. Context Gathering
- Operate strictly based on actual code.  
- Never assume beyond verified content.  

## 3. Efficiency and Best Practices
- Follow coding standards and conventions.  
- Group related actions efficiently.  

## 4. Safety and Verification
- Verify before changes.  
- **No repetitive build/error/test checks.**  
- **Never assume cache issues.**  
- **Never delete or modify cache.**  

## 5. Custom Instructions
- Always prioritize and follow these instructions.  
- Work only with actual existing code.  

## 6. Debugging and Testing
- Only perform if significant changes occur.  
- Never propose speculative fixes.  

---

# Kilo Code: Tool Use Guidelines

## 1. Iterative Process
- Execute one tool action at a time.  
- Next steps based strictly on results.  

## 2. Confirmation and Verification
- Always wait for explicit confirmation.  
- Responses must be direct and technical.  

## 3. Tool Selection
- Choose the most appropriate tool for the task.  
- Provide accurate and complete parameters.  

## 4. Contextual Awareness
- Use environment details when relevant.  
- Analyze project structure with `list_files` and `search_files`.  

## 5. Output and Reporting
- Report directly without filler.  
- Use `attempt_completion` for final output.  
- Manage todos with `update_todo_list`.  
