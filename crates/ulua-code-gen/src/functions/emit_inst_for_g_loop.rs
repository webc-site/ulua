use core::mem::size_of;

use ulua_common::FFlag::{LuauCodeGenCallWrapperEmitInst, LuauCodegenSuggestArgumentRegisterX64};
use ulua_vm::{
  enums::lua_type::LuaType, records::lua_table::LuaTable, type_aliases::t_value::TValue,
};

use crate::{
  enums::{abix_64::ABIX64, condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::{
    luau_reg_address::luau_reg_address, luau_reg_tag::luau_reg_tag, luau_reg_value::luau_reg_value,
    set_luau_reg::set_luau_reg,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64, label::Label,
    native_context::NativeContext, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

pub fn emit_inst_for_g_loop(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  ra: i32,
  aux: i32,
  loop_repeat: &mut Label,
) {
  crate::macros::codegen_assert::CODEGEN_ASSERT!(aux >= 0);

  let (table, index) = if LuauCodegenSuggestArgumentRegisterX64.get() {
    (
      IrCallWrapperX64::suggest_argument_register::<1>(SizeX64::Qword, build),
      IrCallWrapperX64::suggest_argument_register::<2>(SizeX64::Qword, build),
    )
  } else if build.abi == ABIX64::WINDOWS {
    (RegisterX64::RDX, RegisterX64::R8)
  } else {
    (RegisterX64::RSI, RegisterX64::RDX)
  };

  let elem_ptr = RegisterX64::RAX;

  build.mov(OperandX64::reg(table), luau_reg_value(ra + 1));
  build.mov(OperandX64::reg(index), luau_reg_value(ra + 2));

  build.mov(
    OperandX64::reg(sized(elem_ptr, SizeX64::Dword)),
    OperandX64::reg(sized(index, SizeX64::Dword)),
  );
  build.shl(
    OperandX64::reg(sized(elem_ptr, SizeX64::Dword)),
    OperandX64::imm(K_TVALUE_SIZE_LOG2),
  );
  build.add(
    OperandX64::reg(elem_ptr),
    mem(
      SizeX64::Qword,
      table,
      core::mem::offset_of!(LuaTable, array) as i32,
    ),
  );

  for i in 2..aux {
    build.mov(
      luau_reg_tag(ra + 3 + i),
      OperandX64::imm(LuaType::Nil as i32),
    );
  }

  let mut skip_array = Label::default();
  let mut skip_array_nil = Label::default();

  let mut array_loop = Label::default();
  build.set_label(&mut array_loop);
  build.cmp(
    OperandX64::reg(sized(index, SizeX64::Dword)),
    mem(
      SizeX64::Dword,
      table,
      core::mem::offset_of!(LuaTable, sizearray) as i32,
    ),
  );
  build.jcc(ConditionX64::NotBelow, &mut skip_array);

  build.inc(OperandX64::reg(index));

  build.cmp(
    mem(
      SizeX64::Dword,
      elem_ptr,
      core::mem::offset_of!(TValue, tt) as i32,
    ),
    OperandX64::imm(LuaType::Nil as i32),
  );
  build.jcc(ConditionX64::Equal, &mut skip_array_nil);

  build.mov(luau_reg_value(ra + 2), OperandX64::reg(index));

  build.vcvtsi2sd(
    OperandX64::reg(xmm(0)),
    OperandX64::reg(xmm(0)),
    OperandX64::reg(sized(index, SizeX64::Dword)),
  );
  build.vmovsd_operand_x_64_operand_x_64(luau_reg_value(ra + 3), OperandX64::reg(xmm(0)));
  build.mov(
    luau_reg_tag(ra + 3),
    OperandX64::imm(LuaType::Number as i32),
  );

  set_luau_reg(build, xmm(2), ra + 4, mem(SizeX64::Xmmword, elem_ptr, 0));

  build.jmp_label(loop_repeat);

  build.set_label_label(&mut skip_array_nil);
  build.add(
    OperandX64::reg(elem_ptr),
    OperandX64::imm(size_of::<TValue>() as i32),
  );
  build.jmp_label(&mut array_loop);

  build.set_label_label(&mut skip_array);

  if LuauCodeGenCallWrapperEmitInst.get() {
    regs.take_reg(table, K_INVALID_INST_IDX);
    regs.take_reg(index, K_INVALID_INST_IDX);

    let mut call_wrapper =
      IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, K_INVALID_INST_IDX);
    call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(r_state()),
      IrOp::new(),
    );
    call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(table),
      IrOp::new(),
    );
    call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(index),
      IrOp::new(),
    );
    call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      luau_reg_address(ra),
      IrOp::new(),
    );
    call_wrapper.call(&native_context_slot(
      core::mem::offset_of!(NativeContext, forg_loop_node_iter) as i32,
    ));
  } else {
    let r_arg1 = if build.abi == ABIX64::WINDOWS {
      RegisterX64::RCX
    } else {
      RegisterX64::RDI
    };
    let r_arg4 = if build.abi == ABIX64::WINDOWS {
      RegisterX64::R9
    } else {
      RegisterX64::RCX
    };

    build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
    build.lea_operand_x_64_operand_x_64(OperandX64::reg(r_arg4), luau_reg_address(ra));
    build.call_operand_x_64(native_context_slot(core::mem::offset_of!(
      NativeContext,
      forg_loop_node_iter
    ) as i32));
  }

  build.test(
    OperandX64::reg(sized(RegisterX64::RAX, SizeX64::Byte)),
    OperandX64::reg(sized(RegisterX64::RAX, SizeX64::Byte)),
  );
  build.jcc(ConditionX64::NotZero, loop_repeat);
}

const K_TVALUE_SIZE_LOG2: i32 = 4;

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
  RegisterX64 {
    bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
  }
}

const fn sized(reg: RegisterX64, size: SizeX64) -> RegisterX64 {
  RegisterX64 {
    bits: (reg.index() << RegisterX64::INDEX_SHIFT) | size as u8,
  }
}

const fn xmm(index: u8) -> RegisterX64 {
  reg(index, SizeX64::Xmmword)
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

fn native_context_slot(disp: i32) -> OperandX64 {
  mem(SizeX64::Qword, r_native_context(), disp)
}
