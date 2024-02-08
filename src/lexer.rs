use std::ops::Range;

use pulldown_cmark::{CowStr, Event, OffsetIter, Parser};

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    None,
    Event {
        value: Event<'a>,
        offset: Range<usize>,
    },
    Char {
        value: char,
        offset: Range<usize>,
    },
}

impl Token<'_> {
    pub fn offset(&self) -> Option<Range<usize>> {
        match self {
            Token::None => None,
            Token::Event { value: _, offset } => Some(offset.clone()),
            Token::Char { value: _, offset } => Some(offset.clone()),
        }
    }
}

pub struct Lexer<'a> {
    events: OffsetIter<'a, 'a>,

    text: Option<CowStr<'a>>,
    event_offset: usize,
    char_offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Lexer {
            events: Parser::new(text).into_offset_iter(),
            text: None,
            event_offset: 0,
            char_offset: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(text) = &self.text {
            if let Some(c) = text[self.char_offset..].chars().next() {
                let start = self.event_offset + self.char_offset;
                self.char_offset += c.len_utf8();
                let end = self.event_offset + self.char_offset;
                return Some(Token::Char {
                    value: c,
                    offset: start..end,
                });
            } else {
                self.text = None;
            }
        }
        match self.events.next() {
            Some((Event::Text(text), offset)) => {
                self.text = Some(text);
                self.event_offset = offset.start;
                self.char_offset = 0;
                self.next()
            }
            Some((value, offset)) => Some(Token::Event { value, offset }),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let md = r#"- xxx
- xxx
  abc
"#;

        for token in Lexer::new(md) {
            println!("{:?}", token);
        }
    }
}
