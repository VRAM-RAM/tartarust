use std::{ops::{Index, IndexMut, Deref, DerefMut}};

pub mod block_operations;

const BLOCK_WORDS: usize = 256; 

#[derive(Clone, Copy)]
pub struct Block([u32; BLOCK_WORDS]);

impl Index<usize> for Block {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Deref for Block {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Block {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IndexMut<usize> for Block {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Block {
    pub fn new(array: [u32; BLOCK_WORDS]) -> Self {
        Self(array)
    }

    pub fn empty() -> Self {
        Self([0u32; BLOCK_WORDS])
    }

    pub fn from_state_with_index(state: &Block, index: u32) -> Self {
        let mut block = state.clone();

        block[0] ^= index;
        block[1] ^= !index;
        block[2] ^= index.rotate_left(16);
        block[3] ^= index.wrapping_mul(0x9E3779B9);

        block.mix();

        block
    }

    pub fn words_mut(&mut self) -> &mut [u32; BLOCK_WORDS] {
        &mut self.0
    }

    pub fn words(&self) -> &[u32; BLOCK_WORDS] {
        &self.0
    }

    pub fn from_digest(digest: &[u8]) -> Self {
        let mut block = Self([0u32; 256]);

        for (word, bytes) in block.0[..16]
            .iter_mut()
            .zip(digest.chunks_exact(4))
        {
            *word = u32::from_le_bytes(bytes.try_into().unwrap());
        }

        block
    }

    fn chunks_16(&mut self) -> impl Iterator<Item=&mut [u32]> {
        self.chunks_mut(16)
    }

    pub fn copy_column(&self, index: usize) -> [u32; 16] {
        assert!(index < 16);
        let mut column = [0u32; 16];
        for row in 0..16 {
            column[row] = self[index + (row * 16)];
        }
        column
    }

    pub fn write_column(&mut self, index: usize, column: [u32; 16]) {
        assert!(index < 16);
        for row in 0..16 {
            self[index + (row * 16)] = column[row];
        }
    }
}