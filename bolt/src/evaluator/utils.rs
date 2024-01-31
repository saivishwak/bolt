use crate::object::{
    object::{BooleanObj, Interger, Object},
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

pub fn evaluate_binary_expression(
    operator: String,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Result<Box<dyn Object>, ()> {
    let right_value_any = right.as_any();
    let left_value_any = left.as_any();
    if left.get_type() == ObjectType::INTERGER && right.get_type() == ObjectType::INTERGER {
        let right_val: &Interger;
        let left_value: &Interger;
        if let Some(int) = right_value_any.downcast_ref::<Interger>() {
            right_val = int;
        } else {
            return Err(());
        }
        if let Some(int) = left_value_any.downcast_ref::<Interger>() {
            left_value = int;
        } else {
            return Err(());
        }
        match operator.as_str() {
            "+" => {
                let new_value = left_value.v + right_val.v;
                return Ok(Box::new(Interger { v: new_value }));
            }
            "-" => {
                let new_value = left_value.v - right_val.v;
                return Ok(Box::new(Interger { v: new_value }));
            }
            "*" => {
                let new_value = left_value.v * right_val.v;
                return Ok(Box::new(Interger { v: new_value }));
            }
            "/" => {
                let new_value = left_value.v / right_val.v;
                return Ok(Box::new(Interger { v: new_value }));
            }
            "<" => {
                let new_value = left_value.v < right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            ">" => {
                let new_value = left_value.v > right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            "==" => {
                let new_value = left_value.v == right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            "!=" => {
                let new_value = left_value.v != right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            ">=" => {
                let new_value = left_value.v >= right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            "<=" => {
                let new_value = left_value.v <= right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            _ => return Err(()),
        }
    } else if left.get_type() == ObjectType::BOOLEAN && right.get_type() == ObjectType::BOOLEAN {
        let right_val: &BooleanObj;
        let left_value: &BooleanObj;
        if let Some(bool) = right_value_any.downcast_ref::<BooleanObj>() {
            right_val = bool;
        } else {
            return Err(());
        }
        if let Some(bool) = left_value_any.downcast_ref::<BooleanObj>() {
            left_value = bool;
        } else {
            return Err(());
        }
        match operator.as_str() {
            "<" => {
                let new_value = left_value.v < right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            ">" => {
                let new_value = left_value.v > right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            "==" => {
                let new_value = left_value.v == right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            "!=" => {
                let new_value = left_value.v != right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            ">=" => {
                let new_value = left_value.v >= right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            "<=" => {
                let new_value = left_value.v <= right_val.v;
                return Ok(Box::new(BooleanObj { v: new_value }));
            }
            _ => return Err(()),
        }
    } else {
        // Cases like 1 + true or 1 > true , true + 1 true > 1 are errored
        return Err(());
    }
}
