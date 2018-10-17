extern crate md5;
extern crate hex;

mod block;
pub use block::Block;

mod blockchain;
pub use blockchain::Blockchain;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
