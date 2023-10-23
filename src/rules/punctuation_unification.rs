//! This rule will unify similar punctuations into the same one.
//! Usually, it's just about Chinese quotes.
//!
//! Options:
//! - unified_punctuation: "simplified" (default) | "traditional" | None

use crate::{
    config::{Config, ZhScript},
    parser::Cursor,
};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    cursor.set(match config.unified_punctuation {
        Some(ZhScript::Simplified) => match cursor.current() {
            '「' => '“',
            '」' => '”',
            '『' => '‘',
            '』' => '’',
            _ => return,
        },
        Some(ZhScript::Traditional) => match cursor.current() {
            '“' => '「',
            '”' => '」',
            '‘' => '『',
            '’' => '』',
            _ => return,
        },
        None => return,
    });
}
