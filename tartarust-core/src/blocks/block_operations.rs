use std::ops::{BitXor, BitXorAssign};

use crate::{blocks::{Block}, rounds::double_round::State};

/// Block mixing. Here's how it works :
/// 
/// First, we apply the `round function` over the lines of the `block`. 
/// To do that, we get each `line` of the block, using `chunk_16()` method, that gives an iterator of chunks of size 16.
/// Since our matrice is 16 * 16, the `.chunk_16()` method returns exactly an iterator of lines.
/// 
/// After that, we convert our chunk, which is an `&[u32]` into a `State`, and apply, `10` times, the `round()` method over the state.
/// Since it was borrowed as a mutable, the `block` is directly affected.
/// 
/// 
/// Then, we apply the `round function` over the columns of the `block`.
/// To do that this time, we don't borrow it as a mutable. We *copy* the column of that begins at the index we want 
/// (for example, if the index is `0`, we copy the values at indexes `[0, 16, 32, 48, 64, 80, ..., 240]`).
/// 
/// After copying, we convert our `column`, which is an `[u32; 16]` into a `State`, and apply, also `10` times, the `round()` method over the state.
/// 
/// Finally, we need to rewrite the `column` in our `block`. To do that, we use the `write_column` method.
impl Block {
    pub fn mix(&mut self) {
        for chunk in self.chunks_16() {
            let mut state: State = chunk.try_into().unwrap();

            for _ in 0..10 {
                state.round();
            }
        }

        for index in 0..16 {
            let mut column = self.copy_column(index);

            let mut state: State = (&mut column[..])
                .try_into()
                .unwrap();

            for _ in 0..10 {
                state.round();
            }

            self.write_column(index, column);
        }
    }
}


impl BitXor for Block {
    type Output = Block;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut index = 0;
        let mut output_block = Block::empty();
        loop {
            output_block[index] = self[index] ^ rhs[index];
            index += 1;
            if index > 256 { break; }
        }
        output_block
    }
}

impl BitXorAssign for Block {
    fn bitxor_assign(&mut self, rhs: Self) {
        let mut index = 0;
        loop {
            self[index] ^= rhs[index];
            index += 1;
            if index > 256 { break; }
        }
    }
}
