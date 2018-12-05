extern crate md5;

use std::fmt;

use to_bytes::*;
use thashable::*;

#[derive(Debug)]
pub enum Value {
    Int32(u32),
    Bool(bool),
    Str(String),
    Hash([u8; 16]),
    Error,
}

impl Clone for Value {
    fn clone (&self) -> Self {
        match self {
            Value::Int32(x) => Value::Int32(*x),
            Value::Bool(x) => Value::Bool(*x),
            Value::Str(x) => Value::Str(x.to_string()),
            Value::Hash(x) => Value::Hash(*x),
            Value::Error => Value::Error,
        }
    }
}

impl THashable for Value {
    fn calc_hash (&self) -> [u8; 16] {
        match self {
            Value::Bool(x) => md5::compute([*x as u8]),
            Value::Int32(x) => md5::compute(u32_bytes(&x)),
            Value::Str(x) => md5::compute(x.to_string().as_bytes()),
            Value::Hash(x) => md5::compute(x),
            _ => panic!("Cannot hash"),
        }.0
    }
}

#[derive(Debug)]
pub enum Instruction {
    NOP,
    Push(Value),
    Pop,
    Equal,
    GreaterThan,
    GreaterThanEqualTo,
    Not,
    Hash,
    Dup,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl THashable for Instruction {
    fn calc_hash (&self) -> [u8; 16] {
        let mut bytes = vec![];

        bytes.extend(md5::compute(self.to_string().as_bytes()).iter());

        if let Instruction::Push(x) = self {
            bytes.extend(x.calc_hash().iter());
        }

        let mut h = md5::Context::new();
        h.consume(bytes);

        h.compute().0
    }
}

fn to_value (value: Option<Value>) -> Value {
    if let Option::None = value {
        Value::Error
    } else if let Option::Some(Value::Error) = value {
        Value::Error
    } else {
        value.unwrap()
    }
}

impl Instruction {
    pub fn op (&self, stack: &mut Vec<Value>) -> () {
        match &self {
            Instruction::NOP => (),
            Instruction::Push(x) => {
                stack.push(x.clone());
            },
            Instruction::Pop => {
                stack.pop();
            },
            Instruction::Equal => {
                let a = to_value(stack.pop());
                let b = to_value(stack.pop());
                let v = match (a, b) {
                    (Value::Int32(x), Value::Int32(y)) => Value::Bool(x == y),
                    (Value::Bool(x), Value::Bool(y)) => Value::Bool(x == y),
                    (Value::Str(x), Value::Str(y)) => Value::Bool(x == y),
                    (Value::Hash(x), Value::Hash(y)) => Value::Bool(x == y),
                    _ => Value::Error,
                };
                stack.push(v);
            },
            Instruction::GreaterThan => {
                let a = to_value(stack.pop());
                let b = to_value(stack.pop());
                let v = match (a, b) {
                    (Value::Int32(x), Value::Int32(y)) => Value::Bool(x > y),
                    _ => Value::Error,
                };
                stack.push(v);
            },
            Instruction::Or => {
                let a = to_value(stack.pop());
                let b = to_value(stack.pop());
                let v = match (a, b) {
                    (Value::Bool(x), Value::Bool(y)) => Value::Bool(x || y),
                    _ => Value::Error,
                };
                stack.push(v);
            },
            Instruction::And => {
                let a = to_value(stack.pop());
                let b = to_value(stack.pop());
                let v = match (a, b) {
                    (Value::Bool(x), Value::Bool(y)) => Value::Bool(x && y),
                    _ => Value::Error,
                };
                stack.push(v);
            },
            Instruction::GreaterThanEqualTo => {
                let a = to_value(stack.pop());
                let b = to_value(stack.pop());
                let v = match (a, b) {
                    (Value::Int32(x), Value::Int32(y)) => Value::Bool(x >= y),
                    _ => Value::Error,
                };
                stack.push(v);
            },
            Instruction::Not => {
                let a = to_value(stack.pop());
                let v = match a {
                    Value::Bool(x) => Value::Bool(!x),
                    _ => Value::Error,
                };
                stack.push(v);
            },
            Instruction::Dup => {
                let a = to_value(stack.pop());
                stack.push(a.clone());
                stack.push(a);
            },
            Instruction::Hash => {
                let a = to_value(stack.pop());
                if let Value::Error = a {
                    stack.push(Value::Error);
                } else {
                    let v = match a {
                        Value::Bool(x) => md5::compute([x as u8]),
                        Value::Int32(x) => md5::compute(u32_bytes(&x)),
                        Value::Str(x) => md5::compute(x.to_string().as_bytes()),
                        Value::Hash(x) => md5::compute(x),
                        _ => unreachable!(),
                    };
                    stack.push(Value::Hash(v.0));
                }
            },
            Instruction::Add => {
                let a = to_value(stack.pop());
                let b = to_value(stack.pop());
                let v = match (a, b) {
                    (Value::Int32(x), Value::Int32(y)) => Value::Int32(x + y),
                    _ => Value::Error,
                };
                stack.push(v);
            },
            Instruction::Sub => {
                let a = to_value(stack.pop());
                let b = to_value(stack.pop());
                let v = match (a, b) {
                    (Value::Int32(x), Value::Int32(y)) => Value::Int32(x - y),
                    _ => Value::Error,
                };
                stack.push(v);
            },
            Instruction::Mul => {
                let a = to_value(stack.pop());
                let b = to_value(stack.pop());
                let v = match (a, b) {
                    (Value::Int32(x), Value::Int32(y)) => Value::Int32(x * y),
                    _ => Value::Error,
                };
                stack.push(v);
            },
            Instruction::Div => {
                let a = to_value(stack.pop());
                let b = to_value(stack.pop());
                let v = match (a, b) {
                    (Value::Int32(x), Value::Int32(y)) => Value::Int32(x / y),
                    _ => Value::Error,
                };
                stack.push(v);
            },
        }
    }
}
