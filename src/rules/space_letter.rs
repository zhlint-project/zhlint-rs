//! This rule is used to check whether there should be a space between
//! content.
//!
//! Options:
//! - space_between_halfwidth_content: bool
//!   - `true`: ensure one space between half-width content (default)
//!   - `false` or `undefined`: do nothing, just keep the original format
//! - no_space_between_fullwidth_content: bool
//!   - `true`: remove the space between full-width content (default)
//!   - `false` or `undefined`: do nothing, just keep the original format
//! - space_between_mixedwidth_content: Option<bool>
//!   - `true`: keep one space between width-mixed content (default)
//!   - `false`: no space between width-mixed content
//!   - `None`: do nothing, just keep the original format
//!
//! Examples (space_between_mixedwidth_content = true):
//! - *a*啊 -> *a* 啊
//! - *a *啊 -> *a* 啊
//! - *啊*a -> *啊* a
//! - *啊 *a -> *啊* a
//!
//! Examples (space_between_mixedwidth_content = false):
//! - *a* 啊 -> *a*啊
//! - *a *啊 -> *a*啊
//! - *啊* a -> *啊*a
//! - *啊 *a -> *啊*a

use crate::{config::Config, cursor::Cursor, nodes::Node};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    match (cursor.current(), cursor.after_visible()) {
        // 1. half x half
        (Node::HalfwidthContent { .. }, Some(Node::HalfwidthContent { .. })) => {
            if config.space_between_halfwidth_content {
                cursor.current_mut().add_space_after();
            }
        }
        // 2. full x full
        (Node::FullwidthContent { .. }, Some(Node::FullwidthContent { .. })) => {
            if config.no_space_between_fullwidth_content {
                cursor.current_mut().remove_space_after();
            }
        }
        // 2. half x full, full x half
        (Node::HalfwidthContent { .. }, Some(Node::FullwidthContent { .. }))
        | (Node::FullwidthContent { .. }, Some(Node::HalfwidthContent { .. })) => {
            if let Some(v) = config.space_between_mixedwidth_letters {
                cursor.current_mut().modify_space_after(v.into())
            }
        }
        _ => (),
    }
}
