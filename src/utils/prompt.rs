use std::io::{self, Write};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use log::{info, error, warn};
use dirs::home_dir;

pub struct CommandHandler {
    commands: HashMap<String, Box<dyn Fn(&[&str])>>,
}

impl CommandHandler {
    /// Creates a new CommandHandler instance.
    pub fn new() -> Self {
        CommandHandler {
            commands: HashMap::new(),
        }
    }

    /// Registers a command with its associated function.
    pub fn register_command<F>(&mut self, command: &str, handler: F)
    where
        F: Fn(&[&str]) + 'static,
    {
        self.commands.insert(command.to_string(), Box::new(handler));
    }

    /// Executes a command by name with the provided arguments.
    pub fn execute(&self, input: &str) {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if let Some((command, args)) = parts.split_first() {
            if let Some(handler) = self.commands.get(*command) {
                handler(args);
            } else {
                warn!("Unknown command: {}", command);
            }
        }
    }
}

/// Helper function to list files in a directory.
pub fn list_files_in_dir(relative_path: &str) {
    if let Some(home) = home_dir() {
        let path = home.join(relative_path);
        if path.exists() && path.is_dir() {
            match fs::read_dir(&path) {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            info!("- {}", entry.file_name().to_string_lossy());
                        }
                    }
                }
                Err(e) => error!("Failed to read directory: {}", e),
            }
        } else {
            warn!("Directory not found: {}", path.display());
        }
    } else {
        error!("Failed to determine home directory.");
    }
}

pub fn start_prompt() {
    let mut handler = CommandHandler::new();

    // Register commands
    handler.register_command("help", |args| {
        if args.is_empty() {
            info!("Available commands: help, exit, run, show payloads, show exploits");
        } else {
            info!("Help for command: {}", args[0]);
        }
    });

    handler.register_command("exit", |_| {
        info!("Exiting...");
        std::process::exit(0);
    });

    handler.register_command("run", |args| {
        info!("Executing Python script: {:?}", args);
        // Here, you can integrate with PyRunner to actually run the script
    });

    handler.register_command("show payloads", |_| {
        info!("Listing all payloads in ~/.zek/payloads:");
        list_files_in_dir(".zek/payloads");
    });

    handler.register_command("show exploits", |_| {
        info!("Listing all exploits in ~/.zek/exploits:");
        list_files_in_dir(".zek/exploits");
    });
    handler.register_command("help", |_| {
        println!("This help msg");
    });

    info!("Welcome to the interactive prompt! Type 'help' for a list of commands.");

    loop {
        print!("prompt> "); // thig prompt msg should be changable
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            handler.execute(&input);
        } else {
            warn!("Failed to read input. Try again.");
        }
    }
}
