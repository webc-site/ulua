use core::ptr::from_mut;

use ulua_common::macros::luau_insn_ops::{luau_insn_a, luau_insn_b};
use ulua_vm::{
  enums::lua_type::LuaType, macros::setnvalue::setnvalue, type_aliases::t_value::TValue,
};

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    get_loop_step_k::get_loop_step_k, translate_inst_get_global::translate_global_access,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_builder::{IrBuilder, LoopInfo},
    ir_op::IrOp,
  },
  type_aliases::ir::Instruction,
};

pub fn translate_inst_move(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 指向 proto.code 内一条 MOVE 指令主字(译码器保证在 sizecode 界内、Instruction=u32 对齐);
  // 只读取出该字供 A/B 域提取, 纯读无别名冲突, 取代原先对 *pc 的两次重复读取。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;

  let load_arg = build.vm_reg(rb);
  let load = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, load_arg);

  let store_arg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, store_arg, load);
}

pub fn translate_inst_get_upval(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 由译码器定位到 proto.code 内一条 GETUPVAL 指令起始, 在 sizecode 界内;
  // Instruction 为 u32 且 code 数组按 u32 对齐, 读一次 *pc 即取该指令字(合并原两处 *pc 读)。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let up = luau_insn_b(insn) as u8;

  let vm_upvalue = build.vm_upvalue(up);
  let value = build.inst_ir_cmd_ir_op(IrCmd::GetUpvalue, vm_upvalue);

  let vm_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, vm_reg, value);
}

pub fn translate_inst_set_upval(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 指向 proto.code 内一条 SETUPVAL 指令主字(译码器保证在 sizecode 界内、Instruction=u32 对齐);
  // 只读取出该字供 A/B 域提取, 纯读无别名冲突, 取代原先对 *pc 的两次重复读取。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let up = luau_insn_b(insn) as u8;

  let load_arg = build.vm_reg(ra);
  let value = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, load_arg);

  let upvalue = build.vm_upvalue(up);
  let undef = build.undef();
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::SetUpvalue, upvalue, value, undef);
}

/// 翻译 SETGLOBAL 指令。
pub fn translate_inst_set_global(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_global_access(build, code, pcpos, false);
}

pub fn translate_inst_close_upvals(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // 界内约定:  契约保证 code 切片于 pcpos 指向字节码缓冲内存活、对齐(Instruction=u32)的合法指令字，code[pcpos] 只读取
  // A 域作为 upvalue 关闭边界寄存器，纯读无别名冲突。
  let ra = luau_insn_a(code[pcpos as usize]) as u8;
  let vm_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op(IrCmd::CloseUpvals, vm_reg);
}

/// bit32/math/int64 族 builtin 快速路径的共同收尾（cpp IrTranslateBuiltins.cpp 各翻译器
/// 内联同形尾巴的收敛）：把 number 结果 `StoreDouble` 写入 ra；`ra != arg` 时 check 只
/// 落在 arg 上、ra 的旧 tag 尚未失效，须补写 number tag（`LUA_TNUMBER` = 3）。
pub(crate) fn builtin_store_number_result(build: &mut IrBuilder, ra: i32, arg: i32, value: IrOp) {
  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, value);

  if ra != arg {
    build.store_tag(ra_reg, LuaType::Number as u8);
  }
}

/// 标量三分量向量结果 builtin（vector 构造 / vector.cross / map1 标量路径）的公共
/// 尾部（cpp `buildStoreVector(A, x, y, z) + buildStoreTag(LUA_TVECTOR)` 收敛）：
/// 以 `StoreVector` 存 x/y/z 后打 Vector 标签。
pub(crate) fn builtin_store_vector_result(
  build: &mut IrBuilder,
  ra: i32,
  x: IrOp,
  y: IrOp,
  z: IrOp,
) {
  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, ra_reg, x, y, z);
  build.store_tag(ra_reg, LuaType::Vector as u8);
}

/// TValue 形态向量结果 builtin（map1_x_4 / min_max / lerp / normalize）的公共尾部
/// （cpp `buildStoreTvalue(tagVector(value))` 收敛）：`TagVector` 打标向量值后
/// `StoreTvalue` 整体写回 ra。
pub(crate) fn builtin_store_vector_tvalue_result(build: &mut IrBuilder, ra: i32, value: IrOp) {
  let ra_reg = build.vm_reg(ra as u8);
  let tagged = build.inst_ir_cmd_ir_op(IrCmd::TagVector, value);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, ra_reg, tagged);
}

pub fn after_inst_for_n_loop(build: &mut IrBuilder) {
  CODEGEN_ASSERT!(!build.numeric_loop_stack.is_empty());
  build.numeric_loop_stack.pop();
}

pub fn before_inst_for_n_prep(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // 界内约定:  契约保证 code 切片于 pcpos 指向存活且对齐的 Instruction，code[pcpos] 只读取出合法指令字后交 luau_insn_a
  // 取 A 域；此处为纯读、无别名冲突。
  let ra = luau_insn_a(code[pcpos as usize]) as i32;
  let step_k = get_loop_step_k(build, ra);
  build.numeric_loop_stack.push(LoopInfo {
    step: step_k,
    startpc: pcpos + 1,
  });
}

/// 就地构造一个 number `TValue`（cpp 慢路径 `TValue n; setnvalue(&n, v);` 的定值形）。
///
/// 根因在 `ulua_vm::TValue` 的 union 载荷写入是 `unsafe`（`set_nvalue`，§11 的
/// `enum Value` 路线落地前不可再收），此处把"写局部联合值"这一最小需求收口为
/// 唯一封装点：返回按值所有，调用点无裸指针、无 unsafe、无悬垂窗口。
#[inline]
pub(crate) fn nvalue(n: f64) -> TValue {
  let mut v = TValue::default();
  // Safety: `v` 为活局部；`set_nvalue` 只写 `value.n` union 域与 `tt` 标签，
  // 该值其后仅按 number 读取。
  unsafe { setnvalue!(from_mut(&mut v), n) };
  v
}
