use clap::{Parser, Subcommand};

/// The `Cli` structure, that define the command-line tool. All the commands are in `Commands`.
#[derive(Parser)]
#[command(name = "tartarus")]
#[command(author = "Tartarus' contributors")]
#[command(version = "1.0")]
#[command(about = "Experimental hashing algorithm.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}


/// The `subcommands` of tartarus-cli. We have :
/// 
/// - Hash : hashes a password, with the password and optionally the salt as arguments.
/// 
/// - Verify : takes a password, the stored hash and the stored salt as arguments, and verifies if the password is correct or not.
/// 
/// - Params : Changes the saved parameters (memory usage, number of iterations).
#[derive(Subcommand)]
pub enum Commands {
    Hash {
        /// Password to hash
        #[arg(short = 'p', long = "password")]
        password: String,

        /// The `salt` the value of the salt will determine which function will be used :
        /// 
        /// `salt == None -> hash();`
        /// 
        /// `salt == Some(String) == hash_with_custom_salt();`
        /// 
        /// So, if you don't provide salt, it will generate one randomly.
        #[arg(short = 's', long = "salt")]
        salt: Option<String>,
    },

    Verify {
        /// Password to verify
        #[arg(short = 'p', long = "password")]
        password: String,

        /// The `salt` of the stored hash.
        #[arg(short = 's', long = "salt")]
        salt: String,

        /// The stored `hash`.
        #[arg(short = 'H', long = "hash")]
        hash: String
    },

    
    Params {
        /// Changes the memory usage
        #[arg(short = 'm', long = "memory", value_name = "MEM")]
        memory: Option<u32>,

        /// Changes the number of iterations
        #[arg(long = "iterations", visible_alias = "it", value_name = "ITER")]
        iterations: Option<u32>,
    }
}