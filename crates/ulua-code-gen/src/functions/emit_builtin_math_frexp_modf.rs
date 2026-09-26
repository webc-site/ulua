//! `math.frexp` / `math.modf` 的 X64 内建发射孪生收敛。
//!
//! 两者同为「XMM0 主返回 + `qword [rsp]` 栈草稿槽指针回写次返回」的双返回
//! libm 调用，仅分量落位互换：
//! - frexp：主值 = 尾数（XMM0）；次值 = 槽内 `int` 指数，`vcvtsi2sd` 转双精度；
//! - modf：主值 = 小数部分（槽内 double，经 XMM1 中转）；次值 = 整数部分（XMM0）。
//!
//! 指令序列与拆分前逐条一致（含 XMM0 被 `vcvtsi2sd` 就地覆写——主值已先行落 `ra`）。
//! 参考 cpp `CodeGen/src/EmitBuiltinX64.cpp` emitBuiltinMathFrexp / emitBuiltinMathModf。

use core::mem::offset_of;

use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::size_x_64::SizeX64,
  functions::{luau_reg_tag::luau_reg_tag, luau_reg_value::luau_reg_value},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, emit_common_x_64::R_NATIVE_CONTEXT,
    ir_call_wrapper_x_64::IrCallWrapperX64, ir_data::K_INVALID_INST_IDX, ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64, native_context::NativeContext, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

/// 双返回分量落位布局（决定主/次结果分别从 XMM0 还是草稿槽取回）。
#[derive(Clone, Copy)]
pub enum FrexpModfLayout {
  /// `frexp`：主值=尾数（XMM0），次值=槽内 int 指数。
  Frexp,
  /// `modf`：主值=槽内小数（XMM1 中转），次值=整数部分（XMM0）。
  Modf,
}

/// `[rsp]` 栈草稿槽：承载 libm 经指针回写的次返回分量（frexp 的 int 指数用
/// Dword、modf/frexp 的 double 分量用 Qword）。
fn s_temporary_slot(size: SizeX64) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, RegisterX64::RSP, 0)
}

/// 发射 frexp/modf 的 libm 调用与结果落位（`nresults > 1` 时才落次值）。
pub fn emit_builtin_math_frexp_modf(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  ra: i32,
  arg: i32,
  nresults: i32,
  layout: FrexpModfLayout,
) {
  let (libm_offset, frexp) = match layout {
    FrexpModfLayout::Frexp => (offset_of!(NativeContext, libm_frexp), true),
    FrexpModfLayout::Modf => (offset_of!(NativeContext, libm_modf), false),
  };

  let mut call_wrap =
    IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, K_INVALID_INST_IDX);
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Xmmword,
    luau_reg_value(arg),
    IrOp::new(),
  );
  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    s_temporary_slot(SizeX64::Qword),
    IrOp::new(),
  );
  call_wrap.call(&OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    R_NATIVE_CONTEXT,
    libm_offset as i32,
  ));

  if frexp {
    // 主值 = 尾数（XMM0）；次值 = 槽内 int 指数 → cvtsi2sd 就地覆写 XMM0
    build.vmovsd_operand_x_64_operand_x_64(luau_reg_value(ra), OperandX64::reg(RegisterX64::XMM0));
    build.mov(luau_reg_tag(ra), OperandX64::imm(LuaType::Number as i32));
    if nresults > 1 {
      build.vcvtsi2sd(
        OperandX64::reg(RegisterX64::XMM0),
        OperandX64::reg(RegisterX64::XMM0),
        s_temporary_slot(SizeX64::Dword),
      );
    }
  } else {
    // 主值 = 槽内小数部分（XMM1 中转）；次值（整数部分）已在 XMM0
    build.vmovsd_operand_x_64_operand_x_64(
      OperandX64::reg(RegisterX64::XMM1),
      s_temporary_slot(SizeX64::Qword),
    );
    build.vmovsd_operand_x_64_operand_x_64(luau_reg_value(ra), OperandX64::reg(RegisterX64::XMM1));
    build.mov(luau_reg_tag(ra), OperandX64::imm(LuaType::Number as i32));
  }

  // 次值落位：两种布局此刻均已把次值备好在 XMM0
  if nresults > 1 {
    build
      .vmovsd_operand_x_64_operand_x_64(luau_reg_value(ra + 1), OperandX64::reg(RegisterX64::XMM0));
    build.mov(
      luau_reg_tag(ra + 1),
      OperandX64::imm(LuaType::Number as i32),
    );
  }
}
