use super::*;
use latexify::Latexify;
use std::fmt::Write;

impl OperationTable {
    pub fn display(&self) -> TableDisplay {
        let mut y_terms = Vec::new();
        for x in self.range_min..self.base {
            y_terms.push(x);
        }
        TableDisplay {
            sign: Cow::Owned(self.kind.to_string()),
            x_terms: y_terms.clone(),
            y_terms,
            evaluate: self.kind.apply(self.hide_upper_triangle, self.base),
            display_base: self.base_display,
            alphabet: standard_alphabet,
            show_base: self.show_base,
        }
    }
}

impl OperationKind {
    pub fn apply(&self, hide_upper_triangle: bool, base: usize) -> Box<dyn Fn(usize, usize) -> Option<usize>> {
        match self {
            Self::Enumerate => Box::new(move |x, y| Some(x * base + y)),
            Self::Addition => Box::new(move |x, y| if hide_upper_triangle && x < y { None } else { Some(x + y) }),
            Self::Multiplication => Box::new(move |x, y| if hide_upper_triangle && x < y { None } else { Some(x * y) }),
        }
    }
}

impl Display for OperationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enumerate => f.write_str("{}"),
            Self::Addition => f.write_str("+"),
            Self::Multiplication => f.write_str("×"),
        }
    }
}

impl Latexify for OperationTable {
    type Context = ();

    fn fmt<W: Write>(&self, _: &Self::Context, f: &mut W) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.display()))
    }
}
