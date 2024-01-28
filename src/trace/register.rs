use std::collections::BTreeMap;

use super::RegisterInfo;
use byteorder::{ByteOrder, LittleEndian};

pub struct Registers {
    registers: Vec<Register>,
    lookup: BTreeMap<usize, usize>,
}

impl Registers {
    pub fn new(info: &[RegisterInfo]) -> Self {
        let mut registers = Vec::new();
        let mut lookup = BTreeMap::new();

        // Create the register array and construct the lookup table.
        for register in info {
            if let Some(reg) = register.full_register {
                lookup.insert(register.register as usize, reg as usize);
            } else {
                registers.push(Register::new(register.name.clone(), register.size as usize))
            }
        }

        Registers { registers, lookup }
    }

    pub fn write(&mut self, idx: usize, value: Box<[u8]>) {
        // Use the lookup to see if this is an aliased register.
        let idx = self.lookup.entry(idx).or_insert(idx);

        self.registers[*idx].write(value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Register> {
        self.registers.iter()
    }
}

pub struct Register {
    name: String,
    value: Box<[u8]>,
}

impl Register {
    pub fn new(name: String, size: usize) -> Self {
        Self {
            name,
            value: vec![0u8; size].into_boxed_slice(),
        }
    }

    pub fn write(&mut self, value: Box<[u8]>) {
        self.value
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v &= value[i])
    }

    pub fn to_string(&self) -> String {
        let value = match self.value.len() {
            1 => self.value[0].to_string(),
            2 => LittleEndian::read_u16(&self.value).to_string(),
            3 => LittleEndian::read_u24(&self.value).to_string(),
            4 => LittleEndian::read_u32(&self.value).to_string(),
            6 => LittleEndian::read_u48(&self.value).to_string(),
            8 => LittleEndian::read_u64(&self.value).to_string(),
            16 => LittleEndian::read_u128(&self.value).to_string(),
            _ => panic!("Unsupported register size, unimplemented!"),
        };

        format!("{0} : {value}", self.name)
    }
}
