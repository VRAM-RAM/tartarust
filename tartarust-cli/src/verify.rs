use crate::output::frontend_ouput::FrontendOutput;
use crate::output::message::MessageKind;
use crate::parameters::CliParams;
use ecow::string::{ToEcoString, EcoString};
use tartarust_lib::generation::{Pepper, Salt};
use tartarust_lib::hash::Hash;
use tartarust_lib::params::TartarusParams;


/// Helper function, called by `main()`, that modifies the `output` with the result of the verification.
/// 
/// First, we try to get the `pepper` from the environment variable. If we fail, we fill the output with an error, and exit the function.
/// After that, we try to decode the `salt`. If we fail, we fill the output with an error, and exit the function.
/// Then, we decode the `stored_digest`. If we fail, we fill the output with an error, and exit the function.
/// Finally, we load the params, and update `output` with the result of the verification.
#[allow(unused_assignments)]
pub fn verify_password(password: String, salt_hex: String, hash: String, output: &mut FrontendOutput) {
    let mut pepper = String::new(); // We create an empty string to store the pepper.

    match std::env::var("TARTARUS_PEPPER") { // We try to get the pepper value from the environment variable.
        Ok(value) => { pepper = value; },
        Err(e) => { 
            output.insert_error(e.to_eco_string(),EcoString::from("verify_password function"), EcoString::from("Please configure your environment variable by running : \n On MacOs/ Linux : export TARTARUS_PEPPER=\"Your_Ultra_Secure_Server_Secret_Key_Here\" \n On Windows : $env:TARTARUS_PEPPER=\"Your_Ultra_Secure_Server_Secret_Key_Here\""));
            return; // Because we had an error, we return and exit the function.
        }
    }

    let mut salt = Salt::empty(); //We create and preallocate an empty salt to store the future decoded salt.

    match Salt::from_hex(salt_hex) {
        Ok(value) => { salt = value; }, // We allocate the salt.

        Err(e) => {
            output.insert_error(e.to_eco_string(),EcoString::from("verify_password function"), EcoString::from("Please enter a valid hex salt : It must be 64 chars long !"));
            return; // Because we had an error, we return and exit the function.
        }
    }

    let stored_digest = match hex::decode(&hash) {
        Ok(value) => value,
        Err(e) => {
            output.insert_error(e.to_eco_string(),EcoString::from("verify_password function"), EcoString::from("Please enter a valid hex hash : It must be 128 chars long !"));
            return; // Because we had an error, we return and exit the function.
        }
    };

    if stored_digest.len() != 64 {
        output.insert_error(EcoString::from("The stored hash is not 64 bytes long."), EcoString::from("verify_password function"), EcoString::from("Please enter a valid hex hash : It must be 128 chars long !"));
        return; // Because the stored hash is invalid, we return and exit the function.
    }

    let stored = Hash::new(&stored_digest, salt);
    let params = CliParams::load();
    let params = TartarusParams { memory: params.memory, iterations: params.iterations, pepper: Pepper::from(pepper) };

    match stored.verify(password.as_bytes(), &params) {
        Ok(success) => {
            output.insert_message(MessageKind::Verify { success });
        },

        Err(e) => {
            output.insert_error(e.to_eco_string(), EcoString::from("verify_password function"), EcoString::from("Something went wrong with the hmac crate... please refeer to https://crates.io/crates/hmac."));
            return; // Because we had an error, we return and exit the function.
        }
    }
}
