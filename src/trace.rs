use serde::Deserialize;

#[derive(Deserialize)]
struct Trace {
    info: ArchInfo,
    instructions: Vec<Instruction>,
}

#[derive(Deserialize)]
struct ArchInfo {
    mode: u8,
    registers: Vec<RegisterInfo>,
    memory: Vec<(u64, u64)>,
}

#[derive(Deserialize)]
struct RegisterInfo {
    name: String,
    register: u32,
    base: u32,
    full_register: Option<u32>,
    size: u16,
}

#[derive(Deserialize)]
struct Instruction {
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
