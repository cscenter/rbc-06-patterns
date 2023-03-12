use crate::statement::Statement;
use crate::visitors::Visitor;

pub struct Push {
    pub value: i32,
}

// todo: add Impl and Impl Statement