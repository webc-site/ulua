use crate::records::ir_inst_hash::IrInstHash;
use crate::records::ir_op::IrOp;

impl IrInstHash {
    pub fn mix_u32_ir_op(h: u32, op: IrOp) -> u32 {
        let k = op.kind_and_index;
        Self::mix_u32_u32(h, k)
    }
}
