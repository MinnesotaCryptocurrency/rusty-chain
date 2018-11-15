extern crate rusty_chain;
extern crate time;

use rusty_chain::*;

fn main () {
    let difficulty = 2;

    let addr_alice = "Alice";
    let addr_bob = "Bob";

    let mut b = Blockchain::new();

    let input = TxOutput {
        to_addr: addr_alice.to_string(),
        value: 102,
        transaction_id: [0; 16],
        index: 0,
    };
    let dest_out = TxOutputLoose {
        to_addr: addr_bob.to_string(),
        value: 50,
    };
    let change_out = TxOutputLoose {
        to_addr: addr_alice.to_string(),
        value: 45,
    };
    let tx = Tx::new(vec![input], vec![dest_out, change_out]);
    println!("{:?}", tx.get_fee());

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
