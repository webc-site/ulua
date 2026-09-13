use crate::records::ir_inst_hash::IrInstHash;

impl IrInstHash {
    pub fn mix_u32_u32(h: u32, k: u32) -> u32 {
        let m: u32 = 0x5bd1e995;
        let r: u32 = 24;

        let mut k = k;
        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);

        let mut h = h;
        h = h.wrapping_mul(m);
        h ^= k;

        h
    }
}
