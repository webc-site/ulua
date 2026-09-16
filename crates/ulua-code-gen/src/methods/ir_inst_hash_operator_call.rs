use crate::records::ir_inst::IrInst;
use crate::records::ir_inst_hash::IrInstHash;
use crate::records::ir_op::IrOp;

impl IrInstHash {
    pub fn ir_inst_hash_operator_call(&self, key: &IrInst) -> usize {
        let mut h: u32 = 25;

        h = Self::mix_u32_u32(h, key.cmd as u32);

        let ops_size = key.ops.size() as u32;
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
