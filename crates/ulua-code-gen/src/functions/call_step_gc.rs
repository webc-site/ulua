use core::ptr::null_mut;

use ulua_vm::records::{global_state::global_State, lua_state::lua_State};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::emit_update_base_emit_common_x_64::emit_update_base,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64, label::Label,
    native_context::NativeContext, operand_x_64::OperandX64, register_x_64::RegisterX64,
    scoped_reg_x_64::ScopedRegX64, scoped_spills::ScopedSpills,
  },
};

pub fn call_step_gc(regs: &mut IrRegAllocX64, build: &mut AssemblyBuilderX64) {
  let mut skip = Label::default();

  {
    let mut tmp1 = ScopedRegX64 {
      owner: null_mut(),
      reg: RegisterX64::NOREG,
    };
    tmp1.scoped_reg_x_64_ir_reg_alloc_x_64_size_x_64(regs, SizeX64::Qword);

    let mut tmp2 = ScopedRegX64 {
      owner: null_mut(),
      reg: RegisterX64::NOREG,
    };
    tmp2.scoped_reg_x_64_ir_reg_alloc_x_64_size_x_64(regs, SizeX64::Qword);

    build.mov(
      OperandX64::reg(tmp1.reg),
      mem(
        SizeX64::Qword,
        r_state(),
        core::mem::offset_of!(lua_State, global) as i32,
      ),
    );
    build.mov(
      OperandX64::reg(tmp2.reg),
      mem(
        SizeX64::Qword,
        tmp1.reg,
        core::mem::offset_of!(global_State, totalbytes) as i32,
      ),
    );
    build.cmp(
      OperandX64::reg(tmp2.reg),
      mem(
        SizeX64::Qword,
        tmp1.reg,
        core::mem::offset_of!(global_State, gc_threshold) as i32,
      ),
    );
    build.jcc(ConditionX64::Below, &mut skip);
  }

  {
    let _spill_guard = {
      let mut guard = ScopedSpills {
        owner: regs as *mut _,
        start_spill_id: 0,
      };
      guard.scoped_spills_scoped_spills_ir_reg_alloc_x_64(regs);
      guard
    };

    let mut call_wrap =
      IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, K_INVALID_INST_IDX);
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(r_state()),
      IrOp::new(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Dword,
      OperandX64::imm(1),
      IrOp::new(),
    );
    call_wrap.call(&mem(
      SizeX64::Qword,
      r_native_context(),
      core::mem::offset_of!(NativeContext, lua_c_step) as i32,
    ));
    emit_update_base(build);
  }

  build.set_label_label(&mut skip);
}

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
  RegisterX64 {
    bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
  }
}

const fn r_state() -> RegisterX64 {
  reg(15, SizeX64::Qword)
}

const fn r_native_context() -> RegisterX64 {
  reg(13, SizeX64::Qword)
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}
