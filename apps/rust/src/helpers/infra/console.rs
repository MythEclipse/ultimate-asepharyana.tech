//! Console Commands / CLI Builder.
//!
//! Build artisan-like CLI commands.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::helpers::console::{Console, Command, CommandContext};
//!
//! let mut console = Console::new("myapp");
//! console.register("migrate", "Run migrations", |ctx| {
//!     ctx.info("Running migrations...");
//!     Ok(())
//! });
//! console.run();
//! ```

use std::collections::HashMap;
use std::io::{self, Write};

/// Console output colors.
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    White,
    Reset,
}

impl Color {
    pub fn code(&self) -> &'static str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Reset => "\x1b[0m",
        }
    }
}

/// Command context for execution.
pub struct CommandContext {
    pub args: Vec<String>,
    pub options: HashMap<String, String>,
}

impl CommandContext {
    pub fn new(args: Vec<String>) -> Self {
        let mut options = HashMap::new();
        let mut positional = Vec::new();

        for arg in args {
            if arg.starts_with("--") {
                let parts: Vec<&str> = arg[2..].splitn(2, '=').collect();
                options.insert(
                    parts[0].to_string(),
                    parts.get(1).unwrap_or(&"true").to_string(),
                );
            } else if arg.starts_with('-') {
                options.insert(arg[1..].to_string(), "true".to_string());
            } else {
                positional.push(arg);
            }
        }

        Self {
            args: positional,
            options,
        }
    }

    pub fn arg(&self, index: usize) -> Option<&str> {
        self.args.get(index).map(|s| s.as_str())
    }

    pub fn option(&self, name: &str) -> Option<&str> {
        self.options.get(name).map(|s| s.as_str())
    }

    pub fn has_option(&self, name: &str) -> bool {
        self.options.contains_key(name)
    }

    pub fn info(&self, message: &str) {
        println!(
            "{}[INFO]{} {}",
            Color::Blue.code(),
            Color::Reset.code(),
            message
        );
    }

    pub fn success(&self, message: &str) {
        println!(
            "{}[OK]{} {}",
            Color::Green.code(),
            Color::Reset.code(),
            message
        );
    }

    pub fn warning(&self, message: &str) {
        println!(
            "{}[WARN]{} {}",
            Color::Yellow.code(),
            Color::Reset.code(),
            message
        );
    }

    pub fn error(&self, message: &str) {
        eprintln!(
            "{}[ERROR]{} {}",
            Color::Red.code(),
            Color::Reset.code(),
            message
        );
    }

    pub fn line(&self, message: &str) {
        println!("{}", message);
    }

    pub fn confirm(&self, question: &str) -> bool {
        print!("{} [y/N]: ", question);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    }

    pub fn ask(&self, question: &str) -> String {
        print!("{}: ", question);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        input.trim().to_string()
    }

    pub fn table(&self, headers: &[&str], rows: &[Vec<String>]) {
        // Calculate column widths
        let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() && cell.len() > widths[i] {
                    widths[i] = cell.len();
                }
            }
        }

        // Print header
        let header_line: Vec<String> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:width$}", h, width = widths[i]))
            .collect();
        println!("| {} |", header_line.join(" | "));

        // Print separator
        let separator: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
        println!("|-{}-|", separator.join("-|-"));

        // Print rows
        for row in rows {
            let row_line: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, c)| format!("{:width$}", c, width = widths.get(i).copied().unwrap_or(0)))
                .collect();
            println!("| {} |", row_line.join(" | "));
        }
    }
}

/// Command handler type.
pub type CommandHandler = Box<dyn Fn(&CommandContext) -> Result<(), String> + Send + Sync>;

/// Command definition.
pub struct Command {
    pub name: String,
    pub description: String,
    pub handler: CommandHandler,
}

/// Console application.
pub struct Console {
    name: String,
    version: String,
    commands: HashMap<String, Command>,
}

impl Console {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            commands: HashMap::new(),
        }
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn register<F>(&mut self, name: &str, description: &str, handler: F)
    where
        F: Fn(&CommandContext) -> Result<(), String> + Send + Sync + 'static,
    {
        self.commands.insert(
            name.to_string(),
            Command {
                name: name.to_string(),
                description: description.to_string(),
                handler: Box::new(handler),
            },
        );
    }

    pub fn run(&self, args: Vec<String>) -> Result<(), String> {
        if args.is_empty() || args[0] == "help" || args[0] == "--help" {
            self.show_help();
            return Ok(());
        }

        let command_name = &args[0];
        let command_args = args[1..].to_vec();

        if let Some(command) = self.commands.get(command_name) {
            let ctx = CommandContext::new(command_args);
            (command.handler)(&ctx)
        } else {
            Err(format!(
                "Command '{}' not found. Run 'help' for available commands.",
                command_name
            ))
        }
    }

    fn show_help(&self) {
        println!(
            "{}{} v{}{}",
            Color::Cyan.code(),
            self.name,
            self.version,
            Color::Reset.code()
        );
        println!();
        println!(
            "{}Available commands:{}",
            Color::Yellow.code(),
            Color::Reset.code()
        );
        println!();

        for (name, cmd) in &self.commands {
            println!(
                "  {}{}{}  {}",
                Color::Green.code(),
                name,
                Color::Reset.code(),
                cmd.description
            );
        }
        println!();
    }
}
