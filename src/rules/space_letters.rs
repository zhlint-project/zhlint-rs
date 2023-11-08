//! This rule is used to check whether there should be a space between
//! content.
//!
//! Options:
//! - no_space_between_full_width_letters: bool
//!   - `true`: remove the space between full-width content (default)
//!   - `false` or `undefined`: do nothing, just keep the original format
//! - space_between_mixed_width_letters: Option<bool>
//!   - `true`: keep one space between width-mixed content (default)
//!   - `false`: no space between width-mixed content
//!   - `undefined`: do nothing, just keep the original format
//!
//! Examples (space_between_mixed_width_letters = true):
//! - *a*啊 -> *a* 啊
//! - *a *啊 -> *a* 啊
//! - *啊*a -> *啊* a
//! - *啊 *a -> *啊* a
//!
//! Examples (space_between_mixed_width_letters = false):
//! - *a* 啊 -> *a*啊
//! - *a *啊 -> *a*啊
//! - *啊* a -> *啊*a
//! - *啊 *a -> *啊*a

use crate::{
    char_kind::{CharKind, CharKindTrait},
    config::Config,
    parser::TextCursor,
    Context,
};

pub fn rule(_ctx: &Context, cursor: &mut TextCursor, config: &Config) {
    // 1. no space between full width letters
    if config.rules.no_space_between_full_width_letters
        && cursor.prev().kind() == CharKind::LettersFull
        && cursor.current().is_whitespace()
        && cursor.next().kind() == CharKind::LettersFull
    {
        cursor.delete();
    }

    // 2. space between mixed width letters
    match config.rules.space_between_mixed_width_letters {
        Some(true) => {
            if cursor.current().kind() == CharKind::LettersFull
                && cursor.next().kind() == CharKind::LettersHalf
                && !config.rules.skip_zh_units.contains(&cursor.current())
            {
                cursor.add_next(' ');
            }

            if cursor.prev().kind() == CharKind::LettersHalf
                && cursor.current().kind() == CharKind::LettersFull
                && !config.rules.skip_zh_units.contains(&cursor.current())
            {
                cursor.add_prev(' ');
            }
        }
        Some(false) => {
            if cursor.prev().kind() == CharKind::LettersFull
                && cursor.current().is_whitespace()
                && cursor.next().kind() == CharKind::LettersHalf
            {
                cursor.delete();
            }
            if cursor.prev().kind() == CharKind::LettersHalf
                && cursor.current().is_whitespace()
                && cursor.next().kind() == CharKind::LettersFull
            {
                cursor.delete();
            }
        }
        None => (),
    }
}
