use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<','>','\\','{','}'];
        let containts_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || is_too_long || containts_forbidden_characters {
            Err(format!("{} is invalid subscriber name", s))
        } else {
            Ok(Self(s))
        }
    }

    // Consume the wrapper and return the inner String, useful when we want to
    // pass owned data to downstream APIs (e.g., sqlx) without using references.
    pub fn into_inner(self) -> String {
        self.0
    }
}

// Allow borrowing the inner string as &str (e.g., for SQLx bindings)
impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: String,
}

impl NewSubscriber {
    // Consume self and return owned primitives suitable for DB insertion
    // without needing references in the query bindings.
    pub fn into_parts(self) -> (String, String) {
        let email = self.email;
        let name = self.name.into_inner();
        (email, name)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }
    #[test]
    fn a_name_longer_than_256_graphemes_is_invalid() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".repeat(256);
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn empty_strings_are_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &["/", "(", ")", "<",">","\\","{","}"] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }
    #[test]
    fn a_valid_ame_is_parsed_successfully() {
        let name = "Ursula le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
