use crate::visitors::Visitor;

trait Statement {
    fn accept(&self, visitor: &mut dyn Visitor);
}