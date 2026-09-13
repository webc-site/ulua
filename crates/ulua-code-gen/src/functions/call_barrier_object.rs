use crate::{
  enums::size_x_64::SizeX64,
  functions::check_object_barrier_conditions::check_object_barrier_conditions,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64, ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64, label::Label, native_context::NativeContext,
    operand_x_64::OperandX64, register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64,
    scoped_spills::ScopedSpills,
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

  let mut tmp = ScopedRegX64 {
    owner: regs as *mut _,
    reg: RegisterX64::NOREG,
  };
  tmp.scoped_reg_x_64_ir_reg_alloc_x_64_size_x_64(regs, SizeX64::Qword);

  check_object_barrier_conditions(build, tmp.reg, object, ra, ra_op, ratag, &mut skip);

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
      OperandX64::reg(object),
      object_op,
    );

    call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp);

    let barrierf_offset = core::mem::offset_of!(NativeContext, lua_c_barrierf) as i32;
    let target = OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      r_native_context(),
      barrierf_offset,
    );

    call_wrap.call(&target);
  }

  build.set_label_label(&mut skip);
}
