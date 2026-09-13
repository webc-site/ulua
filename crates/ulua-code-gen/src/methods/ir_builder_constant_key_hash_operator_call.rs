use crate::records::constant_key::ConstantKey;
use crate::records::constant_key_hash::ConstantKeyHash;

impl ConstantKeyHash {
    pub fn call(&self, key: &ConstantKey) -> usize {
        // finalizer from MurmurHash64B
        const M: u32 = 0x5bd1e995;

        let mut h1 = key.value as u32;
        let mut h2 = (key.value >> 32) as u32 ^ ((key.kind as i32 as u32).wrapping_mul(M));

        h1 ^= h2 >> 18;
        h1 = h1.wrapping_mul(M);
        h2 ^= h1 >> 22;
        h2 = h2.wrapping_mul(M);
        h1 ^= h2 >> 17;
        h1 = h1.wrapping_mul(M);
        h2 ^= h1 >> 19;
        h2 = h2.wrapping_mul(M);

        // ... truncated to 32-bit output (normally hash is equal to (uint64_t(h1) << 32) | h2, but we only really need the lower 32-bit half)
        h2 as usize
    }
}

pub fn ir_builder_constant_key_hash_operator_call(
    this: &ConstantKeyHash,
    key: &ConstantKey,
) -> usize {
    this.call(key)
}
