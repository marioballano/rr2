//! Shell - the REPL where dreams go to die

use colored::*;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RlResult};
use std::path::PathBuf;

use crate::binary::loader::BinaryInfo;
use crate::cli::commands::CommandHandler;
use crate::core::state::ShellState;

pub struct Shell {
    state: ShellState,
    handler: CommandHandler,
    editor: DefaultEditor,
}

impl Shell {
    pub fn new(file: Option<PathBuf>, write_mode: bool, debug: bool) -> Self {
        let binary_info = file.as_ref().and_then(|f| BinaryInfo::load(f).ok());
        
        let state = ShellState::new(file, binary_info, write_mode, debug);
        let handler = CommandHandler::new();
        let editor = DefaultEditor::new().expect("Failed to create editor (skill issue)");

        Shell {
            state,
            handler,
            editor,
        }
    }

    fn get_prompt(&self) -> String {
        let mode_indicator = if self.state.write_mode {
            "w".red().to_string()
        } else {
            "r".green().to_string()
        };

        let addr = format!("0x{:08x}", self.state.current_address);
        
        format!(
            "[{}:{}]> ",
            addr.cyan(),
            mode_indicator
        )
    }

    pub fn execute_command(&mut self, input: &str) {
        let input = input.trim();
        
        if input.is_empty() {
            return;
        }

        // Handle special single-character commands
        match input {
            "q" | "quit" | "exit" => {
                println!("{}", "Goodbye! May your binaries be bug-free (lol).".yellow());
                std::process::exit(0);
            }
            _ => {}
        }

        // Dispatch to command handler
        self.handler.handle(input, &mut self.state);
    }

    pub fn run(&mut self) -> RlResult<()> {
        loop {
            let prompt = self.get_prompt();
            
            match self.editor.readline(&prompt) {
                Ok(line) => {
                    let _ = self.editor.add_history_entry(line.as_str());
                    self.execute_command(&line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("{}", "^C - Use 'q' to quit (or keep mashing Ctrl+C, see if I care)".dimmed());
                }
                Err(ReadlineError::Eof) => {
                    println!("{}", "EOF detected. Fleeing the scene...".yellow());
                    break;
                }
                Err(err) => {
                    eprintln!("{}: {:?}", "Error".red(), err);
                    eprintln!("{}", "This is probably your fault somehow.".dimmed());
                }
            }
        }
        Ok(())
    }
}
