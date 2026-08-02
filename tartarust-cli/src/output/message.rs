use colored::Colorize;
use ecow::EcoString;

pub enum MessageKind {
    Hash {
        hashed: EcoString,
        salt: EcoString,
    },

    Verify {
        success: bool
    },

    ChangeParams {
        old_memory: u32,
        new_memory: u32,

        old_iter: u32,
        new_iter: u32,
    },

    ShowParams {
        memory: u32,
        iterations: u32,
    },

    Empty
}

impl MessageKind {
    pub fn print(&self) {
        match self {
            MessageKind::ChangeParams { old_memory, new_memory, old_iter, new_iter } => {
                println!("{}", "Parameters changed:".bold());
                println!(
                    "{} {} {} {}",
                    "Memory:".bold(),
                    format!("{old_memory} MB").red(),
                    "->".white(),
                    format!("{new_memory} MB").green(),
                );
                println!(
                    "{} {} {} {}",
                    "Iterations:".bold(),
                    format!("{old_iter}").red(),
                    "->".white(),
                    format!("{new_iter}").green(),
                );
            },

            MessageKind::ShowParams { memory, iterations } => {
                println!("{}", "Current parameters:".bold());
                println!("{} {}", "Memory:".bold(), format!("{memory} MB").cyan());
                println!("{} {}", "Iterations:".bold(), format!("{iterations}").cyan());
            },

            MessageKind::Empty => {
                unreachable!("MessageKind::Empty should never be printed");
            },

            MessageKind::Hash { hashed, salt } => {
                println!("{} {}", "Salt:".cyan().bold(), salt.cyan());
                println!("{} {}", "Hash:".green().bold(), hashed.green());
            },

            MessageKind::Verify { success } => {
                if *success {
                    println!("{}", "Success: the password is correct.".green().bold());
                } else {
                    println!("{}", "Failure: the password is incorrect.".red().bold());
                }
            }
        }
    }
}