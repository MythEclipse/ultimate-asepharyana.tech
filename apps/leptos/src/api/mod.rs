pub mod anime;
pub mod auth;
pub mod komik;
pub mod social;
pub mod types;

pub const API_BASE_URL: &str = if cfg!(debug_assertions) {
    "http://localhost:4091/api"
} else {
    "https://rust.asepharyana.tech/api"
};
