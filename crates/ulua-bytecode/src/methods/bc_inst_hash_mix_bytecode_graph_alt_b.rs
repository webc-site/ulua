use crate::records::{bc_inst_hash::BcInstHash, bc_op::BcOp};

impl BcInstHash {
  pub fn mix_u32_bc_op(h: u32, op: BcOp) -> u32 {
    let k = ((op.kind as u32) & 0x0F) | (op.index << 4);
    Self::mix_u32_u32(h, k)
  }
}
