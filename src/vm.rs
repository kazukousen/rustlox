use crate::chunk::{Chunk, OpCode::*, Value};
use crate::debug::disassemble_instruction;

#[derive(Debug, Eq, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM<'a> {
    chunk: &'a Chunk,
    pc: usize,
    pub stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            pc: 0,
            stack: Vec::new(),
        }
    }

    // dispatch instructions.
    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = &self.chunk.instructions[self.pc];
            self.pc = self.pc + 1;

            print!("        ");
            for v in self.stack.iter() {
                print!("[{:04}]", v);
            }
            print!("\n");
            disassemble_instruction(self.chunk, self.pc - 1);

            match instruction {
                Return => return InterpretResult::Ok,
                Constant(index) => {
                    let v = self.chunk.values[*index];
                    self.push(v);
                }
                Negate => {
                    let v = self.pop();
                    self.push(-v);
                }
                Add => {
                    let (b, a) = (self.pop(), self.pop());
                    self.push(a + b);
                }
                Subtract => {
                    let (b, a) = (self.pop(), self.pop());
                    self.push(a - b);
                }
                Multiply => {
                    let (b, a) = (self.pop(), self.pop());
                    self.push(a * b);
                }
                Divide => {
                    let (b, a) = (self.pop(), self.pop());
                    self.push(a / b);
                }
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        match self.stack.pop() {
            Some(v) => return v,
            _ => panic!("VM tried to get value from empty stack"),
        }
    }
}
