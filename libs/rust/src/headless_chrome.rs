pub use chromiumoxide::browser::{ Browser as BrowserPool, BrowserConfig };
pub use chromiumoxide::page::Page;
pub use chromiumoxide::handler::Handler;
pub use chromiumoxide::page::Page as Tab; // Alias Page as Tab for compatibility
pub enum LogLevel {
  Info,
  Debug,
  Warn,
  Error,
}
