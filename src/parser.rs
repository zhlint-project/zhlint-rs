use std::ops::Range;

use pulldown_cmark::{Event, Tag};

use crate::{
    char_kind::{
        CharKind, CharKindTrait, CHINESE_LEFT_QUOTATION_PUNCTUATION,
        CHINESE_RIGHT_QUOTATION_PUNCTUATION, WESTERN_QUOTATION_PUNCTUATION,
    },
    errors::{Result, ZhlintError},
    lexer::Token,
    nodes::{Node, OffsetValue, ParagraphNodes, Space},
};

pub struct Parser<'a, T: Iterator<Item = Token<'a>>> {
    pub tokens: T,
    pub prev: Token<'a>,
    pub current: Token<'a>,
    pub next: Token<'a>,
}

impl<'a: 'b, 'b, T: Iterator<Item = Token<'a>>> Parser<'a, T> {
    pub fn new(mut tokens: T) -> Self {
        Self {
            prev: Token::None,
            current: tokens.next().unwrap_or(Token::None),
            next: tokens.next().unwrap_or(Token::None),
            tokens,
        }
    }

    fn bump(&mut self) {
        self.prev = self.current.clone();
        self.current = self.next.clone();
        self.next = self.tokens.next().unwrap_or(Token::None);
    }

    fn eat_event(&mut self, t: &Event<'a>) -> bool {
        let is_present = match &self.current {
            Token::Event { value, offset: _ } => value == t,
            _ => false,
        };
        if is_present {
            self.bump()
        }
        is_present
    }

    fn eat_char_while(
        &mut self,
        mut start_char: String,
        mut start_offset: Range<usize>,
        f: impl Fn(char) -> bool,
    ) -> (String, Range<usize>) {
        while let Token::Char { value, offset } = &self.current {
            if !f(*value) {
                break;
            }
            start_offset.end = offset.end;
            start_char.push(*value);
            self.bump();
        }
        (start_char, start_offset)
    }

    pub fn parse(&mut self) -> Vec<Result<ParagraphNodes<'b>>> {
        let mut res = Vec::new();
        while self.current != Token::None {
            res.push(self.parse_paragraph_nodes());
        }
        res
    }

    fn parse_paragraph_nodes(&mut self) -> Result<ParagraphNodes<'b>> {
        assert!(matches!(
            &self.current,
            Token::Event {
                value: Event::Start(Tag::Paragraph),
                offset: _
            }
        ));
        let mut res = Vec::new();
        while !self.eat_event(&Event::End(Tag::Paragraph)) {
            res.push(self.parse_node()?);
        }
        Ok(ParagraphNodes(res))
    }

    fn parse_node(&mut self) -> Result<Node<'b>> {
        let current = self.current.clone();
        self.bump();
        match current {
            Token::None => Err(ZhlintError::UnexpectedEnd),
            Token::Event { value, offset } => Ok(Node::Event {
                space_after: self.parse_space(offset.end),
                value,
                offset,
            }),
            Token::Char { value, offset } => {
                match value.kind() {
                    CharKind::LetterHalf => {
                        let (value, offset) =
                            self.eat_char_while(value.to_string(), offset.clone(), |c| {
                                c.kind() == CharKind::LetterHalf
                            });
                        return Ok(Node::HalfwidthContent {
                            space_after: self.parse_space(offset.end),
                            value,
                            offset,
                        });
                    }
                    CharKind::LetterFull => {
                        let (value, offset) =
                            self.eat_char_while(value.to_string(), offset.clone(), |c| {
                                c.kind() == CharKind::LetterFull
                            });
                        return Ok(Node::FullwidthContent {
                            space_after: self.parse_space(offset.end),
                            value,
                            offset,
                        });
                    }
                    _ => (),
                }

                if let Some(i) = CHINESE_LEFT_QUOTATION_PUNCTUATION
                    .iter()
                    .position(|c| c == &value)
                {
                    return self.parse_group(value, CHINESE_RIGHT_QUOTATION_PUNCTUATION[i], offset);
                } else if CHINESE_RIGHT_QUOTATION_PUNCTUATION.contains(&value) {
                    return Err(ZhlintError::UnclosedQuotationMark { value, offset });
                } else if WESTERN_QUOTATION_PUNCTUATION.contains(&value) {
                    let mut skip = false;
                    // skip x'x
                    if let (Token::Char { value: prev, .. }, Token::Char { value: next, .. }) =
                        (&self.prev, &self.next)
                    {
                        if prev.kind() == CharKind::LetterHalf
                            && value == '\''
                            && next.kind() == CharKind::LetterHalf
                        {
                            skip = true;
                        }
                    }
                    if !skip {
                        return self.parse_group(value, value, offset);
                    }
                }

                Ok(Node::Char {
                    space_after: self.parse_space(offset.end),
                    value: OffsetValue::new(value, offset),
                })
            }
        }
    }

    fn parse_group(
        &mut self,
        start: char,
        end: char,
        start_offset: Range<usize>,
    ) -> Result<Node<'b>> {
        let end_offset;
        Ok(Node::Group {
            start: OffsetValue::new(start, start_offset.clone()),
            inner_space_before: self.parse_space(start_offset.end),
            nodes: {
                let mut nodes = Vec::new();
                loop {
                    if let Token::Char { value, offset } = &self.current {
                        if value == &end {
                            end_offset = offset.clone();
                            self.bump();
                            break;
                        }
                    }
                    nodes.push(self.parse_node()?);
                }
                nodes
            },
            end: OffsetValue::new(end, end_offset.clone()),
            space_after: self.parse_space(end_offset.end),
        })
    }

    fn parse_space(&mut self, start: usize) -> OffsetValue<Space> {
        let start = self
            .current
            .offset()
            .map(|x| x.start)
            .unwrap_or(self.prev.offset().unwrap().end);
        let (space, offset) = self.eat_char_while(String::new(), start..start, |c| c.is_space());
        OffsetValue::new(space.into(), offset)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Parser::new(Lexer::new(r#"foo:" bar *baz*" xxx"#)).parse(),
            vec![Ok(ParagraphNodes(vec![
                Node::Event {
                    value: Event::Start(Tag::Paragraph),
                    offset: 0..20,
                    space_after: OffsetValue {
                        original: Space::Empty,
                        modified: None,
                        offset: 20..20,
                    },
                },
                Node::HalfwidthContent {
                    value: "foo".to_string(),
                    offset: 0..3,
                    space_after: OffsetValue {
                        original: Space::Empty,
                        modified: None,
                        offset: 3..3,
                    },
                },
                Node::Char {
                    value: OffsetValue {
                        original: ':',
                        modified: None,
                        offset: 3..4,
                    },
                    space_after: OffsetValue {
                        original: Space::Empty,
                        modified: None,
                        offset: 4..4,
                    },
                },
                Node::Group {
                    start: OffsetValue {
                        original: '"',
                        modified: None,
                        offset: 4..5,
                    },
                    inner_space_before: OffsetValue {
                        original: Space::One,
                        modified: None,
                        offset: 5..6,
                    },
                    nodes: vec![
                        Node::HalfwidthContent {
                            value: "bar".to_string(),
                            offset: 6..9,
                            space_after: OffsetValue {
                                original: Space::One,
                                modified: None,
                                offset: 9..10,
                            },
                        },
                        Node::Event {
                            value: Event::Start(Tag::Emphasis),
                            offset: 10..15,
                            space_after: OffsetValue {
                                original: Space::Empty,
                                modified: None,
                                offset: 15..15,
                            },
                        },
                        Node::HalfwidthContent {
                            value: "baz".to_string(),
                            offset: 11..14,
                            space_after: OffsetValue {
                                original: Space::Empty,
                                modified: None,
                                offset: 14..14,
                            },
                        },
                        Node::Event {
                            value: Event::End(Tag::Emphasis),
                            offset: 10..15,
                            space_after: OffsetValue {
                                original: Space::Empty,
                                modified: None,
                                offset: 15..15,
                            },
                        },
                    ],
                    end: OffsetValue {
                        original: '"',
                        modified: None,
                        offset: 15..16,
                    },
                    space_after: OffsetValue {
                        original: Space::One,
                        modified: None,
                        offset: 16..17,
                    },
                },
                Node::HalfwidthContent {
                    value: "xxx".to_string(),
                    offset: 17..20,
                    space_after: OffsetValue {
                        original: Space::Empty,
                        modified: None,
                        offset: 20..20,
                    },
                },
            ]))]
        )
    }
}
