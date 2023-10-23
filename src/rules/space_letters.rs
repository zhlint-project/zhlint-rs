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
    config::Config,
    parser::{CharKind, CharKindTrait, Cursor},
};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // 1. no space between full width letters
    if config.no_space_between_full_width_letters
        && cursor.previous().kind() == CharKind::LettersFull
        && cursor.current().is_whitespace()
        && cursor.next().kind() == CharKind::LettersFull
    {
        cursor.remove()
    }

    // 2. space between mixed width letters
    match config.space_between_mixed_width_letters {
        Some(true) => {
            if (!config.skip_zh_units.contains(&cursor.current())
                && !config.skip_zh_units.contains(&cursor.previous()))
                && (cursor.previous().kind() == CharKind::LettersFull
                    && cursor.current().kind() == CharKind::LettersHalf
                    || cursor.previous().kind() == CharKind::LettersHalf
                        && cursor.current().kind() == CharKind::LettersFull)
            {
                cursor.add(' ')
            }
        }
        Some(false) => {
            if cursor.current().is_whitespace()
                && (cursor.previous().kind() == CharKind::LettersFull
                    && cursor.next().kind() == CharKind::LettersHalf
                    || cursor.previous().kind() == CharKind::LettersHalf
                        && cursor.next().kind() == CharKind::LettersFull)
            {
                cursor.remove()
            }
        }
        None => (),
    }
}
