use core::mem::offset_of;

use ulua_vm::records::{
  call_info::CallInfo, global_state::global_State, lua_callbacks::LuaCallbacks, lua_state::LuaState,
};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::{
    byte_reg::byte_reg, dword_reg::dword_reg, emit_exit_emit_common_x_64::emit_exit,
    emit_update_base_emit_common_x_64::emit_update_base, s_code::s_code,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, emit_common_x_64::R_STATE,
    ir_call_wrapper_x_64::IrCallWrapperX64, label::Label, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

// C++ EmitCommonX64.h: `constexpr RegisterX64 rState = r15;`
const SIZEOF_INSTRUCTION: i32 = 4;

pub fn emit_interrupt(build: &mut AssemblyBuilderX64) {
  // rax = pcpos + 1
  // rbx = native 代码中的返回地址
  // 注：rbx 非易失，interrupt 调用期间自动保存

  // cpp EmitCommonX64.cpp:380-381 无条件使用 suggestArgumentRegister（移植期开关
  // LuauCodegenSuggestArgumentRegisterX64 在 cpp 中不存在，两分支结果恒等，移除）。
  let r_arg1 = IrCallWrapperX64::suggest_argument_register::<0>(SizeX64::Qword, build);
  let r_arg2 = IrCallWrapperX64::suggest_argument_register::<1>(SizeX64::Qword, build);

  let mut skip = Label { id: 0, location: 0 };

  // 更新 l->ci->savedpc；interrupt 报错时必需
  build.mov(OperandX64::reg(RegisterX64::RCX), s_code());
  build.lea_operand_x_64_operand_x_64(
    OperandX64::reg(RegisterX64::RCX),
    OperandX64::mem(
      SizeX64::None,
      RegisterX64::RAX,
      SIZEOF_INSTRUCTION as u8,
      RegisterX64::RCX,
      0,
    ),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      R_STATE,
      offset_of!(LuaState, ci) as i32,
    ),
  );
  build.mov(
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RAX,
      offset_of!(CallInfo, savedpc) as i32,
    ),
    OperandX64::reg(RegisterX64::RCX),
  );

  // 加载 interrupt handler；到达此处前它可能与检查竞态而为 nullptr
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      R_STATE,
      offset_of!(LuaState, global) as i32,
    ),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RAX,
      (offset_of!(global_State, cb) + offset_of!(LuaCallbacks, interrupt)) as i32,
    ),
  );
  build.test(
    OperandX64::reg(RegisterX64::RAX),
    OperandX64::reg(RegisterX64::RAX),
  );
  build.jcc(ConditionX64::Zero, &mut skip);

  // 调用 interrupt
  build.mov(OperandX64::reg(r_arg1), OperandX64::reg(R_STATE));
  build.mov(OperandX64::reg(dword_reg(r_arg2)), OperandX64::imm(-1));
  build.call_operand_x_64(OperandX64::reg(RegisterX64::RAX));

  // 检查是否需要退出
  build.mov(
    OperandX64::reg(byte_reg(RegisterX64::RAX)),
    OperandX64::mem(
      SizeX64::Byte,
      RegisterX64::NOREG,
      1,
      R_STATE,
      offset_of!(LuaState, status) as i32,
    ),
  );
  build.test(
    OperandX64::reg(byte_reg(RegisterX64::RAX)),
    OperandX64::reg(byte_reg(RegisterX64::RAX)),
  );
  build.jcc(ConditionX64::Zero, &mut skip);

  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      R_STATE,
      offset_of!(LuaState, ci) as i32,
    ),
  );
  build.sub(
    OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RAX,
      offset_of!(CallInfo, savedpc) as i32,
    ),
    OperandX64::imm(SIZEOF_INSTRUCTION),
  );
  emit_exit(build, /* continue_in_vm */ false);

  build.set_label_label(&mut skip);

  emit_update_base(build); // interrupt may have reallocated stack

  build.jmp_operand_x_64(OperandX64::reg(RegisterX64::RBX));
}
