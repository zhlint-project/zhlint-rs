use unicode_width::UnicodeWidthChar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharKind {
    Space,
    LettersHalf,
    LettersFull,
    PunctuationHalf,
    PunctuationFull,
    Unknown,
}

pub struct Cursor<'a> {
    tokens: &'a mut Vec<char>,
    index: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(tokens: &'a mut Vec<char>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn advance(self) -> Option<Self> {
        if self.index + 1 < self.tokens.len() {
            Some(Self {
                tokens: self.tokens,
                index: self.index + 1,
            })
        } else {
            None
        }
    }

    pub fn skip_str(&mut self, to_skip: &Vec<String>) {
        for s in to_skip {
            if s.chars()
                .enumerate()
                .all(|(i, c)| Some(&c) == self.tokens.get(self.index + i))
            {
                self.index += s.len();
                if self.current().is_whitespace() {
                    self.index += 1;
                }
                break;
            }
        }
    }

    pub fn set(&mut self, to: char) {
        self.tokens[self.index] = to;
    }

    pub fn remove(&mut self) {
        self.tokens.remove(self.index);
        self.index -= 1;
    }

    pub fn add(&mut self, c: char) {
        self.tokens.insert(self.index, c)
    }

    pub fn current(&self) -> char {
        self.tokens[self.index]
    }

    pub fn previous(&self) -> char {
        if self.index == 0 {
            return '\0';
        }
        self.tokens[self.index - 1]
    }

    pub fn previous_two(&self) -> char {
        if self.index < 2 {
            return '\0';
        }
        self.tokens[self.index - 2]
    }

    pub fn previous_skip_space(&self) -> char {
        let mut i = self.index;
        loop {
            if i == 0 {
                break '\0';
            }
            i -= 1;
            if self.tokens[i].kind() != CharKind::Space {
                break self.tokens[i];
            }
        }
    }

    pub fn search_previous<Target: Fn(char) -> bool, Stop: Fn(char) -> bool>(
        &self,
        target: Target,
        stop: Stop,
    ) -> bool {
        let mut i = self.index;
        loop {
            if i == 0 {
                break false;
            }
            i -= 1;
            if target(self.tokens[i]) {
                break true;
            }
            if stop(self.tokens[i]) {
                break false;
            }
        }
    }

    pub fn count_previous<F: Fn(char) -> bool>(&self, target: F) -> u32 {
        let mut i = self.index;
        let mut count = 0;
        loop {
            if i == 0 {
                break count;
            }
            i -= 1;
            if target(self.tokens[i]) {
                count += 1;
            }
        }
    }

    pub fn next(&self) -> char {
        if self.index + 1 == self.tokens.len() {
            return '\0';
        }
        self.tokens[self.index + 1]
    }

    pub fn next_skip_space(&self) -> char {
        let mut i = self.index;
        loop {
            i += 1;
            if i == self.tokens.len() {
                break '\0';
            }
            if self.tokens[i].kind() != CharKind::Space {
                break self.tokens[i];
            }
        }
    }
}

pub fn tokenize(text: &str) -> impl Iterator<Item = char> + '_ {
    let mut chars = text.chars();
    std::iter::from_fn(move || {
        if let Some(c) = chars.next() {
            if c.is_whitespace() {
                while chars.clone().next()?.is_whitespace() {
                    chars.next();
                }
                Some(' ')
            } else {
                Some(c)
            }
        } else {
            None
        }
    })
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
