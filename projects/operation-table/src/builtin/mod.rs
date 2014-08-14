use crate::table::{standard_alphabet, TableDisplay};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

mod display;

/// The definition of operation table
#[derive(Copy, Clone, Debug)]
pub struct OperationTable {
    pub kind: OperationKind,
    pub base: usize,
    pub show_base: bool,
    pub base_display: usize,
    pub range_min: usize,
    pub hide_upper_triangle: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum OperationKind {
    Enumerate,
    Addition,
    #[default]
    Multiplication,
}

impl Default for OperationTable {
    fn default() -> Self {
        Self {
            kind: OperationKind::default(),
            base: 10,
            show_base: true,
            base_display: 10,
            range_min: 2,
            hide_upper_triangle: true,
        }
    }
}

impl OperationTable {
    pub fn with_base(self, base: usize) -> Self {
        Self { base, ..self }
    }
    pub fn with_display(self, base: usize) -> Self {
        Self { base_display: base, ..self }
    }
    pub fn with_operation(self, kind: OperationKind) -> Self {
        Self { kind, ..self }
    }
    pub fn with_min(self, min: usize) -> Self {
        Self { range_min: min, ..self }
    }
    pub fn with_upper_triangle(self, show: bool) -> Self {
        Self { hide_upper_triangle: !show, ..self }
    }
}
