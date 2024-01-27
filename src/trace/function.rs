use super::{block::BasicBlock, Error, Instruction, JumpKind};
use std::collections::HashMap;

pub struct Function {
    /// The name of the function which follows a `sub_[address]` format with a given starting
    /// address.
    ///
    /// The only exception to this rule is the `start` function.
    pub(crate) name: String,
    /// The starting address of the function.
    pub(crate) start: u64,
    /// All of the basic blocks which make up the function.
    pub(crate) blocks: HashMap<u64, BasicBlock>,
    /// The current block which the function is at.
    ///
    /// Interally used for constructing the function as its executed.
    current_block: u64,
}

impl Function {
    /// Creates a function following the `sub_[address]` format with the given starting address.
    pub fn new(address: u64) -> Self {
        Function {
            name: format!("sub_{address}"),
            start: address,
            blocks: HashMap::from([(address, BasicBlock::new(address))]),
            current_block: address,
        }
    }

    pub fn execute(
        &mut self,
        insn: &Instruction,
        next_insn: Option<&&Instruction>,
    ) -> Result<(), Error> {
        let current_block = self
            .blocks
            .get_mut(&self.current_block)
            .ok_or(Error::MissingCurrentBlock)?;

        if current_block.block.contains_key(&insn.addr) {
            current_block.execute(insn);
        }

        let jump_type = &insn.kind;

        if let Some(jump_type) = jump_type {
            // HANDLE ERROR
            let next_insn = next_insn.unwrap();

            match jump_type {
                JumpKind::Conditional => {
                    current_block.add_edge(next_insn.addr, *jump_type);

                    self.blocks
                        .entry(next_insn.addr)
                        .or_insert(BasicBlock::new(next_insn.addr));
                }
                JumpKind::Unconditional => {
                    current_block.add_edge(next_insn.addr, *jump_type);

                    self.blocks
                        .entry(next_insn.addr)
                        .or_insert(BasicBlock::new(next_insn.addr));
                }
                _ => {}
            }
        }

        Ok(())
    }
}
