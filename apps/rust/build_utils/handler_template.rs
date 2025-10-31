use crate::build_utils::template_generator::generate_template_content;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate_handler_template(path: &Path, root_api_path: &Path) -> Result<()> {
    let template = generate_template_content(path, root_api_path, false)?;
    fs::write(path, template)?;
    println!(
        "cargo:warning=Generated new handler template for {:?}",
        path
    );
    Ok(())
}
