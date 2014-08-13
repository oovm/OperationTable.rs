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
        // align part
        f.write_str("\\begin{array}{")?;
        for _ in 0..=self.x_terms.len() {
            f.write_str("|c")?;
        }
        f.write_str("|}")?;
        // first line
        f.write_str("\n\\hline\n")?;
        f.write_str(&self.sign)?;
        if self.show_base {
            f.write_fmt(format_args!("_{{({})}}", self.display_base))?;
        }
        f.write_str("&")?;
        for (column, i) in self.x_terms.iter().enumerate() {
            f.write_fmt(format_args!("{}", i))?;
            self.write_split(f, column)?;
        }
        // main table
        for line in &self.y_terms {
            f.write_str("\n\\hline\n")?;
            f.write_fmt(format_args!("{}&", line))?;
            for (column, i) in self.x_terms.iter().enumerate() {
                match (self.evaluate)(*line, *i) {
                    Some(value) => {
                        self.write_number_radix(f, &mut digits_buffer, value)?;
                        self.write_split(f, column)?;
                    }
                    None => {
                        self.write_split(f, column)?;
                    }
                }
            }
        }
        f.write_str("\n\\hline\n")?;
        f.write_str("\\end{array}")
    }
}

impl TableDisplay {
    fn write_number_radix(&self, f: &mut Formatter<'_>, digits_buffer: &mut String, mut number: usize) -> std::fmt::Result {
        if number == 0 {
            f.write_str("0")?
        }
        else {
            while number > 0 {
                let digit = number % self.display_base;
                number /= self.display_base;
                match (self.alphabet)(digit) {
                    Some(c) => digits_buffer.push(c),
                    None => digits_buffer.push('?'),
                }
            }
            for c in take(digits_buffer).chars().rev() {
                f.write_char(c)?;
            }
        }
        Ok(())
    }
    fn write_split(&self, f: &mut Formatter<'_>, index: usize) -> std::fmt::Result {
        if self.x_terms.len() != index + 1 { f.write_str("&") } else { f.write_str("\\\\") }
    }
}

#[test]
fn test() {
    println!("$$");
    let m = TableDisplay::default();
    println!("{}", m);
    println!("$$");
}
