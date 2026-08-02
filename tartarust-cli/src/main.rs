use crate::parameters::CliParams;
use crate::{cli::{Cli, Commands}, output::frontend_ouput::FrontendOutput};
use crate::output::message::MessageKind;
use clap::Parser;
use ecow::EcoString;

pub mod output;
pub mod hash;
pub mod verify;
pub mod cli;
pub mod parameters;

fn main() {
    let cli = Cli::parse();
    let mut output = FrontendOutput::new();
    match cli.command {
        Some(Commands::Hash { password, salt }) => {
            if salt.is_some() {
                hash::hash_password_salt(password, salt.unwrap(), &mut output);
            } else {
                hash::hash_password(password, &mut output);
            }
            output.print();
        }

        Some(Commands::Verify { password, salt, hash }) => {
            verify::verify_password(password, salt, hash, &mut output);
            output.print();
        }

        Some(Commands::Params { memory, iterations }) => {
            let current = CliParams::load();
            let updated = current.apply(memory, iterations);

            if memory.is_some() || iterations.is_some() {
                match updated.save() {
                    Ok(()) => output.insert_message(MessageKind::ChangeParams {
                        old_memory: current.memory,
                        new_memory: updated.memory,
                        old_iter: current.iterations,
                        new_iter: updated.iterations,
                    }),
                    Err(e) => output.insert_error(EcoString::from(e), EcoString::from("params function"), EcoString::from("The parameters file could not be saved.")),
                }
            } else {
                output.insert_message(MessageKind::ShowParams { memory: current.memory, iterations: current.iterations });
            }

            output.print();
        }

        None => todo!()
    }

    
}