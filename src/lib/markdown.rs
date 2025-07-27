use std::borrow::Cow;

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, TextMergeStream};

use crate::{NonEmpty, tokeniser::PreProcessor};

pub struct MarkdownPreProcessor {}

impl<'a> PreProcessor<'a, CowStr<'a>> for MarkdownPreProcessor {
    fn process<T>(input: T) -> Vec<NonEmpty<CowStr<'a>>>
    where
        T: AsRef<str>,
    {
        let opts = Options::ENABLE_YAML_STYLE_METADATA_BLOCKS;
        let texts: Vec<NonEmpty<CowStr<'a>>> = TextMergeStream::new(Parser::new_ext(input, opts))
            .scan(false, |skip, e| match e {
                Event::Start(Tag::CodeBlock(_)) => {
                    *skip = true;
                    None
                }
                Event::End(TagEnd::CodeBlock) => {
                    *skip = false;
                    None
                }
                Event::Text(t) if !*skip => Some(t),
                _ => None,
            })
            .filter_map(NonEmpty::new)
            .collect::<Vec<NonEmpty<CowStr<'a>>>>();

        texts
    }
}
