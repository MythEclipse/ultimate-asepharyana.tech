use std::fs;
use std::path::Path;
use anyhow::Result;
use crate::build_utils::template_generator::generate_template_content;

pub fn generate_handler_template(path: &Path, root_api_path: &Path) -> Result<()> {
    let template = generate_template_content(path, root_api_path)?;
    fs::write(path, template)?;
    println!("cargo:warning=Generated new handler template for {:?}", path);
    Ok(())
}

