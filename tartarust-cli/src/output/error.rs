use colored::Colorize;
use ecow::EcoString;

pub struct Error {
    pub message: EcoString,
    pub hint: EcoString,
    pub location: EcoString,
}

impl Error {
    pub fn new(message: EcoString, hint: EcoString, location: EcoString) -> Self {
        Self { message, hint, location }
    }

    pub fn print(&self) {
        println!("{}", "[-]".red().bold());
        println!("{} {}", "Error:".red().bold(), self.message.red().bold());
        println!("{} {}", "Hint:".yellow(), self.hint.yellow());
        println!("{} {}", "At:".cyan().bold(), self.location.cyan());
    }
}