extern crate md5;
extern crate hex;

use std::fmt;
use thashable::*;
use to_bytes::*;

type Address    = String;
type TxOutputId = Hash;
type TxId = Hash;

pub struct TxOutputLoose {
    pub to_addr: Address,
    pub value: u64,
}

pub struct TxOutput {
    pub to_addr: Address,
    pub value: u64,
    pub transaction_id: TxId,
    pub index: u8,
}

impl THashable for TxOutput {
    fn calc_hash (&self) -> TxOutputId {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes().iter());
        bytes.extend(u64_bytes(&self.value).iter());
        bytes.extend(self.transaction_id.iter());
        bytes.push(self.index);

        let mut h = md5::Context::new();
        h.consume(bytes);

        h.compute().0
    }
}

pub struct Tx {
    inputs: Vec<TxOutput>,
    outputs: Vec<TxOutput>,
}

impl Tx {
    pub fn new (inputs: Vec<TxOutput>, loose_outputs: Vec<TxOutputLoose>) -> Self {
        let mut tx = Tx {
            inputs,
            outputs: vec![],
        };

        let hash = tx.calc_hash();

        let outputs = loose_outputs.iter().enumerate().map(move |e| {
            let index = e.0 as u8;
            let loose = e.1;
            TxOutput {
                index,
                to_addr: loose.to_addr.to_string(),
                value: loose.value,
                transaction_id: hash,
            }
        }).collect();

        tx.outputs = outputs;

        tx
    }

    pub fn is_cb (&self) -> bool {
        self.inputs.len() == 0
    }

    pub fn get_fee (&self) -> u64 {
        let total_input = self.inputs.iter().fold(0, |a, x| a + x.value);
        let total_output = self.outputs.iter().fold(0, |a, x| a + x.value);

        total_input - total_output
    }

    pub fn gen_out (&self, to_addr: Address, value: u64) -> TxOutput {
        TxOutput {
            to_addr,
            value,
            transaction_id: self.calc_hash(),
            index: self.outputs.len() as u8,
        }
    }
}

impl fmt::Debug for Tx {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hash_str = hex::encode(&self.calc_hash());
        write!(f,
            "Tx[{}]: {} -> {} ({})",
            &hash_str[..6],
            &self.inputs[0].to_addr,
            &self.outputs[0].to_addr,
            &self.outputs[0].value,
        )
    }
}

impl THashable for Tx {
    fn calc_hash (&self) -> TxId {
        let mut bytes = vec![];

        for i in (&self.inputs).iter() {
            bytes.extend((&i.calc_hash()).iter());
        }

        // Not bothering to hash the outputs for convenience
        // (This is not a security recommendation!)

        let mut h = md5::Context::new();
        h.consume(bytes);

        h.compute().0
    }
}
