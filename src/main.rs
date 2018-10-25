extern crate rusty_chain;
extern crate time;

use rusty_chain::*;

fn main () {
    let difficulty = 2;

    let mut b = Blockchain::new();

    let p = b.blocks[b.blocks.len() - 1].hash.clone();
    let mut s = Block::new(1, 123445, p, String::from("Block 1"));
    s.mine(difficulty);
    b.add(s);

    let p = b.blocks[b.blocks.len() - 1].hash.clone();
    let mut s = Block::new(2, 123445, p, String::from("Block 2"));
    s.mine(difficulty);
    b.add(s);

    let p = b.blocks[b.blocks.len() - 1].hash.clone();
    let mut s = Block::new(3, 123445, p, String::from("Block 3"));
    s.mine(difficulty);
    b.add(s);

    for s in &b.blocks {
        println!("{:?}", s);
    }

    println!("{:?}", b.verify());
}
