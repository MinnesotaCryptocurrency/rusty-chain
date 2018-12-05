mod instruction;

pub use self::instruction::*;

pub struct OxygenScript<'a> {
    pub input: &'a Vec<instruction::Value>,
    pub instructions: &'a Vec<instruction::Instruction>,
}

impl<'a> OxygenScript<'a> {
    pub fn run (&self) -> bool {
        let mut stack = self.input.iter().cloned().collect::<Vec<instruction::Value>>();

        for i in self.instructions.iter() {
            println!("{:?}", &stack);
            i.op(&mut stack);
        }

        println!("{:?}", &stack);

        for v in stack.iter() {
            if let Value::Error = v {
                return false;
            }
        }

        match stack.last() {
            Option::Some(instruction::Value::Bool(true)) => true,
            _ => false,
        }
    }
}
