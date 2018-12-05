extern crate md5;
extern crate hex;

use to_bytes::*;
use thashable::*;
use transaction::Tx;
use std::fmt;

type BlockHash = [u8; 16];

pub struct Block<'a> {
    pub index: u32,
    pub timestamp: u64,
    pub prev_block_hash: BlockHash,
    pub hash: BlockHash,
    pub nonce: u64,

    pub payload: Vec<Tx<'a>>,
}

impl<'a> fmt::Debug for Block<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h_str = hex::encode(&self.hash);
        write!(f, "Block[{}]:\t{} nonce: {}\tat {}\t\tpayload: {:?}", &self.index, h_str, &self.nonce, &self.timestamp, &self.payload)
    }
}

impl<'a> Block<'a> {
    pub fn new (index: u32, timestamp: u64, prev_block_hash: BlockHash, payload: Vec<Tx<'a>>) -> Self {
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

        for tx in &self.payload {
            bytes.extend(tx.calc_hash().iter());
        }

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
