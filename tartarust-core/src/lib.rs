
pub mod hash;
pub mod rounds;
pub mod blocks;
pub mod errors;

#[test]
fn test_block_copying() {
    use crate::blocks::Block;

    let mut block = Block::new([0u32; 256]);
    let column = block.copy_column(0);
    let _ = block.write_column(0, column);
}