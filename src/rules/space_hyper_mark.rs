//! This rule is to ensure all the existing spaces should be outside hyper
//! marks like//!, _, [, ], etc.
//!
//! Options:
//! - no_space_inside_mark: bool
//!
//! For example:
//! - `x _//!* yyy//!* _ z` should be `x _**yyy**_ z`
//!
//! Details:
//! - left-mark x left-mark: `x _//!*yyy**_ z`
//!                             ^^^
//! - right-mark x right-mark: `x _**yyy** _ z`
//!                                      ^^^
//! - left-mark x non-mark: `x _** yyy**_ z`
//!                              ^^^
//! - non-mark x right-mark: `x _**yyy//!*_ z`
//!                                 ^^^

use crate::{config::Config, cursor::Cursor};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // skip if there is no options
    if !config.no_space_inside_hyper_mark {
        return;
    }

    // skip non-after-token situations
    if let Some(after) = cursor.after() {
        // skip non-mark situations
        if !cursor.current().is_wrapper() && !after.is_wrapper() {
            return;
        }

        // 1. left x left
        // 2. right x right
        // 3. left x non-mark
        // 4. non-mark x right
        if (cursor.current().is_start_wrapper() && after.is_start_wrapper())
            || (cursor.current().is_end_wrapper() && after.is_end_wrapper())
            || (cursor.current().is_start_wrapper() && !after.is_wrapper())
            || (!cursor.current().is_wrapper() && after.is_end_wrapper())
        {
            cursor.current_mut().remove_space_after();
        }
    }
}
