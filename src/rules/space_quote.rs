//! This rule is checking spaces besides quotes.
//!
//! Options
//! - no_space_inside_quote: bool
//! - space_outside_half_quote: Option<bool>
//! - no_space_outside_full_quote: bool
//!
//! Details:
//! - no_space_inside_quote:
//!   - left-quote x right-quote
//!   - content/punctuation/right-quote/right-quote/code/unknown/container x right-quote
//!   - left-quote x content/punctuation/left-quote/left-quote/code/unknown/container
//! - space_outside_half_quote:
//!   - right-half-quote x left-half-quote
//!   - content/code x left-half-quote
//!   - right-half-quote x content/code
//! - no_space_outside_full_quote:
//!   - right-full-quote x left-full-quote
//!   - content/code x left-full-quote
//!   - right-full-quote x content/code

use crate::{char_kind::CharKindTrait, config::Config, parser::TextCursor, Context};

pub fn rule(ctx: &Context, cursor: &mut TextCursor, config: &Config) {
    // 1. no space inside quote
    let previous_is_left_quote = cursor.prev().is_left_quote()
        || cursor.prev().is_half_width_quote() && ctx.half_width_double_quote_count % 2 == 1;
    let next_is_right_quote = cursor.next().is_right_quote()
        || cursor.next().is_half_width_quote() && ctx.half_width_double_quote_count % 2 == 1;
    if config.rules.no_space_inside_quote
        && cursor.current().is_whitespace()
        && ((previous_is_left_quote && cursor.next().is_letters())
            || (cursor.prev().is_letters() && next_is_right_quote))
    {
        cursor.delete();
    }

    // 2. spaces outside half quote
    match config.rules.space_outside_half_quote {
        Some(true) => {
            if cursor.prev().is_letters()
                && cursor.current().is_half_width_quote()
                && ctx.half_width_double_quote_count % 2 == 0
            {
                cursor.add_prev(' ');
            }
            if cursor.current().is_half_width_quote()
                && ctx.half_width_double_quote_count % 2 == 0
                && cursor.next().is_letters()
            {
                cursor.add_next(' ');
            }
        }
        Some(false) => {
            if cursor.prev().is_letters()
                && cursor.current().is_whitespace()
                && cursor.next().is_half_width_quote()
                && ctx.half_width_double_quote_count % 2 == 0
            {
                cursor.delete();
            }
            if cursor.prev().is_half_width_quote()
                && ctx.half_width_double_quote_count % 2 == 0
                && cursor.current().is_whitespace()
                && cursor.next().is_letters()
            {
                cursor.delete();
            }
        }
        None => (),
    }

    // 3. no space outside full quote
    if config.rules.no_space_outside_full_quote
        && cursor.current().is_whitespace()
        && ((cursor.prev().is_letters() && cursor.next().is_left_quote())
            || (cursor.prev().is_right_quote() && cursor.next().is_letters()))
    {
        cursor.delete();
    }
}
