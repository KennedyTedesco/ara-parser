use ara_source::source::Source;

use crate::lexer::state::source_bytes::SourceBytes;

pub mod source_bytes;

#[derive(Debug)]
pub struct State<'a> {
    pub source: &'a Source,
    pub bytes: SourceBytes<'a>,
}

impl<'a> State<'a> {
    pub fn new(source: &'a Source, content: &'a [u8]) -> Self {
        Self {
            source,
            bytes: SourceBytes::new(content),
        }
    }
}
