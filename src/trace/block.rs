use std::collections::HashMap;

pub struct BasicBlock {
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) block: HashMap<u64, String>,
    edges: Vec<u64>,
}

impl BasicBlock {
    /// Generates a new BasicBlock with a given start address
    pub fn new(start: u64) -> Self {
        BasicBlock {
            start,
            end: start,
            block: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Appends an instruction to the basic block and sets the end address to the one given.
    pub fn append(&mut self, address: u64, instruction: String) {
        let _ = self.block.insert(address, instruction);

        // TODO: An elegant solution to actually determining the correct end.
        // Right now, this is incorrect.
        self.end = address;
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
    pub fn edges(&self) -> impl Iterator<Item = &u64> {
        self.edges.iter()
    }

    /// Identifies if a given edge already exists for the block.
    pub fn contains_edge(&self, edge: u64) -> bool {
        self.edges.contains(&edge)
    }

    /// Adds a new edge if it cannot find it, otherwise increments the edge counter depending on if it was traversed or not.
    pub fn add_edge(&mut self, edge: u64) {
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }
}
