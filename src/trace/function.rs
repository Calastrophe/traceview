use super::{block::BasicBlock, Error, Instruction, JumpKind};
use std::collections::HashMap;

// NOTE: Explore `ensure_exists` being a result of a bad implementation, determine if there is a
// way to rely just on the

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

    /// Adds a given edge from source block to destination block.
    pub fn add_edge(&mut self, src: u64, dest: u64) -> Result<(), Error> {
        Ok(self
            .blocks
            .get_mut(&src)
            .ok_or(Error::MissingBlock)?
            .add_edge(dest))
    }

    pub fn add_instruction(
        &mut self,
        program_counter: u64,
        instruction: &Instruction,
    ) -> Result<(), Error> {
        let current_block = self
            .blocks
            .get_mut(&self.current_block)
            .ok_or(Error::MissingCurrentBlock)?;

        // Ensure we aren't somehow putting instructions before the start of the block
        if program_counter < current_block.start {
            return Err(Error::InvalidCounter);
        }

        current_block
            .block
            .entry(program_counter)
            .and_modify(|v| *v = instruction.insn.clone())
            .or_insert(instruction.insn.clone());

        let jump_type = &instruction.kind;

        if let Some(jump_type) = jump_type {
            match jump_type {
                JumpKind::Conditional => {
                    todo!()
                }
                JumpKind::Unconditional => {
                    todo!()
                }
                _ => {}
            }
        }

        Ok(())
    }
}
