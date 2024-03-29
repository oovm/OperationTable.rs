use std::{
    borrow::Cow,
    fmt::{Debug, Display, Formatter, Write},
    mem::take,
};

mod display;

pub type Evaluator = Box<dyn Fn(usize, usize) -> Option<usize>>;
pub type Alphabet = fn(usize) -> Option<char>;

pub struct TableDisplay {
    pub sign: Cow<'static, str>,
    pub x_terms: Vec<usize>,
    pub y_terms: Vec<usize>,
    pub display_base: usize,
    pub show_base: bool,
    pub evaluate: Evaluator,
    pub alphabet: Alphabet,
}

impl Default for TableDisplay {
    fn default() -> Self {
        Self {
            display_base: 10,
            sign: Cow::Borrowed("×"),
            x_terms: vec![2, 4, 6, 8],
            y_terms: vec![1, 3, 5, 7, 9],
            evaluate: Box::new(|x, y| Some(x * y)),
            alphabet: standard_alphabet,
            show_base: true,
        }
    }
}

pub fn standard_alphabet(index: usize) -> Option<char> {
    if index < 10 {
        Some((index as u8 + b'0') as char)
    }
    else if index < 36 {
        Some((index as u8 - 10 + b'a') as char)
    }
    else if index < 62 {
        Some((index as u8 - 36 + b'A') as char)
    }
    else {
        None
    }
}
