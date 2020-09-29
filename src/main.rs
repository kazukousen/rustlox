use rustlox::{Chunk, Compiler, Scanner};

fn main() {
    let source = "print 1 + 2;";
    let mut chunk = Chunk::new();
    let mut scanner = Scanner::new(source);
    let mut compiler = Compiler::new(&mut chunk, scanner);
}
