use std::fmt::format;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberToken(String);

impl SubscriberToken {
    pub fn parse(s: String) -> Result<SubscriberToken, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_incomplete = s.graphemes(true).count() > 25 || s.graphemes(true).count() < 25;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || is_incomplete || contains_forbidden_characters {
            Err(format!("{} is not a valid token", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    fn token(length: usize) -> String {
        let mut rng = thread_rng();
        std::iter::repeat_with(|| rng.sample(Alphanumeric))
            .map(char::from)
            .take(length)
            .collect()
    }
    #[test]
    fn a_token_longer_than_25_character_is_invalid() {
        assert_err!(SubscriberToken::parse(token(26)));
    }
    #[test]
    fn a_token_shorter_than_25_character_is_invalid() {
        assert_err!(SubscriberToken::parse(token(24)));
    }
    #[test]
    fn a_whitespace_only_tokens_are_rejected() {
        assert_err!(SubscriberToken::parse(" ".to_string()));
    }
    #[test]
    fn an_empty_string_token_is_rejected() {
        assert_err!(SubscriberToken::parse("".to_string()));
    }
    #[test]
    fn a_token_containing_invalid_character_is_rejected() {
        for token in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let token = token.to_string();
            assert_err!(SubscriberToken::parse(token));
        }
    }
    #[test]
    fn a_valid_token_parsed_successfully() {
        assert_ok!(SubscriberToken::parse(token(25)));
    }
}
