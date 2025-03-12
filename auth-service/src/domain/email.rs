use validator::validate_email;
use color_eyre::eyre::{eyre, Result};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self> {
        if validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(eyre!("Email is invalid and could not be parsed."))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}