pub mod anime;
pub mod auth;
pub mod komik;
pub mod proxy;
pub mod social;
pub mod types;

pub const API_BASE_URL: &str = if cfg!(debug_assertions) {
    "http://localhost:4091/api"
} else {
    "https://rust.asepharyana.tech/api"
};

pub const WS_BASE_URL: &str = if cfg!(debug_assertions) {
    "ws://localhost:4091"
} else {
    "wss://ws.asepharyana.tech"
};
