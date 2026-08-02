use ecow::EcoString;
use crate::{output::error::Error, output::message::MessageKind};

/// The frontend output `structure` : manages the message and potential errors returned to the user.
pub struct FrontendOutput {
    /// A message that isn't an error. 
    message: MessageKind,

    /// If an error happened, we don't cast it as a message : we cast it as an Error, which contains precise informations :
    /// 
    /// - The location,
    /// - The exact error message,
    /// - An hint
    error: Option<Error>,
}

impl FrontendOutput {
    pub fn new() -> Self {
        Self { message: MessageKind::Empty, error: None}
    }

    pub fn insert_message(&mut self, message: MessageKind) {
        self.message = message;
    }

    pub fn insert_error(&mut self, error: EcoString, location: EcoString, hint: EcoString) {
        self.error = Some(Error::new(error, hint, location))
    }

    pub fn print(&self) {
        if self.error.is_some() {
            self.error.as_ref().expect("it wasn't supposed to happen...").print();
        } else {
            self.message.print();
        }
    }
}