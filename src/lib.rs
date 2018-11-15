extern crate md5;
extern crate hex;

// Utils
mod to_bytes;

// Traits
mod thashable;

// Main
mod block;
pub use block::*;

mod blockchain;
pub use blockchain::*;

mod transaction;
pub use transaction::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
