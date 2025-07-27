use std::fmt::Display;

mod lemmatise;
mod markdown;
mod tokeniser;

use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmpty<S: AsRef<str>>(S);

impl<S: AsRef<str>> NonEmpty<S> {
    /// Constructs a new `NonEmpty`, returning `None` if the string is empty.
    pub fn new(input: S) -> Option<Self> {
        (!input.as_ref().is_empty()).then(|| Self(input))
    }

    /// Returns the string slice view of the underlying data.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Consumes `self`, yielding the inner storage.
    pub fn into_inner(self) -> S {
        self.0
    }
}

impl<S: AsRef<str>> fmt::Display for NonEmpty<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

impl<S: AsRef<str>> AsRef<str> for NonEmpty<S> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
