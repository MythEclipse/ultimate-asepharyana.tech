use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashSet;

pub fn inject_schemas(
    content: &str,
    handler_module_path: &str,
    schemas: &mut HashSet<String>,
) -> Result<()> {
    let struct_regex = Regex::new(
        r"(?ms)#\[derive\([^)]*\)\]\s*pub struct (\w+)(Response|Request|Query|Input)?\s*",
    )
    .map_err(|e| anyhow!("Failed to create schema injection regex: {}", e))?;

    for cap in struct_regex.captures_iter(content) {
        let struct_name = &cap[1];
        let suffix = cap.get(2).map(|s| s.as_str()).unwrap_or("");

        // Skip response structs that we've already enhanced
        if suffix == "Response" && content.contains(&format!("{}Response {{", struct_name)) {
            continue;
        }

        // Register schema by its fully-qualified type path so we can import it in the generated mod.
        schemas.insert(format!("{}::{}", handler_module_path, struct_name));
    }

    Ok(())
}
