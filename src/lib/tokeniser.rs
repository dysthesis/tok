use std::{marker::PhantomData, ops::Deref};

use crate::NonEmpty;

pub trait PreProcessor {
    fn process<'a>(input: &'a str) -> Option<NonEmpty<'a>>;
}

pub struct Tokeniser<P, S>
where
    P: PreProcessor,
    S: Splitter,
{
    _preprocessor: PhantomData<P>,
    _splitter: PhantomData<S>,
}

pub struct DefaultPreProcessor {}

impl PreProcessor for DefaultPreProcessor {
    fn process<'a>(input: &'a str) -> Option<NonEmpty<'a>> {
        NonEmpty::new(input)
    }
}

pub trait Splitter {
    fn split<'a>(input: NonEmpty<'a>) -> Vec<NonEmpty<'a>>;
}

pub struct NonAlphaNumSplitter {}

impl Splitter for NonAlphaNumSplitter {
    fn split<'a>(input: NonEmpty<'a>) -> Vec<NonEmpty<'a>> {
        input
            .as_str()
            .split(|c: char| !c.is_alphanumeric())
            .filter_map(NonEmpty::new)
            .collect()
    }
}

impl<P, S> Tokeniser<P, S>
where
    P: PreProcessor,
    S: Splitter,
{
    pub fn new() -> Tokeniser<P, S> {
        Tokeniser {
            _preprocessor: PhantomData::<P>,
            _splitter: PhantomData::<S>,
        }
    }
    pub fn tokenise<'a>(input: &'a str) -> Vec<NonEmpty<'a>> {
        let preprocessed = match P::process(input) {
            Some(result) => result,
            None => return vec![],
        };
        let split = S::split(preprocessed);

        split
    }
}
