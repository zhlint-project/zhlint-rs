//! This rule is checking spaces besides brackets.
//!
//! Options
//! - no_space_inside_bracket: bool
//! - space_outside_half_bracket: Option<bool>
//! - no_space_outside_full_bracket: bool
//!
//! Details:
//! - no_space_inside_bracket:
//!   - left-bracket x anything
//!   - non-left-bracket x right-bracket
//! - space_outside_half_bracket:
//!   - right-half-bracket x left-half-bracket
//!   - right-half-bracket x content/left-quote/code
//!   - content/right-quote/code x left-half-bracket
//! - no_space_outside_full_bracket:
//!   - right-full-bracket x left-full-bracket
//!   - right-full-bracket x content/left-quote/code
//!   - content/right-quote/code x left-full-bracket

use crate::{
    char_kind::{CharKind, CharKindTrait},
    config::Config,
    parser::TextCursor,
    Context,
};

pub fn rule(_ctx: &Context, cursor: &mut TextCursor, config: &Config) {
    // 1. no space inside bracket
    if config.rules.no_space_inside_bracket && cursor.current().is_whitespace() {
        if cursor.prev().is_left_bracket() && cursor.next().is_letters() {
            cursor.delete();
        }
        if cursor.prev().is_letters() && cursor.next().is_right_bracket() {
            cursor.delete();
        }
    }

    // 2. spaces outside half bracket
    match config.rules.space_outside_half_bracket {
        Some(true) => {
            if cursor.prev().kind() == CharKind::LettersFull
                && cursor.current().is_half_width()
                && cursor.current().is_left_bracket()
            {
                cursor.add_prev(' ');
            }
            if cursor.current().is_half_width()
                && cursor.current().is_right_bracket()
                && cursor.next().kind() == CharKind::LettersFull
            {
                cursor.add_next(' ');
            }
        }
        Some(false) => {
            if cursor.prev().is_letters()
                && cursor.current().is_whitespace()
                && cursor.next().is_half_width()
                && cursor.next().is_left_bracket()
            {
                cursor.delete();
            }
            if cursor.prev().is_half_width()
                && cursor.prev().is_right_bracket()
                && cursor.current().is_whitespace()
                && cursor.next().is_letters()
            {
                cursor.delete();
            }
        }
        None => (),
    }

    // 3. no space outside full bracket
    if config.rules.no_space_outside_full_bracket && cursor.current().is_whitespace() {
        if cursor.prev().is_letters()
            && cursor.next().is_full_width()
            && cursor.next().is_left_bracket()
        {
            cursor.delete();
        }
        if cursor.prev().is_full_width()
            && cursor.prev().is_right_bracket()
            && cursor.next().is_letters()
        {
            cursor.delete();
        }
    }
}
