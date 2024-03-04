use crate::{parser::ExprType, scanner::TokenType, Value};

mod error;
pub use error::*;

pub fn evaluate(expr: ExprType) -> Result<Value> {
    match expr {
        ExprType::Unary(operator, right) => {
            let right_value = evaluate(*right)?;

            match (operator.token_type, right_value) {
                (TokenType::Subtract, Value::Int(value)) => Ok(Value::Int(-value)),
                (TokenType::Subtract, Value::Float(value)) => Ok(Value::Float(-value)),
                (TokenType::LogicalNot, Value::Bool(value)) => Ok(Value::Bool(!value)),
                _ => Err(Error::new(ErrorType::MismatchedType)),
            }
        }
        ExprType::Literal(value) => Ok(value),
        ExprType::Binary(left, operator, right) => {
            let left_value = evaluate(*left)?;
            let right_value = evaluate(*right)?;

            include!(concat!(env!("OUT_DIR"), "/binary.rs"))
        }
    }
}
