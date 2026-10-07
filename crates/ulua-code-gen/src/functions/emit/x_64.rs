use core::mem::{offset_of, size_of};

use ulua_vm::{enums::lua_type::LuaType, type_aliases::t_value::TValue};

use crate::{
  enums::{condition_x_64::ConditionX64, ir_cmd::IrCmd, size_x_64::SizeX64, x_64::CategoryX64},
  functions::native_abi::K_STACK_OFFSET_TO_LOCALS,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    ir_builder::IrBuilder,
    ir_op::IrOp,
    label::Label,
    operand_x_64::{ADDR, OperandX64},
    register_x_64::RegisterX64,
  },
};

pub fn jump_if_falsy(
  build: &mut AssemblyBuilderX64,
  ri: i32,
  target: &mut Label,
  fallthrough: &mut Label,
) {
  jump_if_tag_is(build, ri, LuaType::Nil, target); // false if nil
  jump_if_tag_is_not(build, ri, LuaType::Boolean, fallthrough); // true if not nil or boolean

  build.cmp(luau_reg_value_int(ri), 0.into());
  build.jcc(ConditionX64::Equal, target); // true if boolean value is 'true'
}

pub fn jump_if_truthy(
  build: &mut AssemblyBuilderX64,
  ri: i32,
  target: &mut Label,
  fallthrough: &mut Label,
) {
  jump_if_tag_is(build, ri, LuaType::Nil, fallthrough); // false if nil
  jump_if_tag_is_not(build, ri, LuaType::Boolean, target); // true if not nil or boolean

  build.cmp(luau_reg_value_int(ri), 0.into());
  build.jcc(ConditionX64::NotEqual, target); // true if boolean value is 'true'
}

fn jump_if_tag_is(build: &mut AssemblyBuilderX64, ri: i32, tag: LuaType, label: &mut Label) {
  build.cmp(luau_reg_tag(ri), (tag as i32).into());
  build.jcc(ConditionX64::Equal, label);
}

fn jump_if_tag_is_not(
  build: &mut AssemblyBuilderX64,
  ri: i32,
  tag: LuaType,
  label: &mut Label,
) {
  build.cmp(luau_reg_tag(ri), (tag as i32).into());
  build.jcc(ConditionX64::NotEqual, label);
}

pub fn set_luau_reg(build: &mut AssemblyBuilderX64, tmp: RegisterX64, ri: i32, op: OperandX64) {
  debug_assert!(op.cat == CategoryX64::Mem);

  build.vmovups(OperandX64::reg(tmp), op);
  build.vmovups(luau_reg(ri), OperandX64::reg(tmp));
}

#[inline]
pub fn luau_reg(ri: i32) -> OperandX64 {
  // sizeof(TValue) 为 16 字节。
  let tvalue_size = size_of::<TValue>() as i32;

  // xmmword 对应 128 位（16 字节）访问。
  OperandX64::mem(
    SizeX64::Xmmword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R14,
    ri * tvalue_size,
  )
}

#[inline]
pub fn luau_reg_address(ri: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;

  OperandX64::mem(
    SizeX64::None,
    RegisterX64::NOREG,
    1,
    RegisterX64::R14,
    ri * tvalue_size,
  )
}

#[inline]
pub fn luau_reg_tag(ri: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;
  let tt_offset = offset_of!(TValue, tt) as i32;

  OperandX64::mem(
    SizeX64::Dword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R14,
    ri * tvalue_size + tt_offset,
  )
}

#[inline]
pub fn luau_reg_value(ri: i32) -> OperandX64 {
  // Luau VM 中 TValue 为 16 字节（Value value + int extra[2] + int tt）
  let tvalue_size = size_of::<TValue>() as i32;
  // offsetof(TValue, value) 为 0
  let value_offset = 0;

  OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R14,
    ri * tvalue_size + value_offset,
  )
}

/// 返回 Luau 寄存器中 TValue 整数部分对应的操作数。
///
/// C++: dword[rBase + ri * sizeof(TValue) + offsetof(TValue, value)]
///
#[inline]
pub fn luau_reg_value_int(ri: i32) -> OperandX64 {
  OperandX64::mem(
    SizeX64::Dword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R14,
    ri * 16,
  )
}

