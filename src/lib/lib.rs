use std::fmt::Display;

mod lemmatise;
mod markdown;
mod tokeniser;

pub struct NonEmpty<'a>(&'a str);

impl<'a> NonEmpty<'a> {
    pub fn new(input: &'a str) -> Option<NonEmpty<'a>> {
        if input.is_empty() {
            None
        } else {
            Some(NonEmpty(input))
        }
    }
}

impl<'a> Display for NonEmpty<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> NonEmpty<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}
