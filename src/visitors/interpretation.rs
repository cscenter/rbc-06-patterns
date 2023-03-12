use crate::tokens::exec::Exec;
use crate::tokens::push::Push;
use crate::visitors::Visitor;

pub struct InterpretationVisitor{
    stack: Vec<i32>, // stack that uses for Push and Exec operations
    stdout: String // your program output
}

// todo: add Impl and Impl Visitor
// visit methods executes the code and print to stdout if required, e.g.
// push 123 - push 123 to stack
// exec Summ - pop 2 top items from stack and push the summ back
// exec Print - pop 1 item from stack and print it to stdout

