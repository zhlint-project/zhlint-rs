//! This rule is checking spaces besides normal punctuations.
//! Usually, for full-width punctuations, we don't need any spaces around.
//! For half-width punctuations, we need a space after that.
//!
//! Options
//! - no_space_before_punctuation: bool
//!   - `true`: remove spaces before a half-width punctuation (default)
//!   - `false` or `undefined`: do nothing, just keep the original format
//! - space_after_half_width_punctuation: bool
//!   - `true`: ensure one space after a half-width punctuation (default)
//!   - `false` or `undefined`: do nothing, just keep the original format
//! - no_space_after_full_width_punctuation: bool
//!   - `true`: remove spaces around a full-width punctuation (default)
//!   - `false` or `undefined`: do nothing, just keep the original format
//!
//! Details:
//! - no_space_before_punctuation:
//!   content/right-quote/right-bracket/code x punctuation
//! - space_after_half_width_punctuation:
//!   half x content/left-quote/left-bracket/code
//! - no_space_after_full_width_punctuation:
//!   full x content/left-quote/left-bracket/code
//!
//! - skip half-width punctuations between half-width content without space
//! - skip successive multiple half-width punctuations

use pulldown_cmark::Event;

use crate::{char_kind::CharKindTrait, config::Config, cursor::Cursor, nodes::Node};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // skip non-punctuation tokens and non-normal punctuations
    let current = if let Node::Char { value, .. } = &cursor.current() {
        value.clone()
    } else {
        return;
    };
    if !current.is_pause_stop_punctuation() {
        return;
    }
    // skip half-width punctuations between half-width content without space
    if cursor.is_halfwidth_punctuation_without_space_around() {
        return;
    }
    // skip successive multiple half-width punctuations
    if cursor.is_successive_halfwidth_punctuation() {
        return;
    }

    // check whether node is content/right-quotation/right-bracket/code
    let is_content = |node: &Node| {
        match &node {
            // right-bracket
            Node::Char {
                value,
                space_after: _,
            } => value.is_bracket_punctuation() && value.is_right_punctuation(),
            // content
            Node::HalfwidthContent { .. } | Node::FullwidthContent { .. } => true,
            // code
            Node::Event {
                value,
                offset: _,
                space_after: _,
            } => matches!(value, Event::Code(_)),
            // right-quotation
            Node::Group { .. } => true,
        }
    };

    // 1. content/right-quotation/right-bracket/code x punctuation
    if config.no_space_before_pause_or_stop {
        if let Some(before) = cursor.before_visible_mut() {
            if is_content(before) {
                before.remove_space_after();
            }
        }
    }

    // 2. half/full x content/left-quotation/left-bracket/code
    if let Some(after) = cursor.after_visible_mut() {
        if is_content(after) {
            if current.modified().is_wide() {
                if config.no_space_after_fullwidth_pause_or_stop {
                    cursor.current_mut().remove_space_after();
                }
            } else if let Some(space) = config.space_after_halfwidth_pause_or_stop {
                cursor.current_mut().modify_space_after(space.into());
            }
        }
    }
}
