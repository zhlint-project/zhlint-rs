//! This rule will unify similar punctuations into the same one.
//! Usually, it's just about Chinese quotes.
//!
//! Options:
//! - unified_punctuation: "simplified" (default) | "traditional" | None

use crate::{
    config::{Config, ZhScript},
    parser::TextCursor,
    Context,
};

pub fn rule(_ctx: &Context, cursor: &mut TextCursor, config: &Config) {
    cursor.replace(match config.rules.unified_punctuation {
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
