use ulua_vm::records::{
  closure::{Closure, LClosure},
  proto::Proto,
};

use crate::{
  enums::size_x_64::SizeX64,
  records::{
    ir_lowering_x_64::IrLoweringX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
    scoped_reg_x_64::ScopedRegX64,
  },
};

impl IrLoweringX64 {
  pub fn increment_counter_at(&mut self, offset: usize) {
    unsafe {
      let mut tmp = ScopedRegX64 {
        owner: &mut self.regs,
        reg: RegisterX64::NOREG,
      };
      tmp.alloc(SizeX64::Qword);

      // Get counter slot
      (*self.build).mov(OperandX64::reg(tmp.reg), OperandX64::reg(RegisterX64::RDI));
      (*self.build).mov(
        OperandX64::reg(tmp.reg),
        OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          tmp.reg,
          (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32,
        ),
      );
      (*self.build).mov(
        OperandX64::reg(tmp.reg),
        OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          tmp.reg,
          core::mem::offset_of!(Proto, execdata) as i32,
        ),
      );

      // Increment
      (*self.build).inc(OperandX64::mem(
        SizeX64::Qword,
        RegisterX64::NOREG,
        1,
        tmp.reg,
        (((*(*self.function).proto).sizecode as u32 + offset as u32) * 4) as i32,
      ));
    }
  }
}
