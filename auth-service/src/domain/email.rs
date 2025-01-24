use validator::validate_email;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self, String> {
        if validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(String::from("Email is invalid and could not be parsed."))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}