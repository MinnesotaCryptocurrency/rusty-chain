use block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new () -> Self {
        let g = Block::new(0, 349084, [0; 16], String::from("Genesis Block"));
        Blockchain {
            blocks: vec![g],
        }
    }

    pub fn add (&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn verify (&self) -> bool {
        for i in 1..self.blocks.len() {
            let p = &self.blocks[i-1];
            let c = &self.blocks[i];
            if c.index != i as u32 || c.prev_block_hash != p.calc_hash() {
                return false;
            }
        }

        true
    }
}
