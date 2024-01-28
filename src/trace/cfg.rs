use super::{function::Function, Error, GraphFile, Instruction, JumpKind};
use std::collections::HashMap;

pub struct ControlFlowGraph {
    functions: HashMap<u64, Function>,
    current_function: u64,
    call_stack: Vec<u64>,
}

impl ControlFlowGraph {
    pub fn new(entry_point: u64) -> Self {
        ControlFlowGraph {
            functions: HashMap::from([(entry_point, Function::new(entry_point))]),
            current_function: entry_point,
            call_stack: Vec::new(),
        }
    }

    pub fn construct(&mut self, insns: &[Instruction]) -> Result<(), Error> {
        let mut iter = insns.iter().peekable();

        while let Some(insn) = iter.next() {
            let current_function = self
                .functions
                .get_mut(&self.current_function)
                .ok_or(Error::MissingCurrentFunction)?;

            let jump_type = &insn.kind;

            let next_insn = iter.peek();

            current_function.execute(insn, next_insn)?;

            if let Some(jump_type) = jump_type {
                // HANDLE ERROR
                let next_insn = next_insn.unwrap();

                match jump_type {
                    JumpKind::Call => {
                        self.call_stack.push(current_function.start);

                        self.add_function(next_insn.addr);

                        self.current_function = next_insn.addr;
                    }
                    JumpKind::Return => {
                        self.current_function = self.call_stack.pop().unwrap();
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Generates the graphs of the control flow graph.
    pub fn gen_graphs(&self) -> Result<Vec<GraphFile>, Error> {
        self.functions.iter().map(|(_, func)| func.dot()).collect()
    }

    /// Adds a function with the given start address if needed, name is auto-generated.
    fn add_function(&mut self, starting_address: u64) {
        self.functions
            .entry(starting_address)
            .or_insert(Function::new(starting_address));
    }
}
