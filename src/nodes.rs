use std::{
    fmt,
    ops::{Deref, Range},
};

use pulldown_cmark::Event;

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphNodes<'a>(pub Vec<Node<'a>>);

#[derive(Debug, Clone, PartialEq)]
pub enum Node<'a> {
    Char {
        value: OffsetValue<char>,
        space_after: OffsetValue<Space>,
    },
    HalfwidthContent {
        value: String,
        offset: Range<usize>,
        space_after: OffsetValue<Space>,
    },
    FullwidthContent {
        value: String,
        offset: Range<usize>,
        space_after: OffsetValue<Space>,
    },
    Event {
        value: Event<'a>,
        offset: Range<usize>,
        space_after: OffsetValue<Space>,
    },
    Group {
        start: OffsetValue<char>,
        inner_space_before: OffsetValue<Space>,
        nodes: Vec<Node<'a>>,
        end: OffsetValue<char>,
        space_after: OffsetValue<Space>,
    },
}

impl Node<'_> {
    const fn space_after(&self) -> &OffsetValue<Space> {
        match self {
            Node::Char { space_after, .. } => space_after,
            Node::HalfwidthContent { space_after, .. } => space_after,
            Node::FullwidthContent { space_after, .. } => space_after,
            Node::Event { space_after, .. } => space_after,
            Node::Group { space_after, .. } => space_after,
        }
    }

    fn space_after_mut(&mut self) -> &mut OffsetValue<Space> {
        match self {
            Node::Char { space_after, .. } => space_after,
            Node::HalfwidthContent { space_after, .. } => space_after,
            Node::FullwidthContent { space_after, .. } => space_after,
            Node::Event { space_after, .. } => space_after,
            Node::Group { space_after, .. } => space_after,
        }
    }

    pub fn has_space_after(&self) -> bool {
        !self.space_after().is_empty()
    }

    pub fn add_space_after(&mut self) {
        self.space_after_mut().to_be(Space::One);
    }

    pub fn remove_space_after(&mut self) {
        self.space_after_mut().to_be(Space::Empty);
    }

    pub fn modify_space_after(&mut self, space: Space) {
        self.space_after_mut().to_be(space);
    }

    pub fn revert_space_after(&mut self) {
        self.space_after_mut().revert_modified()
    }

    pub fn is_char_and(&self, f: impl FnOnce(&OffsetValue<char>) -> bool) -> bool {
        match self {
            Node::Char { value, .. } => f(value),
            _ => false,
        }
    }

    pub const fn is_start_wrapper(&self) -> bool {
        matches!(
            self,
            Node::Event {
                value: Event::Start(_),
                offset: _,
                space_after: _,
            }
        )
    }

    pub const fn is_end_wrapper(&self) -> bool {
        matches!(
            self,
            Node::Event {
                value: Event::End(_),
                offset: _,
                space_after: _,
            }
        )
    }

    pub const fn is_wrapper(&self) -> bool {
        self.is_start_wrapper() || self.is_end_wrapper()
    }
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Char { value, space_after } => write!(f, "{value}{space_after}"),
            Node::HalfwidthContent {
                value,
                offset: _,
                space_after,
            } => write!(f, "{value}{space_after}"),
            Node::FullwidthContent {
                value,
                offset: _,
                space_after,
            } => write!(f, "{value}{space_after}"),
            Node::Event {
                value,
                offset: _,
                space_after,
            } => write!(f, "{:?}{}", value, space_after),
            Node::Group {
                start,
                inner_space_before,
                nodes,
                end,
                space_after,
            } => {
                write!(f, "{}", start)?;
                write!(f, "{}", inner_space_before)?;
                for node in nodes {
                    write!(f, "{}", node)?;
                }
                write!(f, "{}", end)?;
                write!(f, "{}", space_after)?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetValue<T> {
    pub original: T,
    pub modified: Option<T>,
    pub offset: Range<usize>,
}

impl<T> Deref for OffsetValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.original
    }
}

impl<T> OffsetValue<T> {
    pub fn new(value: T, offset: Range<usize>) -> Self {
        OffsetValue {
            original: value,
            modified: None,
            offset,
        }
    }

    pub fn modified(&self) -> &T {
        self.modified.as_ref().unwrap_or(&self.original)
    }
}

impl<T: PartialEq> OffsetValue<T> {
    pub fn to_be(&mut self, modified: T) {
        if self.original != modified {
            self.modified = Some(modified);
        } else {
            self.modified = None;
        }
    }

    pub fn revert_modified(&mut self) {
        self.modified = None;
    }
}

impl<T: PartialEq> PartialEq<T> for OffsetValue<T> {
    fn eq(&self, other: &T) -> bool {
        self.modified() == other
    }
}

impl<T: fmt::Display> fmt::Display for OffsetValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.modified())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Space {
    Empty,
    One,
    String(String),
}

impl Space {
    pub const fn is_empty(&self) -> bool {
        matches!(self, Space::Empty)
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Space::Empty => write!(f, ""),
            Space::One => write!(f, " "),
            Space::String(s) => write!(f, "{s}"),
        }
    }
}

impl From<bool> for Space {
    fn from(value: bool) -> Self {
        if value {
            Space::One
        } else {
            Space::Empty
        }
    }
}

impl From<String> for Space {
    fn from(value: String) -> Self {
        if value.is_empty() {
            Space::Empty
        } else if value == " " {
            Space::One
        } else {
            Space::String(value)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use pulldown_cmark::Parser;
        let md = r#"
<code></code>

<b>123</b>  345*789* `2123`
<abc>

<br/>

"#;
        let parser = Parser::new_ext(md, pulldown_cmark::Options::empty());
        for (event, offset) in parser.into_offset_iter() {
            println!("{:?} {:?} {:?}", event, offset, md.get(offset.clone()));
        }
    }
}
