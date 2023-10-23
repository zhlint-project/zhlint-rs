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
    config::Config,
    parser::{CharKind, CharKindTrait, Cursor},
};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // 1. no space inside bracket
    if config.no_space_inside_bracket
        && cursor.current().is_whitespace()
        && ((cursor.previous().is_left_bracket() && cursor.next().is_letters())
            || (cursor.previous().is_letters() && cursor.next().is_right_bracket()))
    {
        cursor.remove()
    }

    // 2. spaces outside half bracket
    match config.space_outside_half_bracket {
        Some(true) => {
            if (cursor.previous().kind() == CharKind::LettersFull
                && cursor.current().is_half_width()
                && cursor.current().is_left_bracket())
                || (cursor.previous().is_half_width()
                    && cursor.previous().is_right_bracket()
                    && cursor.current().is_letters())
            {
                cursor.add(' ')
            }
        }
        Some(false) => {
            if cursor.current().is_whitespace()
                && ((cursor.previous().is_letters()
                    && cursor.next().is_half_width()
                    && cursor.next().is_left_bracket())
                    || (cursor.previous().is_half_width()
                        && cursor.previous().is_right_bracket()
                        && cursor.next().is_letters()))
            {
                cursor.remove()
            }
        }
        None => (),
    }

    // 3. no space outside full bracket
    if config.no_space_outside_full_bracket
        && cursor.current().is_whitespace()
        && ((cursor.previous().is_letters()
            && cursor.next().is_full_width()
            && cursor.next().is_left_bracket())
            || (cursor.previous().is_full_width()
                && cursor.previous().is_right_bracket()
                && cursor.next().is_letters()))
    {
        cursor.remove()
    }
}
