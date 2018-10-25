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
    pub fn new (index: u32, timestamp: u64, prev_block_hash: BlockHash, payload: String) -> Self {
        let mut b = Block {
            index,
            timestamp,
            prev_block_hash,
            nonce: 0,
            hash: [0; 16],
            payload,
        };

        b.hash();

        b
    }

    pub fn calc_hash (&self) -> BlockHash {
        let mut bytes = vec![];
        bytes.extend(u32_bytes(&self.index).iter());
        bytes.extend(u64_bytes(&self.timestamp).iter());
        bytes.extend((&self.prev_block_hash).iter().cloned());
        bytes.extend(u64_bytes(&self.nonce).iter());
        bytes.extend((&self.payload).as_bytes().iter().cloned());

        let mut h = md5::Context::new();
        h.consume(bytes);

        h.compute().0
    }

    pub fn hash (&mut self) {
        self.hash = self.calc_hash();
    }

    pub fn calc_diff (&self) -> u8 {
        let mut count = 0;

        for b in &self.hash {
            if *b != 0 {
                break;
            }

            count += 1;
        }

        count
    }

    pub fn mine (&mut self, difficulty: u8) {
        self.nonce = 0;
        self.hash();
        while self.calc_diff() < difficulty {
            self.nonce += 1;
            self.hash();
        }
    }
}
