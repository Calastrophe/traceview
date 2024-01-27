use super::{function::Function, Error, Instruction, JumpKind};
use std::collections::HashMap;

pub struct ControlFlowGraph {
    functions: HashMap<u64, Function>,
    current_function: u64,
    call_stack: Vec<u64>,
}

impl ControlFlowGraph {
    pub fn new(entry_point: u64) -> Self {
        ControlFlowGraph {
            functions: HashMap::new(),
            current_function: entry_point,
            call_stack: Vec::new(),
        }
    }

    pub fn execute(
        &mut self,
        program_counter: u64,
        instruction: &Instruction,
    ) -> Result<(), Error> {
        let current_function = self
            .functions
            .get_mut(&self.current_function)
            .ok_or(Error::MissingCurrentFunction)?;

        let jump_type = &instruction.kind;

        current_function.add_instruction(program_counter, instruction)?;

        if let Some(jump_type) = jump_type {
            match jump_type {
                JumpKind::Call => {
                    todo!()
                }
                JumpKind::Return => {
                    todo!()
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Adds a function with the given start address, name is auto-generated.
    fn add_function(&mut self, starting_address: u64) -> &mut Function {
        self.functions
            .entry(starting_address)
            .or_insert(Function::new(starting_address))
    }
}
