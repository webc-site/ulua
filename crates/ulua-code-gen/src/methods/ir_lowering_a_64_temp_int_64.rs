use crate::{
  enums::{ir_op_kind::IrOpKind, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp, register_a_64::RegisterA64},
};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_temp_int_64(&mut self, op: IrOp) -> RegisterA64 {
    if op.kind() == IrOpKind::Inst {
      self.ir_lowering_a_64_reg_op(op)
    } else if op.kind() == IrOpKind::Constant {
      let temp = self.regs.alloc_temp(KindA64::X);
      let u: u64 = self.ir_lowering_a_64_int_64_op(op) as u64;

      // Count non-zero halfwords (movz path) vs non-0xFFFF halfwords (movn path)
      let mut movz_count = 0;
      let mut movn_count = 0;
      for shift in (0..64).step_by(16) {
        let hw = (u >> shift) as u16;
        if hw != 0 {
          movz_count += 1;
        }
        if hw != 0xFFFF {
          movn_count += 1;
        }
      }

      if movz_count <= movn_count {
        // movz path: emit movz for first non-zero halfword, movk for rest
        let mut first = true;
        for shift in (0..64).step_by(16) {
          let hw = (u >> shift) as u16;
          if hw != 0 {
            if first {
              unsafe { (*self.build).movz(temp, hw, shift) };
              first = false;
            } else {
              unsafe { (*self.build).movk(temp, hw, shift) };
            }
          }
        }

        if first {
          unsafe { (*self.build).movz(temp, 0, 0) };
        }
      } else {
        // movn path: use movn for first non-0xFFFF halfword, movk for rest
        let mut first = true;
        for shift in (0..64).step_by(16) {
          let hw = (u >> shift) as u16;
          if hw != 0xFFFF {
            if first {
              unsafe { (*self.build).movn(temp, !hw, shift) };
              first = false;
            } else {
              unsafe { (*self.build).movk(temp, hw, shift) };
            }
          }
        }

        if first {
          unsafe { (*self.build).movn(temp, 0, 0) };
        }
      }

      temp
    } else {
      CODEGEN_ASSERT!(false, "Unsupported instruction form");
      RegisterA64::NOREG
    }
  }
}
