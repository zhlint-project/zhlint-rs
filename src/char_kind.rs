use icu_properties::{maps, EastAsianWidth, GeneralCategory, GeneralCategoryGroup};

pub static CHINESE_PAUSE_STOP_PUNCTUATION: [char; 10] = [
    '\u{3002}', // 。    IDEOGRAPHIC FULL STOP
    '\u{FF0E}', // ．    FULLWIDTH FULL STOP
    '\u{FF0C}', // ，    FULLWIDTH COMMA
    '\u{3001}', // 、    IDEOGRAPHIC COMMA
    '\u{FF1A}', // ：    FULLWIDTH COLON
    '\u{FF1B}', // ；    FULLWIDTH SEMICOLON
    '\u{FF01}', // ！    FULLWIDTH EXCLAMATION MARK
    '\u{203C}', // ‼    DOUBLE EXCLAMATION MARK
    '\u{FF1F}', // ？    FULLWIDTH QUESTION MARK
    '\u{2047}', // ⁇    DOUBLE QUESTION MARK
];

pub static CHINESE_LEFT_QUOTATION_PUNCTUATION: [char; 4] = [
    '\u{300C}', // 「    LEFT CORNER BRACKET
    '\u{300E}', // 『    LEFT WHITE CORNER BRACKET
    '\u{201C}', // “    LEFT DOUBLE QUOTATION MARK
    '\u{2018}', // ‘    LEFT SINGLE QUOTATION MARK
];

pub static CHINESE_RIGHT_QUOTATION_PUNCTUATION: [char; 4] = [
    '\u{300D}', // 」    RIGHT CORNER BRACKET
    '\u{300F}', // 』    RIGHT WHITE CORNER BRACKET
    '\u{201D}', // ”    RIGHT DOUBLE QUOTATION MARK
    '\u{2019}', // ’    RIGHT SINGLE QUOTATION MARK
];

pub static CHINESE_LEFT_BRACKET_PUNCTUATION: [char; 8] = [
    '\u{FF08}', // （    FULLWIDTH LEFT PARENTHESIS
    '\u{300A}', // 《    LEFT DOUBLE ANGLE BRACKET
    '\u{3008}', // 〈    LEFT ANGLE BRACKET
    '\u{3010}', // 【    LEFT BLACK LENTICULAR BRACKET
    '\u{3016}', // 〖    LEFT WHITE LENTICULAR BRACKET
    '\u{3014}', // 〔    LEFT TORTOISE SHELL BRACKET
    '\u{FF3B}', // ［    FULLWIDTH LEFT SQUARE BRACKET
    '\u{FF5B}', // ｛    FULLWIDTH LEFT CURLY BRACKET
];

pub static CHINESE_RIGHT_BRACKET_PUNCTUATION: [char; 8] = [
    '\u{FF09}', // ）    FULLWIDTH RIGHT PARENTHESIS
    '\u{300B}', // 》    RIGHT DOUBLE ANGLE BRACKET
    '\u{3009}', // 〉    RIGHT ANGLE BRACKET
    '\u{3011}', // 】    RIGHT BLACK LENTICULAR BRACKET
    '\u{3017}', // 〗    RIGHT WHITE LENTICULAR BRACKET
    '\u{3015}', // 〕    RIGHT TORTOISE SHELL BRACKET
    '\u{FF3D}', // ］    FULLWIDTH RIGHT SQUARE BRACKET
    '\u{FF5D}', // ｝    FULLWIDTH RIGHT CURLY BRACKET
];

pub static CHINESE_OTHER_PUNCTUATION: [char; 12] = [
    '\u{2E3A}', // ⸺    TWO-EM DASH
    '\u{2014}', // —    EM DASH
    '\u{2026}', // …    HORIZONTAL ELLIPSIS
    '\u{22EF}', // ⋯    MIDLINE HORIZONTAL ELLIPSIS
    '\u{FF5E}', // ～    FULLWIDTH TILDE
    '\u{002D}', // -    HYPHEN-MINUS
    '\u{2013}', // –    EN DASH
    '\u{2014}', // —    EM DASH
    '\u{00B7}', // ·    MIDDLE DOT
    '\u{30FB}', // ・    KATAKANA MIDDLE DOT
    '\u{2027}', // ‧    HYPHENATION POINT
    '\u{FF0F}', // ／    FULLWIDTH SOLIDUS
];

