use super::*;

impl Debug for TableDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableDisplay")
            .field("sign", &self.sign)
            .field("x_terms", &self.x_terms)
            .field("y_terms", &self.y_terms)
            .field("display_base", &self.display_base)
            .field("show_base", &self.show_base)
            .field("evaluate", &"Evaluator")
            .field("alphabet", &"fn(usize) -> Option<char>")
            .finish()
    }
}

impl Display for TableDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut digits_buffer = String::with_capacity(1);

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
                        self.write_number_radix(f, &mut digits_buffer, value)
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
