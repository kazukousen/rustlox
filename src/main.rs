use rustlox::{Compiler, VM};

fn main() {
    let source = "1 + (-2 + 5)";
    let mut compiler = Compiler::new();
    match compiler.compile(source) {
        Some(chunk) => {
            let mut vm = VM::new(&chunk);
            vm.run();
        },
        None => eprintln!("Failed to compile"),
    }
}
