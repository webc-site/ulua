use ulua_common::records::small_vector::SmallVector;

use crate::records::ir_op::IrOp;

pub type Instruction = u32;

pub type IrOps = SmallVector<IrOp, 6>;
