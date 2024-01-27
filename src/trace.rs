use serde::Deserialize;
use thiserror::Error;

mod block;
mod cfg;
mod function;

impl Tracer {
    fn new(trace: TraceFile) -> Tracer {
        let mut registers = Vec::new();

        for register in trace.info.registers {
            registers.push(Register::new(register.name, register.size as usize))
        }

        Self {
            step: 0,
            registers,
            instructions: trace.instructions,
        }
    }
}

pub struct Register {
    name: String,
    value: Box<[u8]>,
}

impl Register {
    pub fn new(name: String, value: usize) -> Register {
        Self {
            name,
            value: Vec::with_capacity(value).into_boxed_slice(),
        }
    }
}

pub struct Tracer {
    step: usize,
    registers: Vec<Register>,
    instructions: Vec<Instruction>,
}

#[derive(Deserialize)]
pub struct TraceFile {
    info: ArchInfo,
    instructions: Vec<Instruction>,
}

#[derive(Deserialize)]
pub struct ArchInfo {
    mode: u8,
    registers: Vec<RegisterInfo>,
    memory: Vec<(u64, u64)>,
}

#[derive(Deserialize)]
pub struct RegisterInfo {
    name: String,
    register: u32,
    full_register: Option<u32>,
    size: u16,
}

#[derive(Deserialize)]
pub struct Instruction {
    addr: u64,
    insn: String,
    size: Option<u16>,
    kind: Option<JumpKind>,
    events: Vec<Event>,
}

#[derive(Deserialize)]
pub enum Event {
    RegRead(u32),
    RegWrite(u32, Box<[u8]>),
    MemRead(u64),
    MemWrite(u64, u64),
}

#[derive(Deserialize)]
pub enum JumpKind {
    Call,
    Return,
    Unconditional,
    Conditional,
}

/// All of the error types associated with the library.
#[derive(Error, Debug)]
pub enum Error {
    /// The block that was queried for in the function does not exist.
    #[error("The block that was attempted to be found does not exist.")]
    MissingBlock,
    /// The current block is somehow missing from the underlying map.
    #[error("The current basic block does not exist.")]
    MissingCurrentBlock,
    /// The current function is somehow missing from the underlying map.
    #[error("The current function does not exist inside the control flow graph.")]
    MissingCurrentFunction,
    /// The program counter provided is inaccurate as it is executing backwards or behind where it
    /// is expected.
    #[error("There was an attempt to add an instruction behind the start of a basic block.")]
    InvalidCounter,
}
