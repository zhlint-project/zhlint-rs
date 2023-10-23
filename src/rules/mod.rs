use crate::{config::Config, parser::Cursor};

mod punctuation_unification;
mod punctuation_width;
mod space_bracket;
mod space_letters;
mod space_punctuation;
mod space_quote;

pub fn rules() -> Vec<fn(&mut Cursor, &Config)> {
    vec![
        punctuation_width::rule,
        punctuation_unification::rule,
        space_bracket::rule,
        space_letters::rule,
        space_punctuation::rule,
        space_quote::rule,
    ]
}
