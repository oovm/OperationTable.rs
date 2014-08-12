use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use crate::table::{TableDisplay};

#[derive(Copy, Clone, Debug)]
pub struct OperationTable {
    pub kind: OperationKind,
    pub base: usize,
    pub base_display: u32,
    pub skip_one: bool,
    pub hide_upper_triangle: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum OperationKind {
    Addition,
    #[default]
    Multiplication,
}


impl Display for OperationTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display = TableDisplay {
            sign: Cow::Owned(self.kind.to_string()),
            x_terms: (0..self.base).collect(),
            y_terms: (0..self.base).collect(),
            hide_upper_triangle: self.hide_upper_triangle,
            display_base: self.base_display as usize,
        };
        Display::fmt(&display, f)
    }
}


impl OperationTable {
    pub fn with_base(self, base: usize) -> Self {
        Self { base, ..self }
    }
    pub fn with_display(self, base: u32) -> Self {
        Self { base_display: base, ..self }
    }
    pub fn with_operation(self, kind: OperationKind) -> Self {
        Self { kind, ..self }
    }
}

impl Default for OperationTable {
    fn default() -> Self {
        Self { kind: OperationKind::default(), base: 10, base_display: 10, skip_one: true, hide_upper_triangle: true }
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

impl OperationKind {
    pub fn apply(&self, a: u32, b: u32) -> u32 {
        match self {
            Self::Addition => a + b,
            Self::Multiplication => a * b,
        }
    }
    pub(crate) fn write_sign(&self, display: u32, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition => f.write_str("+"),
            Self::Multiplication => f.write_str("×"),
        }?;
        f.write_fmt(format_args!("_{{({})}}", display))
    }
}

#[test]
fn test() {
    println!("$$");
    let m = OperationTable::default().with_base(7).with_operation(OperationKind::Addition);
    println!("{}", m);
    println!("$$");
    println!("$$");
    let m = OperationTable::default().with_base(7);
    println!("{}", m);
    println!("$$");
}
