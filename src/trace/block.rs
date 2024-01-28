use super::{Instruction, JumpKind};
use std::collections::BTreeMap;

pub struct BasicBlock {
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) block: BTreeMap<u64, String>,
    // The first item indicates if it is Unconditional or Conditional, the second is the address,
    // the third is how many traverses
    edges: Vec<(JumpKind, u64, u64)>,
}

impl BasicBlock {
    /// Generates a new BasicBlock with a given start address
    pub fn new(start: u64) -> Self {
        BasicBlock {
            start,
            end: start,
            block: BTreeMap::new(),
            edges: Vec::new(),
        }
    }

    /// Appends an instruction to the basic block and sets the end address to the one given.
    pub fn execute(&mut self, insn: &Instruction) {
        let _ = self.block.insert(insn.addr, insn.insn.clone());

        if let Some(size) = insn.size {
            self.end += insn.addr + size as u64
        } else {
            self.end = insn.addr
        }
    }

    /// Get a string representation of the block.
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for (address, instruction) in &self.block {
            output.push_str(&format!("0x{address:X}\t{0}", instruction));
            output.push_str("\\n");
        }
        output
    }

    /// Checks if the given address is **currently** in the range of this basic block.
    pub fn in_range(&self, address: u64) -> bool {
        self.start >= address && address <= self.end
    }

    /// Returns an iterator of the address/instruction pairs inside the underlying HashMap.
    pub fn iter(&self) -> impl Iterator<Item = (&u64, &String)> {
        self.block.iter()
    }

    /// Returns an iterator of the edges/count pairs inside the underlying Vector.
    pub fn edges(&self) -> impl Iterator<Item = &(JumpKind, u64, u64)> {
        self.edges.iter()
    }

    /// Identifies if a given edge already exists for the block.
    pub fn contains_edge(&self, edge: u64) -> bool {
        self.edges.iter().any(|(_, e, _)| *e == edge)
    }

    /// Adds a new edge if it cannot find it, otherwise increments the edge counter depending on if it was traversed or not.
    pub fn add_edge(&mut self, edge: u64, kind: JumpKind) {
        if let Some((_, _, count)) = self.edges.iter_mut().find(|(_, e, _)| *e == edge) {
            *count += 1;
        } else {
            self.edges.push((kind, edge, 1));
        }
    }
}
