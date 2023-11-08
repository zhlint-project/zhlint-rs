use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum ZhScript {
    #[default]
    Simplified,
    Traditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Rules {
    /* PUNCTUATIONS */
    /// Convert these punctuations into half-width.
    /// default preset: `()`
    /// e.g. `（文字）` -> `(文字)`
    pub half_width_punctuation: String,

    /// Convert these punctuations into full-width.
    /// default preset: `，。：；？！“”‘’`
    /// e.g. `文字,文字.` -> `文字，文字。`
    pub full_width_punctuation: String,

    /// Convert traditional Chinese punctuations into simplified ones or vice versa.
    /// default preset: `simplified`
    /// e.g. `「文字」` -> `“文字”`
    pub unified_punctuation: Option<ZhScript>,

    /// Special case: skip `full_width_punctuation` for abbreviations.
    /// default preset:
    /// `['Mr.','Mrs.','Dr.','Jr.','Sr.','vs.','etc.','i.e.','e.g.','a.k.a']`
    pub skip_abbrs: Vec<String>,

    /* SPACES AROUND LETTERS */
    // default preset: `true`
    // - `true`: one space
    // - `false`: do nothing
    // e.g. `foo  bar` -> `foo bar`
    pub space_between_half_width_letters: bool,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文 字` -> `文字`
    pub no_space_between_full_width_letters: bool,

    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. `文字 foo文字` -> `文字 foo 文字` (`true`)
    /// e.g. `文字foo 文字` -> `文字foo文字` (`false`)
    pub space_between_mixed_width_letters: Option<bool>,

    /// Special case: skip `space_between_mixed_width_letters`
    /// for numbers x Chinese units.
    /// default preset: `['年','月','日','天','号','时','分','秒']`
    pub skip_zh_units: Vec<char>,

    /* SPACES AROUND PUNCTUATIONS */
    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字 ，文字` -> `文字，文字`
    pub no_space_before_punctuation: bool,

    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. `文字,文字` -> `文字, 文字` (`true`)
    /// e.g. `文字, 文字` -> `文字,文字` (`false`)
    pub space_after_half_width_punctuation: Option<bool>,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字， 文字` -> `文字，文字`
    pub no_space_after_full_width_punctuation: bool,

    /* SPACES AROUND QUOTES */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. `文字 "文字"文字` -> `文字 "文字" 文字` (`true`)
    /// e.g. `文字"文字" 文字` -> `文字"文字"文字` (`false`)
    pub space_outside_half_quote: Option<bool>,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字 “文字” 文字` -> `文字“文字”文字`
    pub no_space_outside_full_quote: bool,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字“ 文字 ”文字` -> `文字“文字”文字`
    pub no_space_inside_quote: bool,

    /* SPACES AROUND BRACKETS */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    pub space_outside_half_bracket: Option<bool>,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    pub no_space_outside_full_bracket: bool,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    pub no_space_inside_bracket: bool,

    /* SPACES AROUND CODE */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. '文字 `code`文字' -> '文字 `code` 文字' ('true')
    /// e.g. '文字`code` 文字' -> '文字`code`文字' ('false')
    pub space_outside_code: Option<bool>,

    /* SPACES AROUND MARKDOWN/HTML WRAPPERS */
    // default `true`
    // - `true`: zero space
    // - `undefined`: do nothing
    // e.g. `文字** foo **文字` -> `文字 **foo** 文字`
    pub no_space_inside_wrapper: bool,
    // /* SPACES AT THE BEGINNING/END */
    // /// default `true`
    // /// e.g. ` 文字 ` -> `文字`
    // pub trim_space: bool,
}

impl Rules {
    pub fn empty() -> Self {
        Self {
            half_width_punctuation: String::new(),
            full_width_punctuation: String::new(),
            unified_punctuation: None,
            skip_abbrs: Vec::new(),
            space_between_half_width_letters: false,
            no_space_between_full_width_letters: false,
            space_between_mixed_width_letters: None,
            skip_zh_units: Vec::new(),
            no_space_before_punctuation: false,
            space_after_half_width_punctuation: None,
            no_space_after_full_width_punctuation: false,
            space_outside_half_quote: None,
            no_space_outside_full_quote: false,
            no_space_inside_quote: false,
            space_outside_half_bracket: None,
            no_space_outside_full_bracket: false,
            no_space_inside_bracket: false,
            space_outside_code: None,
            no_space_inside_wrapper: false,
        }
    }
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            half_width_punctuation: "()".to_string(),
            full_width_punctuation: "，。：；？！“”‘’".to_string(),
            unified_punctuation: Some(ZhScript::Simplified),
            skip_abbrs: vec![
                "Mr.".to_string(),
                "Mrs.".to_string(),
                "Dr.".to_string(),
                "Jr.".to_string(),
                "Sr.".to_string(),
                "vs.".to_string(),
                "etc.".to_string(),
                "i.e.".to_string(),
                "e.g.".to_string(),
                "a.k.a".to_string(),
            ],
            space_between_half_width_letters: true,
            no_space_between_full_width_letters: true,
            space_between_mixed_width_letters: Some(true),
            skip_zh_units: vec!['年', '月', '日', '天', '号', '时', '分', '秒'],
            no_space_before_punctuation: true,
            space_after_half_width_punctuation: Some(true),
            no_space_after_full_width_punctuation: true,
            space_outside_half_quote: Some(true),
            no_space_outside_full_quote: true,
            no_space_inside_quote: true,
            space_outside_half_bracket: Some(true),
            no_space_outside_full_bracket: true,
            no_space_inside_bracket: true,
            space_outside_code: Some(true),
            no_space_inside_wrapper: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub preset: Option<String>,
    pub rules: Rules,
    pub ignores: Vec<String>,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            preset: None,
            rules: Rules::empty(),
            ignores: Vec::new(),
        }
    }
}

impl From<Rules> for Config {
    fn from(value: Rules) -> Self {
        Config {
            preset: None,
            rules: value,
            ignores: Vec::new(),
        }
    }
}
