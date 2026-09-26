use core::mem::offset_of;

use ulua_vm::records::{global_state::global_State, lua_state::LuaState};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::call_vm_helper::call_vm_helper,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, emit_common_x_64::R_STATE,
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64, label::Label,
    native_context::NativeContext, operand_x_64::OperandX64, register_x_64::RegisterX64,
    scoped_reg_x_64::ScopedRegX64, scoped_spills::ScopedSpills,
  },
};

pub fn call_step_gc(regs: &mut IrRegAllocX64, build: &mut AssemblyBuilderX64) {
  let mut skip = Label::default();

  {
    let tmp1 = ScopedRegX64::with_size(regs, SizeX64::Qword);
    let tmp2 = ScopedRegX64::with_size(regs, SizeX64::Qword);

    build.mov(
      OperandX64::reg(tmp1.reg),
      mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, global) as i32),
    );
    build.mov(
      OperandX64::reg(tmp2.reg),
      mem(
        SizeX64::Qword,
        tmp1.reg,
        offset_of!(global_State, totalbytes) as i32,
      ),
    );
    build.cmp(
      OperandX64::reg(tmp2.reg),
      mem(
        SizeX64::Qword,
        tmp1.reg,
        offset_of!(global_State, gc_threshold) as i32,
      ),
    );
    build.jcc(ConditionX64::Below, &mut skip);
  }

  {
    // cpp `ScopedSpills scopedSpills(regs);`：构造即接线 owner 与 spill 恢复下界
    let _spill_guard = ScopedSpills::new(regs);

    // lua_c_step(L, 1)
    call_vm_helper(
      regs,
      build,
      K_INVALID_INST_IDX,
      &[(SizeX64::Dword, OperandX64::imm(1), IrOp::new())],
      offset_of!(NativeContext, lua_c_step) as i32,
      true,
    );
  }

  build.set_label_label(&mut skip);
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}
