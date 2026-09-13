use ulua_common::records::dense_hash_table::DenseHasher;

use crate::records::{ir_inst::IrInst, ir_op::IrOp};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IrInstHash;

impl IrInstHash {
  pub fn mix_u32_u32(h: u32, k: u32) -> u32 {
    let m: u32 = 0x5bd1e995;
    let r: u32 = 24;

    let mut k: u32 = k;
    k = k.wrapping_mul(m);
    k ^= k >> r;
    k = k.wrapping_mul(m);

    let mut h: u32 = h;
    h = h.wrapping_mul(m);
    h ^= k;

    h
  }

  pub fn mix_u32_ir_op(h: u32, op: IrOp) -> u32 {
    let k: u32 = op.kind_and_index;
    Self::mix_u32_u32(h, k)
  }

  pub fn ir_inst_hash_operator_call(&self, key: &IrInst) -> usize {
    let mut h: u32 = 25;

    h = Self::mix_u32_u32(h, key.cmd as u32);

    let ops_size = key.ops.size();
    for i in 0..7 {
      let op = if i < ops_size {
        key.ops[i as usize]
      } else {
        IrOp { kind_and_index: 0 }
      };
      h = Self::mix_u32_ir_op(h, op);
    }

    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1e995);
    h ^= h >> 15;

    h as usize
  }
}

impl DenseHasher<IrInst> for IrInstHash {
  fn hash(&self, key: &IrInst) -> usize {
    self.ir_inst_hash_operator_call(key)
  }
}
