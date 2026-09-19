use crate::records::ir_inst::IrInst;

#[inline]
pub fn has_op(inst: &IrInst, idx: u32) -> bool {
  idx < inst.ops.size()
}
