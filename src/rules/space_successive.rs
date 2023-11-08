//! This rule is remove successive spaces.

use crate::{
    char_kind::{CharKind, CharKindTrait},
    config::Config,
    parser::TextCursor,
    Context,
};

pub fn rule(_ctx: &Context, cursor: &mut TextCursor, _config: &Config) {
    if cursor.current().is_whitespace() && cursor.next().kind() == CharKind::Space {
        cursor.delete();
    }
}
