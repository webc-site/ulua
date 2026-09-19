use ulua_common::records::dense_hash_table::DenseHasher;

use crate::records::{ir_inst::IrInst, ir_op::IrOp};

/// cpp IrData.h `IrInstHash`：MurmurHash2 参数与 unrolled 轮数
const MURMUR_M: u32 = 0x5bd1e995;
const MURMUR_R: u32 = 24;
const MURMUR_SEED: u32 = 25;
const MURMUR_TAIL_SHIFT_1: u32 = 13;
const MURMUR_TAIL_SHIFT_2: u32 = 15;
/// cpp `for (size_t i = 0; i < 7; i++)`：固定混合 7 个操作数槽（OP_A..OP_G）
const HASH_OP_SLOTS: usize = 7;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IrInstHash;

impl IrInstHash {
  #[inline]
  pub fn mix_u32_u32(h: u32, k: u32) -> u32 {
    // MurmurHash2 step
    let mut k = k;
    k = k.wrapping_mul(MURMUR_M);
    k ^= k >> MURMUR_R;
    k = k.wrapping_mul(MURMUR_M);

    let mut h = h;
    h = h.wrapping_mul(MURMUR_M);
    h ^= k;

    h
  }

  #[inline]
  pub fn mix_u32_ir_op(h: u32, op: IrOp) -> u32 {
    let k: u32 = op.kind_and_index;
    Self::mix_u32_u32(h, k)
  }

  pub fn ir_inst_hash_operator_call(&self, key: &IrInst) -> usize {
    let mut h: u32 = MURMUR_SEED;

    h = Self::mix_u32_u32(h, key.cmd as u32);

    let ops = key.ops.as_slice();
    // 前 min(len, 7) 个槽用实际操作数，余下槽用空 IrOp 填充（对齐 cpp unrolled 循环）
    let n = ops.len().min(HASH_OP_SLOTS);
    for &op in ops[..n].iter() {
      h = Self::mix_u32_ir_op(h, op);
    }
    for _ in n..HASH_OP_SLOTS {
      h = Self::mix_u32_ir_op(h, IrOp { kind_and_index: 0 });
    }

    // MurmurHash2 tail
    h ^= h >> MURMUR_TAIL_SHIFT_1;
    h = h.wrapping_mul(MURMUR_M);
    h ^= h >> MURMUR_TAIL_SHIFT_2;

    h as usize
  }
}

impl DenseHasher<IrInst> for IrInstHash {
  fn hash(&self, key: &IrInst) -> usize {
    self.ir_inst_hash_operator_call(key)
  }
}
