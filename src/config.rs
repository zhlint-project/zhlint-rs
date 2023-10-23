use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum ZhScript {
    #[default]
    Simplified,
    Traditional,
}

const fn bool_true() -> bool {
    true
}

const fn some_true() -> Option<bool> {
    Some(true)
}

fn half_width_punctuation() -> String {
    "()".to_string()
}

fn full_width_punctuation() -> String {
    "，。：；？！“”‘’".to_string()
}

fn unified_punctuation() -> Option<ZhScript> {
    Some(ZhScript::Simplified)
}

fn front_matter_delimiter() -> String {
    "---".to_string()
}

fn skip_abbrs() -> Vec<String> {
    vec![
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
    ]
}

fn skip_zh_units() -> Vec<char> {
    vec!['年', '月', '日', '天', '号', '时', '分', '秒']
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /* PUNCTUATIONS */
    /// Convert these punctuations into half-width.
    /// default preset: `()`
    /// e.g. `（文字）` -> `(文字)`
    #[serde(default = "half_width_punctuation")]
    pub half_width_punctuation: String,

    /// Convert these punctuations into full-width.
    /// default preset: `，。：；？！“”‘’`
    /// e.g. `文字,文字.` -> `文字，文字。`
    #[serde(default = "full_width_punctuation")]
    pub full_width_punctuation: String,

    /// Convert traditional Chinese punctuations into simplified ones or vice versa.
    /// default preset: `simplified`
    /// e.g. `「文字」` -> `“文字”`
    #[serde(default = "unified_punctuation")]
    pub unified_punctuation: Option<ZhScript>,

    /// Special case: skip `full_width_punctuation` for abbreviations.
    /// default preset:
    /// `['Mr.','Mrs.','Dr.','Jr.','Sr.','vs.','etc.','i.e.','e.g.','a.k.a']`
    #[serde(default = "skip_abbrs")]
    pub skip_abbrs: Vec<String>,

    /* SPACES AROUND LETTERS */
    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文 字` -> `文字`
    #[serde(default = "bool_true")]
    pub no_space_between_full_width_letters: bool,

    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. `文字 foo文字` -> `文字 foo 文字` (`true`)
    /// e.g. `文字foo 文字` -> `文字foo文字` (`false`)
    #[serde(default = "some_true")]
    pub space_between_mixed_width_letters: Option<bool>,

    /// Special case: skip `space_between_mixed_width_letters`
    /// for numbers x Chinese units.
    /// default preset: `['年','月','日','天','号','时','分','秒']`
    #[serde(default = "skip_zh_units")]
    pub skip_zh_units: Vec<char>,

    /* SPACES AROUND PUNCTUATIONS */
    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字 ，文字` -> `文字，文字`
    #[serde(default = "bool_true")]
    pub no_space_before_punctuation: bool,

    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. `文字,文字` -> `文字, 文字` (`true`)
    /// e.g. `文字, 文字` -> `文字,文字` (`false`)
    #[serde(default = "some_true")]
    pub space_after_half_width_punctuation: Option<bool>,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字， 文字` -> `文字，文字`
    #[serde(default = "bool_true")]
    pub no_space_after_full_width_punctuation: bool,

    /* SPACES AROUND QUOTES */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. `文字 "文字"文字` -> `文字 "文字" 文字` (`true`)
    /// e.g. `文字"文字" 文字` -> `文字"文字"文字` (`false`)
    #[serde(default = "some_true")]
    pub space_outside_half_quote: Option<bool>,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字 “文字” 文字` -> `文字“文字”文字`
    #[serde(default = "bool_true")]
    pub no_space_outside_full_quote: bool,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字“ 文字 ”文字` -> `文字“文字”文字`
    #[serde(default = "bool_true")]
    pub no_space_inside_quote: bool,

    /* SPACES AROUND BRACKETS */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    #[serde(default = "some_true")]
    pub space_outside_half_bracket: Option<bool>,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    #[serde(default = "bool_true")]
    pub no_space_outside_full_bracket: bool,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    #[serde(default = "bool_true")]
    pub no_space_inside_bracket: bool,

    /* SPACES AROUND CODE */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. '文字 `code`文字' -> '文字 `code` 文字' ('true')
    /// e.g. '文字`code` 文字' -> '文字`code`文字' ('false')
    #[serde(default = "some_true")]
    pub space_outside_code: Option<bool>,

    /* SPACES AROUND LINK */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. '文字 [link](#link)文字' -> '文字 [link](#link) 文字' ('true')
    /// e.g. '文字[link](#link) 文字' -> '文字[link](#link)文字' ('false')
    #[serde(default = "some_true")]
    pub space_outside_link: Option<bool>,

    /* SPACES AT THE BEGINNING/END */
    /// default `true`
    /// e.g. ` 文字 ` -> `文字`
    #[serde(default = "bool_true")]
    pub trim_space: bool,

    #[serde(default = "front_matter_delimiter")]
    pub front_matter_delimiter: String,

    #[serde(default = "Vec::new")]
    pub ignore: Vec<String>,
}
