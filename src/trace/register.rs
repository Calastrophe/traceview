use super::RegisterInfo;
use byteorder::{ByteOrder, LittleEndian};

pub struct Registers(Vec<Register>);

impl Registers {
    pub fn new(info: &[RegisterInfo]) -> Self {
        let mut registers = Vec::new();

        for register in info {
            registers.push(Register::new(register.name.clone(), register.size as usize))
        }

        Registers(registers)
    }

    pub fn write(&mut self, idx: usize, value: Box<[u8]>) {
        self.0[idx].write(value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Register> {
        self.0.iter()
    }
}

pub struct Register {
    name: String,
    value: Box<[u8]>,
}

impl Register {
    pub fn new(name: String, size: usize) -> Self {
        let mut value: Vec<u8> = Vec::new();
        (0..size).into_iter().for_each(|_| value.push(0u8));

        Self {
            name,
            value: value.into_boxed_slice(),
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
