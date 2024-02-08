use std::sync::OnceLock;

use crate::{config::Config, cursor::Cursor};

pub type Rule = fn(&mut Cursor, &Config);

pub mod case_abbrs;
pub mod case_zh_units;
pub mod punctuation_unification;
pub mod punctuation_width;
pub mod space_bracket;
pub mod space_code;
pub mod space_hyper_mark;
pub mod space_letter;
pub mod space_punctuation;
pub mod space_quotation;

pub fn rules() -> &'static Vec<Rule> {
    static RULES: OnceLock<Vec<Rule>> = OnceLock::new();
    RULES.get_or_init(|| {
        vec![
            punctuation_width::rule,
            punctuation_unification::rule,
            //
            // case_abbrs::rule,
            //
            space_hyper_mark::rule,
            space_code::rule,
            space_letter::rule,
            space_punctuation::rule,
            space_quotation::rule,
            space_bracket::rule,
            //
            case_zh_units::rule,
        ]
    })
}