pub static WESTERN_PAUSE_STOP_PUNCTUATION: [char; 6] = [
    '\u{002E}', // .    FULL STOP
    '\u{002C}', // ,    COMMA
    '\u{003A}', // :    COLON
    '\u{003B}', // ;    SEMICOLON
    '\u{0021}', // !    EXCLAMATION MARK
    '\u{003F}', // ?    QUESTION MARK
];

pub static WESTERN_QUOTATION_PUNCTUATION: [char; 2] = [
    '\u{0022}', // "    QUOTATION MARK
    '\u{0027}', // '    APOSTROPHE
];

pub static WESTERN_LEFT_BRACKET_PUNCTUATION: [char; 3] = [
    '\u{0028}', // (    LEFT PARENTHESIS
    '\u{005B}', // [    LEFT SQUARE BRACKET
    '\u{007B}', // {    LEFT CURLY BRACKET
];

pub static WESTERN_RIGHT_BRACKET_PUNCTUATION: [char; 3] = [
    '\u{0029}', // )    RIGHT PARENTHESIS
    '\u{005D}', // ]    RIGHT SQUARE BRACKET
    '\u{007D}', // }    RIGHT CURLY BRACKET
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharKind {
    /// 空格
    Space,
    /// 半角文字
    LetterHalf,
    /// 全角文字
    LetterFull,
    /// 中文点号
    PunctuationChinesePauseStop,
    /// 中文左引号
    PunctuationChineseLeftQuotation,
    /// 中文右引号
    PunctuationChineseRightQuotation,
    /// 中文左括号
    PunctuationChineseLeftBracket,
    /// 中文左括号
    PunctuationChineseRightBracket,
    /// 中文其他标点符号
    PunctuationChineseOther,
    /// 西文点号
    PunctuationWesternPauseStop,
    /// 西文引号
    PunctuationWesternQuotation,
    /// 西文左括号
    PunctuationWesternLeftBracket,
    /// 西文右括号
    PunctuationWesternRightBracket,
    /// 西文其他标点符号
    PunctuationWesternOther,
    /// 西文其他标点符号
    PunctuationOther,
    /// 其他字符
    Other,
}

pub trait CharKindTrait {
    fn category(&self) -> GeneralCategory;

    fn is_wide(&self) -> bool;

    fn kind(&self) -> CharKind;

    fn is_letter(&self) -> bool {
        matches!(self.kind(), CharKind::LetterHalf | CharKind::LetterFull)
    }

    fn is_space(&self) -> bool {
        matches!(self.kind(), CharKind::Space)
    }

    fn is_chinese_punctuation(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::PunctuationChinesePauseStop
                | CharKind::PunctuationChineseLeftQuotation
                | CharKind::PunctuationChineseRightQuotation
                | CharKind::PunctuationChineseLeftBracket
                | CharKind::PunctuationChineseRightBracket
                | CharKind::PunctuationChineseOther
        )
    }

    fn is_western_punctuation(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::PunctuationWesternPauseStop
                | CharKind::PunctuationWesternQuotation
                | CharKind::PunctuationWesternLeftBracket
                | CharKind::PunctuationWesternRightBracket
                | CharKind::PunctuationWesternOther
        )
    }

    fn is_punctuation(&self) -> bool {
        self.is_chinese_punctuation() || self.is_western_punctuation()
    }

    fn is_pause_stop_punctuation(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::PunctuationChinesePauseStop | CharKind::PunctuationWesternPauseStop
        )
    }

    fn is_quotation_punctuation(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::PunctuationChineseLeftQuotation
                | CharKind::PunctuationChineseRightQuotation
                | CharKind::PunctuationWesternQuotation
        )
    }

    fn is_bracket_punctuation(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::PunctuationChineseLeftBracket
                | CharKind::PunctuationChineseRightBracket
                | CharKind::PunctuationWesternLeftBracket
                | CharKind::PunctuationWesternRightBracket
        )
    }

    fn is_other_punctuation(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::PunctuationChineseOther
                | CharKind::PunctuationWesternOther
                | CharKind::PunctuationOther
        )
    }

    fn is_left_punctuation(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::PunctuationChineseLeftQuotation
                | CharKind::PunctuationChineseLeftBracket
                | CharKind::PunctuationWesternLeftBracket
        )
    }

    fn is_right_punctuation(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::PunctuationChineseRightQuotation
                | CharKind::PunctuationChineseRightBracket
                | CharKind::PunctuationWesternRightBracket
        )
    }

    fn is_single_punctuation(&self) -> bool {
        self.is_pause_stop_punctuation() || self.is_other_punctuation()
    }

    fn is_wide_or_chinese(&self) -> bool {
        self.is_wide() || self.is_chinese_punctuation()
    }
}

