use crate::object::{
    object::{Interger, Object},
    types::ObjectType,
};

use super::constants::{FALSE, TRUE};

pub fn evaluate_prefix_expression(
    operator: String,
    right: Box<dyn Object>,
) -> Result<Box<dyn Object>, ()> {
    match operator.as_str() {
        "!" => {
            if right.get_type() == ObjectType::BOOLEAN {
                if right.inspect().as_str() == "true" {
                    return Ok(Box::new(FALSE));
                } else {
                    return Ok(Box::new(TRUE));
                }
            } else if right.get_type() == ObjectType::NULL {
                return Ok(Box::new(TRUE));
            } else {
                return Err(());
            }
        }
        "-" => {
            let value_any = right.as_any();
            if let Some(int) = value_any.downcast_ref::<Interger>() {
                let new_float = -1.0 * int.v;
                return Ok(Box::new(Interger { v: new_float }));
            }
            return Err(());
        }
        _ => {
            return Err(());
        }
    }
}
