use color_eyre::eyre::{eyre, Result};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Self> {
        if password.len() >= 8 {
            return Ok(Self(password));
        }

        Err(eyre!("Password is not valid."))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}