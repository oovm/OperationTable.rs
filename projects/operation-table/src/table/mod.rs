use std::borrow::Cow;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug)]
pub struct TableDisplay {
    pub shared_buffer: String,
    pub sign: Cow<'static, str>,
    pub x_terms: Vec<usize>,
    pub y_terms: Vec<usize>,
    pub display_base: usize,
    pub show_base: bool,
    pub evaluate: fn(usize, usize) -> Option<usize>,
    pub alphabet: fn(usize) -> Option<char>,
}

impl Default for TableDisplay {
    fn default() -> Self {
        Self {
            shared_buffer: "".to_string(),
            display_base: 10,
            sign: Cow::Borrowed("×"),
            x_terms: vec![2, 4, 6, 8],
            y_terms: vec![1, 3, 5, 7, 9],
            evaluate: |x, y| { Some(x * y) },
            alphabet: standard_alphabet,
            show_base: true,
        }
    }
}

impl TableDisplay {
    fn write_number_radix(&self, number: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut digits = Vec::new();
        let mut number = number;
        while number > 0 {
            digits.push(number % self.display_base);
            number /= self.display_base;
        }
        digits.reverse();
        for digit in digits {
            match (self.alphabet)(digit) {
                Some(c) => f.write_char(c)?,
                None => f.write_str("?")?,
            }
        }
        Ok(())
    }
}

pub fn standard_alphabet(index: usize) -> Option<char> {
    if index < 10 {
        Some((index as u8 + b'0') as char)
    } else if index < 36 {
        Some((index as u8 - 10 + b'a') as char)
    } else if index < 62 {
        Some((index as u8 - 36 + b'A') as char)
    } else {
        None
    }
}


impl Display for TableDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("\\begin{array}{")?;
        for _ in 0..self.x_terms.len() {
            f.write_str("|c")?;
        }
        f.write_str("|}")?;
        // main table
        f.write_str("\n\\hline\n")?;
        f.write_fmt(format_args!("{}_{{({})}}", self.sign, self.display_base))?;
        f.write_str("&")?;
        for i in &self.x_terms {
            f.write_fmt(format_args!("{}", i))?;
            if *i != self.x_terms.len() - 1 {
                f.write_str("&")?;
            } else {
                f.write_str("\\\\")?;
            }
        }
        for line in &self.y_terms {
            f.write_str("\n\\hline\n")?;
            f.write_fmt(format_args!("{}&", line))?;
            for column in &self.x_terms {
                match (self.evaluate)(*line, *column) {
                    Some(value) => {
                        self.write_number_radix(value, f)
                    }
                    None => {
                        f.write_str("")
                    }
                }?;
                if *column != self.y_terms.len() - 1 {
                    f.write_str("&")?;
                } else {
                    f.write_str("\\\\")?;
                }
            }
        }
        f.write_str("\n\\hline\n")?;
        f.write_str("\\end{array}")
    }
}

#[test]
fn test() {
    println!("$$");
    let m = TableDisplay::default();
    println!("{}", m);
    println!("$$");
}
