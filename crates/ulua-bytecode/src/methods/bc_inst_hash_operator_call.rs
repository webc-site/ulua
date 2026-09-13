use core::iter::repeat_n;

use crate::records::{bc_inst::BcInst, bc_inst_hash::BcInstHash, bc_op::BcOp};

/// 参与哈希的 BcOp 操作数上限（与 C++ 展开版一致）。
const HASHED_OPS: usize = 7;

impl BcInstHash {
  pub fn call(&self, key: &BcInst) -> usize {
    // MurmurHash2 unrolled (faithful to BytecodeGraph.h `BcInstHash::operator()`).
    let mut h: u32 = 25;

    h = Self::mix_u32_u32(h, key.op as u32);
    // 不足 HASHED_OPS 个操作数时以默认 BcOp 补齐，保证哈希稳定。
    h = key
      .ops
      .iter()
      .take(HASHED_OPS)
      .fold(h, |h, &op| Self::mix_u32_bc_op(h, op));
    h =
      repeat_n(BcOp::new(), HASHED_OPS.saturating_sub(key.ops.len())).fold(h, Self::mix_u32_bc_op);

    // MurmurHash2 tail
    h ^= h >> 13;
    h = h.wrapping_mul(Self::M);
    h ^= h >> 15;

    h as usize
  }
}
