use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use num::BigUint;


#[derive(Debug)]
pub struct TableDisplay {
    pub sign: Cow<'static, str>,
    pub x_terms: Vec<usize>,
    pub y_terms: Vec<usize>,
    pub hide_upper_triangle: bool,
    pub display_base: usize,
}

impl Default for TableDisplay {
    fn default() -> Self {
        Self {
            display_base: 10,
            sign: Cow::Borrowed("×"),
            x_terms: vec![2, 4, 6, 8],
            y_terms: vec![1, 3, 5, 7, 9],
            hide_upper_triangle: true,
        }
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
                if self.hide_upper_triangle && column > line {
                    f.write_str("")?;
                } else {
                    let value = BigUint::from(line * column);
                    f.write_str(&value.to_str_radix(self.display_base as u32))?;
                }
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
