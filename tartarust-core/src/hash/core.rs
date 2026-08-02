use crate::blocks::Block;
use crate::hash::memory_pool::MemoryPool;
use crate::{errors::TartarusError, hash::HmacSha256};
use zeroize::Zeroize;
use hmac::{KeyInit, Mac};

pub fn tartarus(
    data: &[u8],
    salt: &[u8], 
    pepper: &[u8], 
    memory: u32, 
    iterations: u32, 
) -> Result<Vec<u8>, TartarusError> {
    let mut mac = HmacSha256::new_from_slice(pepper)?;
    
    let len_prefix = &prepare_len_prefix(salt.len() as u32, data.len() as u32);

    mac.update(len_prefix);
    mac.update(salt);
    mac.update(data);
    
    let mut digest = mac.finalize().into_bytes();

    let mut state = Block::from_digest(&digest);
    state.mix();

    let mut memory_pool = MemoryPool::new(memory);
    memory_pool.init(&state);

    memory_pool.forward(&mut state, iterations);

    memory_pool.reduct(state);
    state.mix();

    digest.zeroize();

    let bytes: &[u8] = bytemuck::cast_slice(&state);

    Ok(bytes.to_vec())
}


fn prepare_len_prefix(salt_len: u32, data_len: u32) -> [u8; 8] {
    let mut len_prefix = [0u8; 8];

    len_prefix[..4].copy_from_slice(&(salt_len as u32).to_be_bytes());
    len_prefix[4..].copy_from_slice(&(data_len as u32).to_be_bytes());
    
    len_prefix
}