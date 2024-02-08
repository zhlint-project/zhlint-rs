//! This rule is checking spaces besides brackets.
//!
//! Options
//! - no_space_inside_bracket: boolean | undefined
//! - space_outside_half_bracket: boolean | undefined
//! - nospace_outside_full_bracket: boolean | undefined
//!
//! Details:
//! - no_space_inside_bracket:
//!   - left-bracket x anything
//!   - non-left-bracket x right-bracket
//! - space_outside_half_bracket:
//!   - right-half-bracket x left-half-bracket
//!   - right-half-bracket x content/left-quotation/code
//!   - content/right-quotation/code x left-half-bracket
//! - no_space_outside_full_bracket:
//!   - right-full-bracket x left-full-bracket
//!   - right-full-bracket x content/left-quotation/code
//!   - content/right-quotation/code x left-full-bracket

use pulldown_cmark::Event;

use crate::{char_kind::CharKindTrait, config::Config, cursor::Cursor, nodes::Node};

fn should_skip(cursor: &Cursor) -> bool {
    if let (
        Some(before),
        Node::Char {
            value: current,
            space_after: _,
        },
        Some(after),
    ) = (
        cursor.before_visible(),
        &cursor.current(),
        cursor.after_visible(),
    ) {
        assert!(current.is_bracket_punctuation());
        if current.is_wide_or_chinese() || current.modified().is_wide_or_chinese() {
            return false;
        }
        if before.has_space_after() || after.has_space_after() {
            return false;
        }
        match (&before, &after) {
            // x(x  x)x
            //  ^    ^
            (Node::HalfwidthContent { .. }, Node::HalfwidthContent { .. }) => true,
            // x()
            //  ^
            (Node::HalfwidthContent { .. }, Node::Char { value: after, .. }) => {
                current == &'(' && after == &')'
            }
            //()x
            // ^
            (Node::Char { value: before, .. }, Node::HalfwidthContent { .. }) => {
                before == &'(' && current == &')'
            }
            _ => false,
        }
    } else {
        false
    }
}

pub fn rule(cursor: &mut Cursor, config: &Config) {
    let current = if let Node::Char { value, .. } = cursor.current() {
        value.clone()
    } else {
        return;
    };
    // skip non-bracket tokens
    if !current.is_bracket_punctuation() {
        return;
    }

    // 1. no space inside bracket
    if config.no_space_inside_bracket {
        if current.is_left_punctuation() {
            // no space after
            if cursor.after().is_some() {
                cursor.current_mut().remove_space_after();
            }
        } else {
            // no space before
            if let Some(before) = cursor.before_mut() {
                if !before.is_char_and(|c| c.is_left_punctuation()) {
                    before.remove_space_after();
                }
            }
        }
    }

    // skip bracket between half-width content without spaces
    // or empty brackets beside half-width content without spaces
    if should_skip(cursor) {
        return;
    }

    // 2. spaces outside half/full bracket
    if !config.no_space_outside_fullwidth_bracket
        && config.space_outside_halfwidth_bracket.is_none()
    {
        return;
    }

    // 2.1 right-bracket x left-bracket
    if let Some(Node::Char { value: after, .. }) = cursor.after_visible() {
        if current.is_right_punctuation()
            && after.is_bracket_punctuation()
            && after.is_left_punctuation()
        {
            // 2.1.1 any-full-bracket
            // 2.1.2 right-half-bracket x left-half-bracket
            if current.modified().is_wide() || after.modified().is_wide() {
                if config.no_space_outside_fullwidth_bracket {
                    cursor.current_mut().remove_space_after();
                }
            } else if let Some(space) = config.space_outside_halfwidth_bracket {
                cursor.current_mut().modify_space_after(space.into());
            }
        }
    }

    if current.is_left_punctuation() {
        // 2.2 content/right-quotation/code x left-bracket
        if let Some(before) = cursor.before_visible_mut() {
            if match &before {
                Node::Char { .. } => false,
                // content
                Node::HalfwidthContent { .. } | Node::FullwidthContent { .. } => true,
                // code
                Node::Event {
                    value,
                    offset: _,
                    space_after: _,
                } => matches!(value, Event::Code(_)),
                // right-quotation
                Node::Group { .. } => true, // TODO
            } {
                if current.is_wide() {
                    if config.no_space_outside_fullwidth_bracket {
                        before.remove_space_after();
                    }
                } else if let Some(space) = config.space_outside_halfwidth_bracket {
                    before.modify_space_after(space.into());
                }
            }
        }
    } else {
        // 2.3 right-racket x content/left-quotation/code
        if let Some(after) = cursor.after_visible() {
            // 2.3.1 right-full-bracket x content/left-quotation/code
            // 2.4.2 right-half-bracket x content/left-quotation/code
            if match &after {
                Node::Char { .. } => false,
                // content
                Node::HalfwidthContent { .. } | Node::FullwidthContent { .. } => true,
                // code
                Node::Event {
                    value,
                    offset: _,
                    space_after: _,
                } => matches!(value, Event::Code(_)),
                // right-quotation
                Node::Group { .. } => true, // TODO
            } {
                if current.is_wide() {
                    if config.no_space_outside_fullwidth_bracket {
                        cursor.current_mut().remove_space_after();
                    }
                } else if let Some(space) = config.space_outside_halfwidth_bracket {
                    cursor.current_mut().modify_space_after(space.into());
                }
            }
        }
    }
}
