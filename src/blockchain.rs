use block::Block;

pub struct Blockchain<'a> {
    pub blocks: Vec<Block<'a>>,
}

impl<'a> Blockchain<'a> {
    pub fn new () -> Self {
        let g = Block::new(0, 0, [0; 16], vec![]);
        Blockchain {
            blocks: vec![g],
        }
    }

    pub fn add (&mut self, block: Block<'a>) {
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
