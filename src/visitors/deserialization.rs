use crate::tokens::exec::Exec;
use crate::tokens::push::Push;
use crate::visitors::Visitor;

pub struct DeserializationVisitor{
    output: String
}

// todo: add Impl and Impl Visitor
// visit methods adds lines to output, e.g.
// push 123
// exec Print
// don't forget to add 'new line' to the end of each line
