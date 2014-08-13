use super::*;

impl OperationTable {
    pub fn display(&self) -> TableDisplay {
        let mut x_terms = Vec::new();
        let mut y_terms = Vec::new();
        for x in 0..self.base {
            if self.skip_one && x == 1 {
                continue;
            }
            x_terms.push(x);
        }
        for y in 0..self.base {
            if self.skip_one && y == 1 {
                continue;
            }
            y_terms.push(y);
        }
        TableDisplay {
            sign: Cow::Owned(self.kind.to_string()),
            x_terms,
            y_terms,
            evaluate: self.kind.apply(self.hide_upper_triangle),
            display_base: self.base_display as usize,
            alphabet: standard_alphabet,
            show_base: true,
        }
    }
}

impl Display for OperationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition => f.write_str("+"),
            Self::Multiplication => f.write_str("×"),
        }
    }
}
