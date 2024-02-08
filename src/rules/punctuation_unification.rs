//! This rule will unify similar punctuations into the same one.
//! Usually, it's just about Chinese quotes.
//!
//! Options:
//! - unified_punctuation: "simplified" (default) | "traditional" | None

use crate::{
    config::{Config, ZhScript},
    cursor::Cursor,
    nodes::{Node, OffsetValue},
};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    let check = |value: &mut OffsetValue<char>| {
        value.to_be(match config.unified_punctuation {
            Some(ZhScript::Simplified) => match value.modified() {
                '「' => '“',
                '」' => '”',
                '『' => '‘',
                '』' => '’',
                _ => return,
            },
            Some(ZhScript::Traditional) => match value.modified() {
                '“' => '「',
                '”' => '」',
                '‘' => '『',
                '’' => '』',
                _ => return,
            },
            None => return,
        })
    };
    if let Node::Group {
        start,
        inner_space_before: _,
        nodes: _,
        end,
        space_after: _,
    } = &mut cursor.current_mut()
    {
        check(start);
        check(end);
    }
}
