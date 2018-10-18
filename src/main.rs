extern crate rusty_chain;
extern crate time;

use rusty_chain::*;

fn main () {
    let b = Block::new(0, 13452, [0; 16], String::from("Block"));

    let h = b.calc_hash();

    for b in &h {
        print!("{} ", b);
    }

    println!("");

    println!("{:?}", b);
}
