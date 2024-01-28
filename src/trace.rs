use self::{cfg::ControlFlowGraph, register::Registers};
use serde::Deserialize;
use std::collections::BTreeMap;
use thiserror::Error;

mod block;
mod cfg;
mod function;
mod register;

pub struct GraphFile {
    pub address: u64,
    pub name: String,
    pub path: String,
}

pub struct Tracer {
    pub step: usize,
    pub registers: Registers,
    pub memory: BTreeMap<u64, u64>,
    pub graphs: Vec<GraphFile>,
    pub instructions: Vec<Instruction>,
}

impl Tracer {
    pub fn new(trace: TraceFile) -> Result<Tracer, Error> {
        let registers = Registers::new(&trace.info.registers);
        let memory: BTreeMap<u64, u64> = BTreeMap::from_iter(trace.info.memory.into_iter());

        let first = trace.instructions.first().unwrap();
        let mut graph = ControlFlowGraph::new(first.addr);
        graph.construct(&trace.instructions)?;
        let graphs = graph.gen_graphs()?;

        Ok(Self {
            step: 0,
            registers,
            memory,
            graphs,
            instructions: trace.instructions,
        })
    }

    pub fn step_forward(&mut self) {
        let events = &self.instructions[self.step].events;

        for event in events {
            match event {
                Event::RegWrite(reg, val) => {
                    self.registers.write(*reg as usize, val.clone());
                }
                Event::MemWrite(addr, val) => {
                    self.memory
                        .entry(*addr)
                        .and_modify(|v| *v = *val)
                        .or_insert(*val);
                }
                _ => {}
            }
        }

        if self.step < self.instructions.len() {
            self.step += 1;
        }
    }

    pub fn step_backward(&mut self) {
        if self.step < 1 {
            return;
        }

        let events = &self.instructions[self.step].events;

        for event in events.iter().rev() {
            match event {
                Event::RegWrite(reg, val) => {
                    self.registers.write(*reg as usize, val.clone());
                }
                Event::MemWrite(addr, val) => {
                    self.memory
                        .entry(*addr)
                        .and_modify(|v| *v = *val)
                        .or_insert(*val);
                }
                _ => {}
            }
        }

        self.step -= 1;
    }
}

#[derive(Deserialize)]
pub struct TraceFile {
    info: ArchInfo,
    instructions: Vec<Instruction>,
}

#[derive(Deserialize)]
pub struct ArchInfo {
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

#[derive(Deserialize, Debug)]
pub struct Instruction {
    addr: u64,
    pub insn: String,
    size: Option<u16>,
    kind: Option<JumpKind>,
    events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
pub enum Event {
    RegRead(u32),
    RegWrite(u32, Box<[u8]>),
    MemRead(u64),
    MemWrite(u64, u64),
}

#[derive(Deserialize, Debug, Clone, Copy)]
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
    #[error("There was a failure when writing to a file.")]
    IO(#[from] std::io::Error),
    #[error("Failed to grab another instruction when one was expected.")]
    MissingInstruction,
    #[error("There was an error loading an image.")]
    Image(#[from] image::error::ImageError),
}
