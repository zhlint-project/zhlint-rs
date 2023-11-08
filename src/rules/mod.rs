use crate::{config::Config, parser::TextCursor, Context};

mod punctuation_unification;
mod punctuation_width;
mod space_bracket;
mod space_code;
mod space_letters;
mod space_punctuation;
mod space_quote;
mod space_successive;
mod space_trim;

pub fn rules() -> Vec<fn(&Context, &mut TextCursor, &Config)> {
    vec![
        space_trim::rule,
        space_successive::rule,
        //
        punctuation_width::rule,
        punctuation_unification::rule,
        //
        space_code::rule,
        space_letters::rule,
        space_punctuation::rule,
        space_quote::rule,
        space_bracket::rule,
    ]
}
