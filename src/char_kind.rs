use unicode_width::UnicodeWidthChar;

use crate::parser::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharKind {
    Space,
    LettersHalf,
    LettersFull,
    PunctuationHalf,
    PunctuationFull,
    Unknown,
}

pub trait CharKindTrait {
    fn kind(&self) -> CharKind;

    fn is_letters(&self) -> bool {
        matches!(self.kind(), CharKind::LettersHalf | CharKind::LettersFull)
    }

    fn is_half_width(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::LettersHalf | CharKind::PunctuationHalf
        )
    }

    fn is_full_width(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::LettersFull | CharKind::PunctuationFull
        )
    }

    fn is_punctuation(&self) -> bool {
        matches!(
            self.kind(),
            CharKind::PunctuationHalf | CharKind::PunctuationFull
        )
    }

    fn is_left_bracket(&self) -> bool;
    fn is_right_bracket(&self) -> bool;
    fn is_bracket(&self) -> bool {
        self.is_left_bracket() || self.is_right_bracket()
    }

    fn is_half_width_quote(&self) -> bool;
    fn is_left_quote(&self) -> bool;
    fn is_right_quote(&self) -> bool;
    fn is_quote(&self) -> bool {
        self.is_half_width_quote() || self.is_left_quote() || self.is_right_quote()
    }

    fn is_common_punctuation(&self) -> bool;

    fn to_half_width(&self) -> char;

    fn to_full_width(&self) -> char;
}

impl CharKindTrait for char {
    fn kind(&self) -> CharKind {
        if self.is_whitespace() {
            CharKind::Space
        } else if self.is_ascii_punctuation() {
            CharKind::PunctuationHalf
        } else if "，。、；：？！…—～｜·‘’“”《》【】「」（）".contains(*self)
            || ('\u{003000}'..='\u{00303F}').contains(self)
        {
            CharKind::PunctuationFull
        } else {
            match self.width() {
                Some(1) => CharKind::LettersHalf,
                Some(2) => CharKind::LettersFull,
                _ => CharKind::Unknown,
            }
        }
    }

    fn is_left_bracket(&self) -> bool {
        matches!(self, '(' | '（')
    }

    fn is_right_bracket(&self) -> bool {
        matches!(self, ')' | '）')
    }

    fn is_half_width_quote(&self) -> bool {
        matches!(self, '"')
    }

    fn is_left_quote(&self) -> bool {
        matches!(self, '“' | '‘' | '「' | '『')
    }

    fn is_right_quote(&self) -> bool {
        matches!(self, '”' | '’' | '」' | '』')
    }

    fn is_common_punctuation(&self) -> bool {
        (self.kind() == CharKind::PunctuationFull || "!,.;?".contains(*self))
            && !self.is_bracket()
            && !self.is_quote()
    }

    fn to_half_width(&self) -> char {
        if self.kind() == CharKind::PunctuationFull {
            match self {
                '（' => '(',
                '）' => ')',
                '，' => ',',
                '。' => '.',
                '；' => ';',
                '：' => ':',
                '？' => '?',
                '！' => '!',
                '“' | '”' => '"',
                '‘' | '’' => '\'',
                _ => *self,
            }
        } else {
            *self
        }
    }

    fn to_full_width(&self) -> char {
        if self.kind() == CharKind::PunctuationHalf {
            match self {
                '(' => '（',
                ')' => '）',
                ',' => '，',
                '.' => '。',
                ';' => '；',
                ':' => '：',
                '?' => '？',
                '!' => '！',
                _ => *self,
            }
        } else {
            *self
        }
    }
}

impl CharKindTrait for Token<'_> {
    fn kind(&self) -> CharKind {
        if let Token::Char(c) = self {
            c.kind()
        } else {
            CharKind::Unknown
        }
    }

    fn is_left_bracket(&self) -> bool {
        if let Token::Char(c) = self {
            c.is_left_bracket()
        } else {
            false
        }
    }

    fn is_right_bracket(&self) -> bool {
        if let Token::Char(c) = self {
            c.is_right_bracket()
        } else {
            false
        }
    }

    fn is_half_width_quote(&self) -> bool {
        if let Token::Char(c) = self {
            c.is_half_width_quote()
        } else {
            false
        }
    }

    fn is_left_quote(&self) -> bool {
        if let Token::Char(c) = self {
            c.is_left_quote()
        } else {
            false
        }
    }

    fn is_right_quote(&self) -> bool {
        if let Token::Char(c) = self {
            c.is_right_quote()
        } else {
            false
        }
    }

    fn is_common_punctuation(&self) -> bool {
        if let Token::Char(c) = self {
            c.is_common_punctuation()
        } else {
            false
        }
    }

    fn to_half_width(&self) -> char {
        if let Token::Char(c) = self {
            c.to_half_width()
        } else {
            '\0'
        }
    }

    fn to_full_width(&self) -> char {
        if let Token::Char(c) = self {
            c.to_full_width()
        } else {
            '\0'
        }
    }
}
