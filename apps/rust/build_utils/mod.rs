pub mod constants;
pub mod handler_template;
pub mod handler_updater;
pub mod mod_generator;
pub mod openapi_generator;
pub mod path_utils;
pub mod template_generator;

#[allow(dead_code)]
#[derive(Debug)]
pub struct BuildOperation {
    pub errors: Vec<String>,
}

#[allow(dead_code)]
impl Default for BuildOperation {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl BuildOperation {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
