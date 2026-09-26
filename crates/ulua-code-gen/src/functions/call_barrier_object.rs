use core::mem::offset_of;

use crate::{
  enums::size_x_64::SizeX64,
  functions::check_object_barrier_conditions::check_object_barrier_conditions,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64::{R_NATIVE_CONTEXT, R_STATE},
    ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64,
    label::Label,
    native_context::NativeContext,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
    scoped_reg_x_64::ScopedRegX64,
    scoped_spills::ScopedSpills,
  },
};

pub fn call_barrier_object(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  object: RegisterX64,
  object_op: IrOp,
  ra: RegisterX64,
  ra_op: IrOp,
  ratag: i32,
) {
  let mut skip = Label { id: 0, location: 0 };

  let mut tmp = ScopedRegX64::with_size(regs, SizeX64::Qword);

  check_object_barrier_conditions(build, tmp.reg, object, ra, ra_op, ratag, &mut skip);

  {
    // cpp `ScopedSpills scopedSpills(regs);`：构造即接线 owner 与 spill 恢复下界
    let _spill_guard = ScopedSpills::new(regs);

    let mut call_wrap =
      IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, regs.curr_inst_idx);

    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(R_STATE),
      IrOp::new(),
    );

    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(object),
      object_op,
    );

    call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp);

    let barrierf_offset = offset_of!(NativeContext, lua_c_barrierf) as i32;
    let target = OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      R_NATIVE_CONTEXT,
      barrierf_offset,
    );

    call_wrap.call(&target);
  }

  build.set_label_label(&mut skip);
}
