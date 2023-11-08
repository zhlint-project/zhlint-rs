use std::ops::Range;

use pulldown_cmark::{Event, OffsetIter};

use crate::char_kind::{CharKind, CharKindTrait};

pub struct EventCursor<'a> {
    events: OffsetIter<'a, 'a>,
    pub prev_event: Option<(Event<'a>, Range<usize>)>,
    pub current_event: Option<(Event<'a>, Range<usize>)>,
    pub next_event: Option<(Event<'a>, Range<usize>)>,
}

impl<'a> EventCursor<'a> {
    pub fn new(mut events: OffsetIter<'a, 'a>) -> Self {
        Self {
            prev_event: None,
            current_event: events.next(),
            next_event: events.next(),
            events,
        }
    }

    pub fn advance(&mut self) {
        self.prev_event = self.current_event.clone();
        self.current_event = self.next_event.clone();
        self.next_event = self.events.next();
    }

    pub fn to_text_cursor(&self) -> Option<TextCursor> {
        if let Some((Event::Text(s), r)) = &self.current_event {
            Some(TextCursor {
                chars: s
                    .chars()
                    .enumerate()
                    .map(|(i, c)| (c, Some(r.start + i)))
                    .collect(),
                index: 0,
                prev_event: self.prev_event.clone(),
                next_event: self.next_event.clone(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    None,
    Event(&'a Event<'a>),
    Char(char),
}

impl<'a> From<&'a Event<'a>> for Token<'a> {
    fn from(value: &'a Event<'a>) -> Self {
        Token::Event(value)
    }
}

impl<'a> From<char> for Token<'a> {
    fn from(value: char) -> Self {
        Token::Char(value)
    }
}

impl<'a> From<Option<&'a (Event<'a>, Range<usize>)>> for Token<'a> {
    fn from(value: Option<&'a (Event<'a>, Range<usize>)>) -> Self {
        match value {
            Some((event, _)) => Token::Event(event),
            None => Token::None,
        }
    }
}

impl<'a> From<(char, Option<usize>)> for Token<'a> {
    fn from(value: (char, Option<usize>)) -> Self {
        Token::Char(value.0)
    }
}

pub struct TextCursor<'a> {
    chars: Vec<(char, Option<usize>)>,
    index: usize,
    prev_event: Option<(Event<'a>, Range<usize>)>,
    next_event: Option<(Event<'a>, Range<usize>)>,
}

impl TextCursor<'_> {
    pub fn advance(&mut self) -> bool {
        self.index += 1;
        self.index < self.chars.len()
    }

    pub fn skip_str(&mut self, to_skip: &Vec<String>) {
        for s in to_skip {
            if s.chars()
                .enumerate()
                .all(|(i, c)| self.chars.get(self.index + i).is_some_and(|x| x.0 == c))
            {
                self.index += s.len();
                if self.current().is_whitespace() {
                    self.index += 1;
                }
                break;
            }
        }
    }

    pub fn prev(&self) -> Token {
        if self.index == 0 {
            self.prev_event.as_ref().into()
        } else {
            Token::Char(self.chars[self.index - 1].0)
        }
    }

    pub fn prev_skip_space(&self) -> Token {
        let mut i = self.index;
        loop {
            if i == 0 {
                break Token::None;
            }
            i -= 1;
            if self.chars[i].0.kind() != CharKind::Space {
                break self.chars[i].into();
            }
        }
    }

    pub fn current(&self) -> char {
        self.chars[self.index].0
    }

    pub fn current_offset(&self) -> Option<usize> {
        self.chars[self.index].1
    }

    pub fn next(&self) -> Token {
        if self.index >= self.chars.len() - 1 {
            self.next_event.as_ref().into()
        } else {
            Token::Char(self.chars[self.index + 1].0)
        }
    }

    pub fn next_skip_space(&self) -> Token {
        let mut i = self.index;
        loop {
            i += 1;
            if i == self.chars.len() {
                break Token::None;
            }
            if self.chars[i].0.kind() != CharKind::Space {
                break self.chars[i].0.into();
            }
        }
    }

    pub fn delete(&mut self) {
        self.chars.remove(self.index);
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn replace(&mut self, c: char) {
        self.chars[self.index].0 = c;
    }

    pub fn add_prev(&mut self, c: char) {
        self.chars.insert(self.index, (c, None));
    }

    pub fn add_next(&mut self, c: char) {
        self.chars.insert(self.index + 1, (c, None));
    }
}

impl From<TextCursor<'_>> for String {
    fn from(value: TextCursor<'_>) -> Self {
        value.chars.into_iter().map(|(c, _)| c).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use pulldown_cmark::Parser;
        let md = r#"<!-- the good case -->

text before (text inside) text after

<!-- the bad case -->

vm.$on( event, callback )

<!-- then we could write this down below to make it work -->
<!-- zhlint ignore: ( , ) -->
"#;
        let parser = Parser::new_ext(md, pulldown_cmark::Options::empty());
        let mut event_cursor = EventCursor::new(parser.into_offset_iter());
        while let Some(event) = &event_cursor.current_event {
            match event.0 {
                Event::Text(_) => {
                    let mut text_cursor = event_cursor.to_text_cursor().unwrap();
                    loop {
                        println!(
                            "{:?} {:?} {:?}",
                            text_cursor.prev(),
                            text_cursor.current(),
                            text_cursor.next()
                        );
                        if !text_cursor.advance() {
                            break;
                        }
                    }
                }
                _ => println!("Event: {:?}", event),
            }
            event_cursor.advance();
        }
    }
}
