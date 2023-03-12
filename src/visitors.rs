use crate::tokens::exec::Exec;
use crate::tokens::push::Push;

trait Visitor {
    fn visit_push(&mut self, push: &Push);
    fn visit_exec(&mut self, exec: &Exec);
}