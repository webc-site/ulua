use ulua_bytecode::records::{bc_block::BcBlock, bc_op::BcOp};

pub fn get_op(block: &BcBlock, idx: usize) -> BcOp {
  *block
    .ops
    .get(idx)
    .unwrap_or_else(|| panic!("bytecode block op index {idx} out of range"))
}