static BROAD_SENSE_LETTER: GeneralCategoryGroup = GeneralCategoryGroup::Letter
    .union(GeneralCategoryGroup::Number)
    .union(GeneralCategoryGroup::Symbol);

impl CharKindTrait for char {
    fn category(&self) -> GeneralCategory {
        maps::general_category().get(*self)
    }

    fn is_wide(&self) -> bool {
        matches!(
            maps::east_asian_width().get(*self),
            EastAsianWidth::Wide | EastAsianWidth::Fullwidth
        )
    }

    fn kind(&self) -> CharKind {
        if GeneralCategoryGroup::SpaceSeparator.contains(self.category()) {
            CharKind::Space
        } else if BROAD_SENSE_LETTER.contains(self.category()) {
            if self.is_wide() {
                CharKind::LetterFull
            } else {
                CharKind::LetterHalf
            }
        } else if GeneralCategoryGroup::Punctuation.contains(self.category()) {
            if CHINESE_PAUSE_STOP_PUNCTUATION.contains(self) {
                CharKind::PunctuationChinesePauseStop
            } else if CHINESE_LEFT_QUOTATION_PUNCTUATION.contains(self) {
                CharKind::PunctuationChineseLeftQuotation
            } else if CHINESE_RIGHT_QUOTATION_PUNCTUATION.contains(self) {
                CharKind::PunctuationChineseRightQuotation
            } else if CHINESE_LEFT_BRACKET_PUNCTUATION.contains(self) {
                CharKind::PunctuationChineseLeftBracket
            } else if CHINESE_RIGHT_BRACKET_PUNCTUATION.contains(self) {
                CharKind::PunctuationChineseRightBracket
            } else if CHINESE_OTHER_PUNCTUATION.contains(self) {
                CharKind::PunctuationChineseOther
            } else if WESTERN_PAUSE_STOP_PUNCTUATION.contains(self) {
                CharKind::PunctuationWesternPauseStop
            } else if WESTERN_QUOTATION_PUNCTUATION.contains(self) {
                CharKind::PunctuationWesternQuotation
            } else if WESTERN_LEFT_BRACKET_PUNCTUATION.contains(self) {
                CharKind::PunctuationWesternLeftBracket
            } else if WESTERN_RIGHT_BRACKET_PUNCTUATION.contains(self) {
                CharKind::PunctuationWesternRightBracket
            } else if self.is_ascii() {
                CharKind::PunctuationWesternOther
            } else {
                CharKind::PunctuationOther
            }
        } else {
            CharKind::Other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for c in CHINESE_PAUSE_STOP_PUNCTUATION
            .into_iter()
            .chain(CHINESE_LEFT_QUOTATION_PUNCTUATION)
            .chain(CHINESE_RIGHT_QUOTATION_PUNCTUATION)
            .chain(CHINESE_LEFT_BRACKET_PUNCTUATION)
            .chain(CHINESE_RIGHT_BRACKET_PUNCTUATION)
            .chain(CHINESE_OTHER_PUNCTUATION)
            .chain(WESTERN_PAUSE_STOP_PUNCTUATION)
            .chain(WESTERN_QUOTATION_PUNCTUATION)
            .chain(WESTERN_LEFT_BRACKET_PUNCTUATION)
            .chain(WESTERN_RIGHT_BRACKET_PUNCTUATION)
        {
            println!("{c} {:?}", c.category());
        }
    }
}