#[inline]
pub fn luau_reg_value_vector(ri: i32, index: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;
  let value_offset = 0;
  let float_size = size_of::<f32>() as i32;

  OperandX64::mem(
    SizeX64::Dword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R14,
    ri * tvalue_size + value_offset + (float_size * index),
  )
}

#[inline]
pub fn luau_constant(ki: i32) -> OperandX64 {
  // cpp EmitCommonX64.h luauConstant: xmmword[rConstants + ki * sizeof(TValue)], rConstants=r12
  OperandX64::mem(
    SizeX64::Xmmword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R12,
    ki * size_of::<TValue>() as i32,
  )
}

pub fn luau_constant_address(ki: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;

  ADDR.with_index(RegisterX64::R12 + ki * tvalue_size)
}

#[inline]
pub fn luau_constant_tag(ki: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;
  let tt_offset = offset_of!(TValue, tt) as i32;

  // cpp EmitCommonX64.h luauConstantTag: dword[rConstants + ki*sizeof(TValue) + offsetof(TValue,tt)], rConstants=r12
  OperandX64::mem(
    SizeX64::Dword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R12,
    ki * tvalue_size + tt_offset,
  )
}

// kOffsetOfTKeyTagNext = 12（来自 EmitCommon.h）
// offsetof(LuaNode, key) = 16（LuaNode 占 32 字节，前 16 字节是 val，之后是 key）
// 因此偏移为 16 + 12 = 28
// 使用 dword[base + disp] 形式
pub const fn luau_node_key_tag(node: RegisterX64) -> OperandX64 {
  OperandX64::mem(SizeX64::Dword, RegisterX64::NOREG, 0, node, 28)
}

#[inline]
pub fn luau_node_key_value(node: RegisterX64) -> OperandX64 {
  // offsetof(LuaNode, key) = 16（LuaNode 占 32 字节，前 16 字节是 base，之后是 key）
  // offsetof(TKey, value) = 0（TKey 以 Value value 开头）
  // 因此偏移为 16 + 0 = 16
  // 使用 qword[base + disp] 形式
  OperandX64::mem(SizeX64::Qword, RegisterX64::NOREG, 0, node, 16)
}

// Source: `CodeGen/src/EmitCommonX64.h:88` — `inline constexpr OperandX64
// sClosure = qword[rsp + kStackOffsetToLocals + 0]` (Closure* cl)。
// 原先在 emit_return / build_entry / emit_inst_call / check_safe_env 各复制
// 一份 fn, 收敛于此。

#[inline]
pub fn s_closure() -> OperandX64 {
  OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS as i32,
  )
}

// Source: `CodeGen/src/EmitCommonX64.h:89` — `inline constexpr OperandX64 sCode =
// qword[rsp + kStackOffsetToLocals + 8]` (Instruction* code)。
// 原先在 7 个 emit/lowering 文件里各复制一份 fn, 收敛于此。

#[inline]
pub fn s_code() -> OperandX64 {
  OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS as i32 + 8,
  )
}

pub fn convert_number_to_index_or_jump(
  build: &mut AssemblyBuilderX64,
  tmp: RegisterX64,
  numd: RegisterX64,
  numi: RegisterX64,
  label: &mut Label,
) {
  CODEGEN_ASSERT!(numi.size() == SizeX64::Dword);

  build.vcvttsd2si(numi.into(), numd.into());
  build.vcvtsi2sd(tmp.into(), numd.into(), numi.into());
  build.vucomisd(tmp.into(), numd.into());
  build.jcc(ConditionX64::NotZero, label);
}

/// tag 直发守卫收口（binary/minus/gettableks 六站同构，userdata 直发臂同型）：
/// `tb_op` 不匹配 `tag` 即 vm_exit，随后 `CheckTag` 校验。
#[inline]
pub(crate) fn check_tag_exit(build: &mut IrBuilder, tb_op: IrOp, tag: u8, pcpos: i32) {
  let tag_op = build.const_tag(tag);
  let exit = build.vm_exit(pcpos as u32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb_op, tag_op, exit);
}
