extern crate md5;
extern crate hex;

use std::fmt;
use thashable::*;
use to_bytes::*;
use oxygen::*;

type Address    = String;
type TxOutputId = Hash;
type TxId = Hash;

pub struct TxOutputLoose {
    pub to_addr: Address,
    pub value: u64,
    pub oxygen_pub_key: Vec<Instruction>,
}

pub struct TxOutput {
    pub to_addr: Address,
    pub value: u64,
    pub transaction_id: TxId,
    pub index: u8,
    // Corresponds to Bitcoin's scriptPubKey field
    pub oxygen_pub_key: Vec<Instruction>,
}

pub struct TxInput<'a> {
    pub output: &'a TxOutput,
    // Corresponds to Bitcoin's scriptSig field
    pub oxygen_sig: Vec<Value>,
}

impl<'a> TxInput<'a> {
    pub fn validate (&self) -> bool {
        OxygenScript {
            input: &self.oxygen_sig,
            instructions: &self.output.oxygen_pub_key,
        }.run()
    }
}

impl THashable for TxOutput {
    fn calc_hash (&self) -> TxOutputId {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes().iter());
        bytes.extend(u64_bytes(&self.value).iter());
        bytes.extend(self.transaction_id.iter());
        bytes.push(self.index);

        for i in self.oxygen_pub_key.iter() {
            bytes.extend(i.calc_hash().iter());
        }

        let mut h = md5::Context::new();
        h.consume(bytes);

        h.compute().0
    }
}

pub struct Tx<'a> {
    inputs: Vec<TxInput<'a>>,
    outputs: Vec<TxOutput>,
}

impl<'a> Tx<'a> {
    pub fn new (inputs: Vec<TxInput<'a>>, loose_outputs: Vec<TxOutputLoose>) -> Self {
        let mut tx = Tx {
            inputs,
            outputs: vec![],
        };

        let hash = tx.calc_hash();

        let outputs = loose_outputs.into_iter().enumerate().map(move |e| {
            let index = e.0 as u8;
            let loose = e.1;
            TxOutput {
                index,
                to_addr: loose.to_addr.to_string(),
                value: loose.value,
                transaction_id: hash,
                oxygen_pub_key: loose.oxygen_pub_key,
            }
        }).collect();

        tx.outputs = outputs;

        tx
    }

    pub fn is_cb (&self) -> bool {
        self.inputs.len() == 0
    }

    pub fn get_fee (&self) -> u64 {
        let total_input = self.inputs.iter().fold(0, |a, x| a + x.output.value);
        let total_output = self.outputs.iter().fold(0, |a, x| a + x.value);

        total_input - total_output
    }

    pub fn validate (&self) -> bool {
        self.inputs.iter().all(|i| i.validate())
    }
}

impl<'a> fmt::Debug for Tx<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hash_str = hex::encode(&self.calc_hash());
        write!(f,
            "Tx[{}]: {} -> {} ({})",
            &hash_str[..6],
            &self.inputs[0].output.to_addr,
            &self.outputs[0].to_addr,
            &self.outputs[0].value,
        )
    }
}

impl<'a> THashable for Tx<'a> {
    fn calc_hash (&self) -> TxId {
        let mut bytes = vec![];

        for i in (&self.inputs).iter() {
            bytes.extend((&i.output.calc_hash()).iter());
        }

        // Not bothering to hash the outputs for convenience
        // (This is not a security recommendation!)

        let mut h = md5::Context::new();
        h.consume(bytes);

        h.compute().0
    }
}
