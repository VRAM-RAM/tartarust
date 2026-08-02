use std::{ops::Rem, process::id};
use zeroize::Zeroize;
use crate::blocks::Block;

pub struct MemoryPool {
    blocks: Vec<Block>,
    count: usize,
}

impl MemoryPool {
    pub fn new(memory: u32) -> Self {
        let block_count = (memory * 1024 * 1024) / size_of::<Block>() as u32;

        let block_count = block_count.max(4);

        Self { blocks: vec![Block::empty(); block_count as usize], count: block_count as usize }
    }

    pub fn init(&mut self, state: &Block) {
        self.blocks[0] = Block::from_state_with_index(&state, 1);
        self.blocks[1] = Block::from_state_with_index(&state, 2);

        let mut index = 2;

        loop {
            self.generate_block(index, state);
            index += 1;
            if index >= self.count { break; }
        }
    } 

    pub fn forward(&mut self, state: &mut Block, iterations: u32) {
        let mut accumulator = self.blocks[self.count - 1][0];

        for iter in 0..iterations as usize {
            state[iter % 16] ^= accumulator;
            state.mix();

            for index in 0..self.count {
                let reference_index = self.infer_index(state, index, iter, accumulator);

                let mut block = self.blocks[index] ^ self.blocks[reference_index];
                block.mix();

                self.blocks[index] = self.blocks[index] ^ block;

                accumulator ^= self.blocks[index][0];     
            }
        }
    }

    pub fn reduct(&mut self, state: &mut Block) {
        for index in 0..self.count {
            state[index % 16] ^= index as u32;
            *state ^= self.blocks[index];

            if index % 256 == 255 { state.mix(); }
        }
    }

    fn infer_index(&self, state: &Block, index: usize, iter: usize, accumulator: u32) -> usize {
        let current_index = (((self.blocks[index][0] as u64 ^ accumulator as u64) * self.count as u64) >> 32) as u32;
        let mut previous_index = 0;

        if iter > 0 {
            let pr2 = self.blocks[index][1] ^ state[(index + 1) % 16];
            previous_index = ((pr2 as u64 * self.count as u64) >> 32) as u32;
        }

        let use_prev = if iter % 2 == 0 {
            index < self.count / 2
        } else {
            index >= self.count / 2
        };

        if iter > 0 && use_prev {
            previous_index as usize
        } else {
            current_index as usize
        }
    }

    fn generate_block(&mut self, index: usize, state: &Block) {
        let pr_a = self.blocks[index - 1][0] ^ state[index % 16];
        let pr_b = self.blocks[index -1][1] ^ state[(index + 1) % 16];

        let pseudo_rand = pr_a ^ (pr_b.rotate_left(16));

        let ref_index = select_reference_index(pseudo_rand, index);

        self.blocks[index] = self.blocks[index - 1] ^ self.blocks[ref_index];
        self.blocks[index].mix();
    }
}

fn select_reference_index(pseudo_rand: u32, index: usize) -> usize {
    ((pseudo_rand as u64 * index as u64) >> 32) as usize
}