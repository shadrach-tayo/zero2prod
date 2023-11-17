use secrecy::{ExposeSecret, Secret};
#[derive(Debug)]
pub struct Password(Secret<String>);

impl Password {
    pub fn parse(s: String) -> Result<Password, String> {
        if s.len() > 12 && s.len() < 128 {
            Ok(Self(Secret::new(s)))
        } else {
            Err(
                "Password should be longer that 4 characters and less than 128 characters."
                    .to_string(),
            )
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug)]
pub struct ChangePasswordParam {
    pub current_password: Password,
    pub new_password: Password,
    pub new_password_check: Password,
}
