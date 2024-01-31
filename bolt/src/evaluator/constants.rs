use crate::object::object::{BooleanObj, Null};

pub const TRUE: BooleanObj = BooleanObj { v: true };
pub const FALSE: BooleanObj = BooleanObj { v: false };
pub const NULL: Null = Null {};
