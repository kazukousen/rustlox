use rustlox::{Chunk, Compiler, Scanner, VM};

fn main() {
    let source = "1 + 2";
    let mut compiler = Compiler::new();
    match compiler.compile(source) {
        Some(chunk) => {
            let mut vm = VM::new(&chunk);
            vm.run();
        },
        None => eprintln!("Failed to compile"),
    }
}
