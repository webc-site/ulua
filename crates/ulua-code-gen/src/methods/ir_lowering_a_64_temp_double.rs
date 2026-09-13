use crate::{
  enums::{address_kind_a_64::AddressKindA64, ir_op_kind::IrOpKind, kind_a_64::KindA64},
  functions::get_double_bits::get_double_bits,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    address_a_64::AddressA64, ir_lowering_a_64::IrLoweringA64, ir_op::IrOp,
    register_a_64::RegisterA64,
  },
};
impl IrLoweringA64 {
  pub fn ir_lowering_a_64_temp_double(&mut self, op: IrOp) -> RegisterA64 {
    if op.kind() == IrOpKind::Inst {
      self.ir_lowering_a_64_reg_op(op)
    } else if op.kind() == IrOpKind::Constant {
      let val = self.ir_lowering_a_64_double_op(op);

      if unsafe { (*self.build).is_fmov_supported_fp_64(val) } {
        let temp = self.regs.alloc_temp(KindA64::D);
        unsafe { (*self.build).fmov_register_a_64_f64(temp, val) };
        temp
      } else {
        let temp1 = self.regs.alloc_temp(KindA64::X);
        let temp2 = self.regs.alloc_temp(KindA64::D);

        let vali = get_double_bits(val);

        if (vali << 16) == 0 {
          unsafe {
            (*self.build).movz(temp1, (vali >> 48) as u16, 48);
            (*self.build).fmov_register_a_64_register_a_64(temp2, temp1);
          }
        } else if (vali << 32) == 0 {
          unsafe {
            (*self.build).movz(temp1, (vali >> 48) as u16, 48);
            (*self.build).movk(temp1, (vali >> 32) as u16, 32);
            (*self.build).fmov_register_a_64_register_a_64(temp2, temp1);
          }
        } else {
          unsafe {
            (*self.build).adr_register_a_64_f64(temp1, val);
            (*self.build).ldr(
              temp2,
              AddressA64 {
                kind: AddressKindA64::Imm,
                base: temp1,
                offset: RegisterA64::NOREG,
                data: 0,
              },
            );
          }
        }

        temp2
      }
    } else {
      CODEGEN_ASSERT!(false, "Unsupported instruction form");
      RegisterA64::NOREG
    }
  }
}
