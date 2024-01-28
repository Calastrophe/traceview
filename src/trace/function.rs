use super::{block::BasicBlock, Error, GraphFile, Instruction, JumpKind};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub struct Function {
    /// The name of the function which follows a `sub_[address]` format with a given starting
    /// address.
    ///
    /// The only exception to this rule is the `start` function.
    pub(crate) name: String,
    /// The starting address of the function.
    pub(crate) start: u64,
    /// All of the basic blocks which make up the function.
    pub(crate) blocks: BTreeMap<u64, BasicBlock>,
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
            blocks: BTreeMap::from([(address, BasicBlock::new(address))]),
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

        if !current_block.block.contains_key(&insn.addr) {
            current_block.execute(insn);
        }

        let jump_type = &insn.kind;

        if let Some(jump_type) = jump_type {
            let next_insn = next_insn.ok_or_else(|| Error::MissingInstruction)?;

            match jump_type {
                JumpKind::Conditional | JumpKind::Unconditional => {
                    current_block.add_edge(next_insn.addr, *jump_type);

                    self.blocks
                        .entry(next_insn.addr)
                        .or_insert(BasicBlock::new(next_insn.addr));

                    self.current_block = next_insn.addr;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn dot(&self) -> Result<GraphFile, Error> {
        let filename = format!("/tmp/graph_{0}", self.start);
        let mut fd = File::create(&filename)?;

        fd.write(b"digraph {\n")?;

        for (address, block) in &self.blocks {
            fd.write(
                format!(
                    "\tnode_{0} [shape=box][label=\"{1}\"][color=\"gray0\"][penwidth=2]\n",
                    address,
                    block.to_string()
                )
                .as_bytes(),
            )?;
        }

        for (address, block) in &self.blocks {
            for (kind, edge, count) in block.edges() {
                let color = match kind {
                    JumpKind::Conditional => "red",
                    JumpKind::Unconditional => "blue",
                    _ => unreachable!(),
                };

                fd.write(
                    format!(
                        "\tnode_{0} -> node_{1} [label=\"{2}\"][color=\"{3}\", headport=n, tailport=s]\n",
                        address, edge, count, color
                    )
                    .as_bytes(),
                )?;
            }
        }

        fd.write(b"}")?;

        let out = format!("/tmp/graph_{0}.jpeg", self.start);

        Command::new("dot")
            .arg("-Tjpg")
            .arg("-Gdpi=300")
            .arg(&filename)
            .arg("-o")
            .arg(&out)
            .spawn()
            .expect("failed to create graph");

        Ok(GraphFile {
            address: self.start,
            name: self.name.clone(),
            path: out,
        })
    }
}
