use crate::chunk::{Chunk, OpCode::*};

pub trait Debug {
    fn disassemble(&self, name: &str);
}

impl Debug for Chunk {
    fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        for i in 0..self.instructions.len() {
            disassemble_instruction(self, i)
        }
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:04} ", chunk.lines[offset]);
    }

    if let Some(op) = chunk.instructions.get(offset) {
        match op {
            Return => simple_instruction("OP_RETURN"),
            Constant(index) => constant_instruction(chunk, *index),
            Negate => simple_instruction("OP_NEGATE"),
            Add => simple_instruction("OP_ADD"),
            Subtract => simple_instruction("OP_SUBSTRACT"),
            Multiply => simple_instruction("OP_MULTIPLY"),
            Divide => simple_instruction("OP_DIVIDE"),
        }
    }
}

fn simple_instruction(name: &str) {
    println!("{}", name);
}

fn constant_instruction(chunk: &Chunk, index: usize) {
    let value = chunk.values[index];
    println!("CONSTANT {:04} {:.2}", index, value);
}
