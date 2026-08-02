use colored::Colorize;
use ecow::EcoString;

/// The `Error` structure : an helper to display the errors.
pub struct Error {
    /// The error message.
    pub message: EcoString,
    /// An hint
    pub hint: EcoString,
    /// The location of the error.
    pub location: EcoString,
}

impl Error {
    pub fn new(message: EcoString, hint: EcoString, location: EcoString) -> Self {
        Self { message, hint, location }
    }

    /// Helper that prints the error.
    pub fn print(&self) {
        println!("{}", "[-]".red().bold());
        println!("{} {}", "Error:".red().bold(), self.message.red().bold());
        println!("{} {}", "Hint:".yellow(), self.hint.yellow());
        println!("{} {}", "At:".cyan().bold(), self.location.cyan());
    }
}