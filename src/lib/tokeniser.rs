use std::marker::PhantomData;

use crate::NonEmpty;

pub trait PreProcessor<'a, T>
where
    T: AsRef<str>,
{
    fn process(input: T) -> Vec<NonEmpty<T>>;
}

pub struct Tokeniser<P, S, T>
where
    T: AsRef<str>,
    P: for<'a> PreProcessor<'a, T>,
    S: Splitter<T>,
{
    _preprocessor: PhantomData<P>,
    _splitter: PhantomData<S>,
    _type: PhantomData<T>,
}

pub struct DefaultPreProcessor {}

impl<'a, T> PreProcessor<'a, T> for DefaultPreProcessor
where
    T: AsRef<str>,
{
    fn process(input: T) -> Vec<NonEmpty<T>> {
        if let Some(value) = NonEmpty::new(input) {
            vec![value]
        } else {
            vec![]
        }
    }
}

pub trait Splitter<T>
where
    T: AsRef<str>,
{
    fn split<'a>(input: Vec<NonEmpty<T>>) -> Vec<NonEmpty<T>>;
}

pub struct NonAlphaNumSplitter {}

impl<'a, T> Splitter<T> for NonAlphaNumSplitter
where
    T: AsRef<str> + 'a,
{
    fn split(input: Vec<NonEmpty<T>>) -> Vec<NonEmpty<T>> {
        input
            .into_iter()
            .flat_map(|c| {
                c.as_ref()
                    .split(|c: char| !c.is_alphanumeric())
                    .map(|x| x as T)
                    .filter_map(NonEmpty::new)
                    .collect::<Vec<NonEmpty<T>>>()
            })
            .collect()
    }
}

impl<P, S, T> Tokeniser<P, S, T>
where
    P: for<'a> PreProcessor<'a, T>,
    S: Splitter<T>,
    T: AsRef<str>,
{
    pub fn new() -> Tokeniser<P, S, T> {
        Tokeniser {
            _preprocessor: PhantomData::<P>,
            _splitter: PhantomData::<S>,
        }
    }
    pub fn tokenise<'a>(input: &'a str) -> Vec<NonEmpty<T>> {
        let preprocessed = P::process(input);
        let split = S::split(preprocessed);
        split
    }
}
