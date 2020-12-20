use crate::chunk::{Chunk, OpCode::*, Value};
use crate::debug::disassemble_instruction;
use crate::value::{Value, print_value};

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

macro_rules! binary_op {
    ( $vm:ident, $constructor:expr, $op:tt ) => {
        {
            let b = $vm.pop().as_number();
            let a = $vm.pop().as_number();
            $vm.stack.push($constructor(a $op b));
        }
    };
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
                    let v = self.chunk.values[*index].clone();
                    self.push(v);
                }
                Negate => {
                    let v = self.pop();
                    self.push(-v);
                }
                Add => binary_op!(self, Value::new_number, +),
                Subtract => binary_op!(self, Value::new_number, -),
                Multiply => binary_op!(self, Value::new_number, *),
                Divide => binary_op!(self, Value::new_number, /),
                Nil => {

                }
                True => {

                }
                False => {

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
