extern crate md5;
extern crate hex;

use std::time::SystemTime;
use std::fmt;

type BlockHash = [u8; 16];

fn u32_bytes (u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0) as u8,
        (u >> 8 * 1) as u8,
        (u >> 8 * 2) as u8,
        (u >> 8 * 3) as u8,
    ]
}

fn u64_bytes (u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0) as u8,
        (u >> 8 * 1) as u8,
        (u >> 8 * 2) as u8,
        (u >> 8 * 3) as u8,

        (u >> 8 * 4) as u8,
        (u >> 8 * 5) as u8,
        (u >> 8 * 6) as u8,
        (u >> 8 * 7) as u8,
    ]
}

fn now () -> u64 {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    time.as_secs() * 1000 + time.subsec_millis() as u64
}

pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub prev_block_hash: BlockHash,
    pub hash: BlockHash,
    pub nonce: u64,

    pub payload: String,
}

impl fmt::Debug for Block {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h_str = hex::encode(&self.hash);
        write!(f, "Block[{}]:\t{} nonce: {}\tat {}\t\tpayload: {:?}", &self.index, h_str, &self.nonce, &self.timestamp, &self.payload)
    }
}

impl Block {
}
