use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<','>','\\','{','}'];
        let containts_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || is_too_long || containts_forbidden_characters {
            panic!("Invalid subscriber name: {}", s);
        } else {
            Self(s)
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

