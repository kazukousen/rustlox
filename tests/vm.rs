use rustlox::*;

#[test]
fn run_arithmetic() {
    let mut chunk = Chunk::new();
    let i = chunk.add_constant(1.2);
    chunk.add_instruction(OpCode::Constant(i), 1);
    let i = chunk.add_constant(3.4);
    chunk.add_instruction(OpCode::Constant(i), 1);
    chunk.add_instruction(OpCode::Add, 1);
    let i = chunk.add_constant(4.6);
    chunk.add_instruction(OpCode::Constant(i), 1);
    chunk.add_instruction(OpCode::Divide, 1);
    chunk.add_instruction(OpCode::Negate, 1);

    chunk.add_instruction(OpCode::Return, 2);

    let mut vm = VM::new(&chunk);
    assert_eq!(vm.run(), InterpretResult::Ok);
    assert_eq!(vm.stack[0], -1_f64); // - (1.2 + 3.4) / 4.6
}
