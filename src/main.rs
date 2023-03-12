#![deny(warnings)]
use crate::statement::Statement;
use crate::tokens::exec::Exec;
use crate::tokens::push::Push;
use crate::visitors::deserialization::DeserializationVisitor;
use crate::visitors::interpretation::InterpretationVisitor;

fn main() {
    // you can't layout Push and Exec in the same Vec because of the different size...
    // You should keep only references to heap in vec
    let tokens /* Specify the type of polymorphic tokens */ = vec![
        Push::new(20),
        Push::new(22),
        Exec::new(String::from("Summ")),
        Exec::new(String::from("Print")),
    ];

    let mut deserialization_visitor = DeserializationVisitor::new();
    for token in &tokens {
        // token is Statement
        token.accept(&mut deserialization_visitor)
    }
    let source_code: String = deserialization_visitor.extract_source_code();
    assert_eq!(source_code, String::from("push 20\npush 22\nexec Summ\nexec Print\n"));

    let mut interpretation_visitor = InterpretationVisitor::new();
    for token in &tokens {
        // token is Statement
        token.accept(&mut interpretation_visitor)
    }
    let std_out: String = interpretation_visitor.extract_stdout();
    assert_eq!(std_out, String::from("42"));
}
