use std::ops::Range;

use miette::{Diagnostic, LabeledSpan};
use thiserror::Error;

use crate::nodes::Space;

pub type Result<T> = std::result::Result<T, ZhlintError>;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ZhlintError {
    #[error("unexpected end.")]
    UnexpectedEnd,
    #[error("unclosed quotation mark.")]
    UnclosedQuotationMark { value: char, offset: Range<usize> },
    #[error("char error.")]
    CharError {
        value: char,
        modified: char,
        offset: Range<usize>,
    },
    #[error("string error.")]
    StringError {
        value: String,
        modified: String,
        offset: Range<usize>,
    },
    #[error("space error.")]
    SpaceError {
        value: Space,
        modified: Space,
        offset: Range<usize>,
    },
}

impl ZhlintError {
    pub fn label(&self) -> LabeledSpan {
        match self {
            ZhlintError::UnexpectedEnd => LabeledSpan::new(None, 0, 0),
            ZhlintError::UnclosedQuotationMark { value, offset } => LabeledSpan::new(
                Some(value.to_string()),
                offset.start,
                offset.end - offset.start,
            ),
            ZhlintError::CharError {
                value: _,
                modified,
                offset,
            } => LabeledSpan::new(
                Some(modified.to_string()),
                offset.start,
                offset.end - offset.start,
            ),
            ZhlintError::StringError {
                value: _,
                modified,
                offset,
            } => LabeledSpan::new(
                Some(modified.clone()),
                offset.start,
                offset.end - offset.start,
            ),
            ZhlintError::SpaceError {
                value: _,
                modified,
                offset,
            } => LabeledSpan::new(
                Some(match modified {
                    Space::Empty => "should no space hear".to_string(),
                    Space::One => "should one space hear".to_string(),
                    Space::String(s) => format!("should be \"{s}\""),
                }),
                offset.start,
                offset.end - offset.start,
            ),
        }
    }
}

impl Diagnostic for ZhlintError {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn severity(&self) -> Option<miette::Severity> {
        None
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        None
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        Some(Box::new([self.label()].into_iter()))
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        None
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        None
    }
}
