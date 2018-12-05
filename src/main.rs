extern crate rusty_chain;

use rusty_chain::*;

fn demo_oxygen_script () {
    let os = OxygenScript {
        input: &vec![
            Value::Str("Attempt".to_string()),
        ],
        instructions: &vec![
            Instruction::Hash,
            Instruction::Push(Value::Hash([94, 182, 187, 21, 117, 40, 179, 101, 248, 76, 39, 187, 71, 132, 3, 27])),
            Instruction::Equal,
        ],
    };

    println!("Result: {}", os.run());
}

fn demo_oxygen_script2 () {
    let os = OxygenScript {
        input: &vec![
            Value::Int32(21),
        ],
        instructions: &vec![
            Instruction::Push(Value::Int32(20)),
            Instruction::Sub,
            Instruction::Dup,
            Instruction::Push(Value::Int32(5)),
            Instruction::GreaterThanEqualTo,
            Instruction::Assert,
            Instruction::Push(Value::Int32(0)),
            Instruction::GreaterThan,
            Instruction::Not,
        ],
    };

    println!("Result: {}", os.run());
}

fn demo_blockchain () {
    let difficulty = 2;

    let addr_alice = "Alice";
    let addr_bob = "Bob";

    let input_output = TxOutput {
        to_addr: addr_alice.to_string(),
        value: 102,
        transaction_id: [0; 16],
        index: 0,
        oxygen_pub_key: vec![
            Instruction::Hash,
            Instruction::Push(Value::Hash([94, 182, 187, 21, 117, 40, 179, 101, 248, 76, 39, 187, 71, 132, 3, 27])),
            Instruction::Equal,
        ],
    };
    let input = TxInput {
        output: &input_output,
        oxygen_sig: vec![
            Value::Str("Attempt".to_string()),
        ],
    };
    println!("{}", input.validate());
    let dest_out = TxOutputLoose {
        to_addr: addr_bob.to_string(),
        value: 50,
        oxygen_pub_key: vec![],
    };
    let change_out = TxOutputLoose {
        to_addr: addr_alice.to_string(),
        value: 45,
        oxygen_pub_key: vec![],
    };
    let tx = Tx::new(vec![input], vec![dest_out, change_out]);
    println!("{:?}", tx.get_fee());

    let mut b = Blockchain::new();

    let p = b.blocks[b.blocks.len() - 1].hash.clone();
    let mut block = Block::new(1, 0, p, vec![tx]);
    block.mine(difficulty);
    b.add(block);

    // let p = b.blocks[b.blocks.len() - 1].hash.clone();
    // let mut s = Block::new(2, 123445, p, Box::new([]));
    // s.mine(difficulty);
    // b.add(s);

    // let p = b.blocks[b.blocks.len() - 1].hash.clone();
    // let mut s = Block::new(3, 123495, p, Box::new([]));
    // s.mine(difficulty);
    // b.add(s);

    for s in &b.blocks {
        println!("{:?}", s);
    }

    println!("{:?}", b.verify());
}

fn main () {
    demo_oxygen_script();
}
