use pulldown_cmark::Event;

use crate::{char_kind::CharKindTrait, nodes::Node};

#[derive(Debug)]
pub struct Cursor<'a, 'b> {
    nodes: &'a mut Vec<Node<'b>>,
    index: usize,
}

impl<'a, 'b> Cursor<'a, 'b> {
    pub fn new(nodes: &'a mut Vec<Node<'b>>, index: usize) -> Self {
        Cursor { nodes, index }
    }

    pub fn before(&self) -> Option<&Node<'b>> {
        self.nodes.get(self.index - 1)
    }

    pub fn before_mut(&mut self) -> Option<&mut Node<'b>> {
        self.nodes.get_mut(self.index - 1)
    }

    pub fn before_visible(&self) -> Option<&Node<'b>> {
        self.nodes[..self.index].iter().rev().find(|node| {
            !matches!(
                &node,
                Node::Event {
                    value: Event::Start(_) | Event::End(_),
                    offset: _,
                    space_after: _,
                }
            )
        })
    }

    pub fn before_visible_mut(&mut self) -> Option<&mut Node<'b>> {
        self.nodes[..self.index].iter_mut().rev().find(|node| {
            !matches!(
                &node,
                Node::Event {
                    value: Event::Start(_) | Event::End(_),
                    offset: _,
                    space_after: _,
                }
            )
        })
    }

    pub fn current(&self) -> &Node<'b> {
        &self.nodes[self.index]
    }

    pub fn current_mut(&mut self) -> &mut Node<'b> {
        &mut self.nodes[self.index]
    }

    pub fn after(&self) -> Option<&Node<'b>> {
        self.nodes.get(self.index + 1)
    }

    pub fn after_mut(&mut self) -> Option<&mut Node<'b>> {
        self.nodes.get_mut(self.index + 1)
    }

    pub fn after_visible(&self) -> Option<&Node<'b>> {
        self.nodes[self.index + 1..].iter().find(|node| {
            !matches!(
                &node,
                Node::Event {
                    value: Event::Start(_) | Event::End(_),
                    offset: _,
                    space_after: _,
                }
            )
        })
    }

    pub fn after_visible_mut(&mut self) -> Option<&mut Node<'b>> {
        self.nodes[self.index + 1..].iter_mut().find(|node| {
            !matches!(
                &node,
                Node::Event {
                    value: Event::Start(_) | Event::End(_),
                    offset: _,
                    space_after: _,
                }
            )
        })
    }

    pub fn is_halfwidth_punctuation_without_space_around(&self) -> bool {
        if let (Some(before), Some(after)) = (self.before(), self.after()) {
            if self.current().is_char_and(|c| c.is_western_punctuation())
                && !self.current().has_space_after()
                && matches!(before, Node::HalfwidthContent { .. })
                && !before.has_space_after()
                && matches!(after, Node::HalfwidthContent { .. })
            {
                return true;
            }
        }
        false
    }

    pub fn is_successive_halfwidth_punctuation(&self) -> bool {
        if let (
            Some(Node::Char {
                value: before,
                space_after: space_before,
            }),
            Node::Char {
                value: current,
                space_after: _,
            },
        ) = (self.before(), self.current())
        {
            if current.is_western_punctuation()
                && before.is_western_punctuation()
                && space_before.is_empty()
            {
                return true;
            }
        }
        if let (
            Node::Char {
                value: current,
                space_after,
            },
            Some(Node::Char {
                value: after,
                space_after: _,
            }),
        ) = (self.current(), self.after())
        {
            println!("{after} {:?}", after.kind());
            if current.is_western_punctuation()
                && after.is_western_punctuation()
                && space_after.is_empty()
            {
                return true;
            }
        }
        false
    }

    pub fn match_abbr(&self, abbrs: &[String]) -> bool {
        true
    }
}
