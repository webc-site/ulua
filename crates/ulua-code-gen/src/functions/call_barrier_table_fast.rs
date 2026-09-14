use ulua_vm::{
  macros::{bitmask::bitmask, blackbit::BLACKBIT},
  records::{g_cheader::GCheader, lua_table::LuaTable},
};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64, ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64, label::Label, native_context::NativeContext,
    operand_x_64::OperandX64, register_x_64::RegisterX64, scoped_spills::ScopedSpills,
  },
};

const fn r_state() -> RegisterX64 {
  RegisterX64 {
    bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

const fn r_native_context() -> RegisterX64 {
  RegisterX64 {
    bits: (13u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

pub fn call_barrier_table_fast(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  table: RegisterX64,
  table_op: IrOp,
) {
  let mut skip = Label { id: 0, location: 0 };

  // isblack(obj2gco(t))
  build.test(
    OperandX64::mem(
      SizeX64::Byte,
      RegisterX64::NOREG,
      1,
      table,
      core::mem::offset_of!(GCheader, marked) as i32,
    ),
    OperandX64::imm(bitmask(BLACKBIT as i32)),
  );
  build.jcc(ConditionX64::Zero, &mut skip);

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
      IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, regs.curr_inst_idx);

    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(r_state()),
      IrOp::new(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(table),
      table_op,
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::mem(
        SizeX64::None,
        RegisterX64::NOREG,
        1,
        table,
        core::mem::offset_of!(LuaTable, gclist) as i32,
      ),
      IrOp::new(),
    );

    call_wrap.call(&OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      r_native_context(),
      core::mem::offset_of!(NativeContext, lua_c_barrierback) as i32,
    ));
  }

  build.set_label_label(&mut skip);
}
