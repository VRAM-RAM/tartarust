use ecow::EcoString;
use ecow::string::ToEcoString;
use tartarust_lib::generation::{Pepper, Salt};
use tartarust_lib::hash::{hash, hash_with_custom_salt};
use tartarust_lib::params::TartarusParams;

use crate::output::frontend_ouput::FrontendOutput;
use crate::output::message::MessageKind;
use crate::parameters::CliParams;

/// Helper function, called by `main()`, that modifies the `output` with the result of the hashing.
/// 
/// First, we try to get the `pepper` from the environment variable. If we fail, we fill the output with an error, and exit the function.
/// Then, we load the parameters, and try hashing the provided `password` with the parameters. If we fail, we fill the output with an error, and exit the function.
/// If we succeed, we return the result of the `hashing` (salt + password).
#[allow(unused_assignments)]
pub fn hash_password(password: String, output: &mut FrontendOutput) {
    let mut pepper = String::new(); // We create an empty string to store the pepper.

    match std::env::var("TARTARUS_PEPPER") { // We try to get the pepper value from the environment variable.
        Ok(value) => { pepper = value; },
        Err(e) => { 
            output.insert_error(e.to_eco_string(),EcoString::from("hash_password function"), EcoString::from("Please configure your environment variable by running : \n On MacOs/ Linux : export TARTARUS_PEPPER=\"Your_Ultra_Secure_Server_Secret_Key_Here\" \n On Windows : $env:TARTARUS_PEPPER=\"Your_Ultra_Secure_Server_Secret_Key_Here\""));
            return; // Because we had an error, we return and exit the function.
        }
    }

    let params = CliParams::load();

    match hash(&password.into_bytes(), &TartarusParams { memory: params.memory, iterations: params.iterations, pepper: Pepper::from(pepper) }) { // We try to hash the password.
        Ok(hash) => {
            let (hex_digest, hex_salt) = hash.encode_hex();
            output.insert_message(MessageKind::Hash { hashed: hex_digest.to_eco_string(), salt: hex_salt.to_eco_string() });
        },

        Err(e) => {
            output.insert_error(e.to_eco_string(),EcoString::from("hash_password function"), EcoString::from("Something went wrong with the hmac crate... please refeer to https://crates.io/crates/hmac."));
            return;
        }
    }
}

/// Helper function, called by `main()`, that modifies the `output` with the result of the hashing, but with a provided salt this time.
/// 
/// First, we try to get the `pepper` from the environment variable. If we fail, we fill the output with an error, and exit the function.
/// After that, we try to decode the `salt`. If we fail, we fill the output with an error, and exit the function.
/// Then, we load the parameters, and try hashing the provided `password` with the parameters. If we fail, we fill the output with an error, and exit the function.
/// If we succeed, we return the result of the `hashing` (salt + password).
#[allow(unused_assignments)]
pub fn hash_password_salt(password: String, salt_hex: String, output: &mut FrontendOutput) {
    let mut pepper = String::new(); // We create an empty string to store the pepper.

    match std::env::var("TARTARUS_PEPPER") { // We try to get the pepper value from the environment variable.
        Ok(value) => { pepper = value; },
        Err(e) => { 
            output.insert_error(e.to_eco_string(),EcoString::from("hash_password_salt function"), EcoString::from("Please configure your environment variable by running : \n On MacOs/ Linux : export TARTARUS_PEPPER=\"Your_Ultra_Secure_Server_Secret_Key_Here\" \n On Windows : $env:TARTARUS_PEPPER=\"Your_Ultra_Secure_Server_Secret_Key_Here\""));
            return; // Because we had an error, we return and exit the function.
        }
    }

    
    let mut salt = Salt::empty(); //We create and preallocate an empty salt to store the future decoded salt.

    match Salt::from_hex(salt_hex) {
        Ok(value) => { salt = value; }, // We allocate the salt.

        Err(e) => {
            output.insert_error(e.to_eco_string(),EcoString::from("hash_password_salt function"), EcoString::from("Please enter a valid hex salt : It must be 32 chars long !"));
            return; // Because we had an error, we return and exit the function.
        }
    }

    let params = CliParams::load();

    match hash_with_custom_salt(&password.into_bytes(), salt, &TartarusParams { memory: params.memory, iterations: params.iterations, pepper: Pepper::from(pepper) }) {
        Ok(hash) => {
            let (hex_digest, hex_salt) = hash.encode_hex();
            output.insert_message(MessageKind::Hash { hashed: hex_digest.to_eco_string(), salt: hex_salt.to_eco_string() });
        },

        Err(e) => {
            output.insert_error(e.to_eco_string(),EcoString::from("hash_password_salt function"), EcoString::from("Something went wrong with the hmac crate... please refeer to https://crates.io/crates/hmac."));
            return;
        }
    }
}