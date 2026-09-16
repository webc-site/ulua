use core::mem::offset_of;

use ulua_vm::{records::closure::Closure, type_aliases::lua_table::LuaTable};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::s_closure::s_closure,
  records::{
    ir_block::IrBlock, ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, operand_x_64::OperandX64,
    register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64,
  },
};

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}

impl IrLoweringX64 {
  pub fn check_safe_env(&mut self, target: IrOp, index: u32, next: &IrBlock) {
    unsafe {
      let mut tmp = ScopedRegX64 {
        owner: &mut self.regs,
        reg: RegisterX64::NOREG,
      };
      tmp.alloc(SizeX64::Qword);

      (*self.build).mov(OperandX64::reg(tmp.reg), s_closure());
      (*self.build).mov(
        OperandX64::reg(tmp.reg),
        mem(SizeX64::Qword, tmp.reg, offset_of!(Closure, env) as i32),
      );
      (*self.build).cmp(
        mem(SizeX64::Byte, tmp.reg, offset_of!(LuaTable, safeenv) as i32),
        OperandX64::imm(0),
      );
    }

    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
      ConditionX64::Equal,
      target,
      index,
      next,
    );
  }
}
