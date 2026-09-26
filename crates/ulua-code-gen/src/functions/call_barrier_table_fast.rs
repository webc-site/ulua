use core::mem::offset_of;

use ulua_vm::{
  macros::{bitmask::bitmask, blackbit::BLACKBIT},
  records::{g_cheader::GCheader, lua_table::LuaTable},
};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::call_vm_helper::call_vm_helper,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64,
    label::Label, native_context::NativeContext, operand_x_64::OperandX64,
    register_x_64::RegisterX64, scoped_spills::ScopedSpills,
  },
};

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
      offset_of!(GCheader, marked) as i32,
    ),
    OperandX64::imm(bitmask(BLACKBIT)),
  );
  build.jcc(ConditionX64::Zero, &mut skip);

  {
    // cpp `ScopedSpills scopedSpills(regs);`：构造即接线 owner 与 spill 恢复下界
    let _spill_guard = ScopedSpills::new(regs);

    let inst_idx = regs.curr_inst_idx;
    // lua_c_barrierback(L, t, gclist)
    call_vm_helper(
      regs,
      build,
      inst_idx,
      &[
        (SizeX64::Qword, OperandX64::reg(table), table_op),
        (
          SizeX64::Qword,
          OperandX64::mem(
            SizeX64::None,
            RegisterX64::NOREG,
            1,
            table,
            offset_of!(LuaTable, gclist) as i32,
          ),
          IrOp::new(),
        ),
      ],
      offset_of!(NativeContext, lua_c_barrierback) as i32,
      false,
    );
  }

  build.set_label_label(&mut skip);
}
