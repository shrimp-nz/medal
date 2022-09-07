use itertools::Itertools;

use crate::{LocalRw, RValue, RcLocal, SideEffects, Traverse};

#[derive(Debug, Clone, PartialEq)]
pub struct SetList {
    pub table: RcLocal,
    pub index: usize,
    pub values: Vec<RValue>,
    pub tail: Option<RValue>,
}

impl SetList {
    pub fn new(table: RcLocal, index: usize, values: Vec<RValue>, tail: Option<RValue>) -> Self {
        Self {
            table,
            index,
            values,
            tail,
        }
    }
}

impl LocalRw for SetList {
    fn values_read(&self) -> Vec<&RcLocal> {
        let tail_locals = self
            .tail
            .as_ref()
            .map(|t| t.values_read())
            .unwrap_or_default();
        std::iter::once(&self.table)
            .chain(self.values.iter().flat_map(|rvalue| rvalue.values_read()))
            .chain(tail_locals)
            .collect()
    }

    fn values_read_mut(&mut self) -> Vec<&mut RcLocal> {
        let tail_locals = self
            .tail
            .as_mut()
            .map(|t| t.values_read_mut())
            .unwrap_or_default();
        std::iter::once(&mut self.table)
            .chain(
                self.values
                    .iter_mut()
                    .flat_map(|rvalue| rvalue.values_read_mut()),
            )
            .chain(tail_locals)
            .collect()
    }
}

impl SideEffects for SetList {
    fn has_side_effects(&self) -> bool {
        self.values
            .iter()
            .chain(self.tail.as_ref())
            .any(|rvalue| rvalue.has_side_effects())
    }
}

impl Traverse for SetList {
    fn rvalues(&self) -> Vec<&RValue> {
        self.values.iter().chain(self.tail.as_ref()).collect()
    }

    fn rvalues_mut(&mut self) -> Vec<&mut RValue> {
        self.values.iter_mut().chain(self.tail.as_mut()).collect()
    }
}

impl std::fmt::Display for SetList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "__set_list({}, {}, {{{}}})",
            self.table,
            self.index,
            self.values.iter().chain(self.tail.as_ref()).join(", ")
        )
    }
}
