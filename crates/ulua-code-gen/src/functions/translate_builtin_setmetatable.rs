//! LBF_SETMETATABLE 的 JIT 快速路径（本 fork 扩展，cpp 无对应）。
//!
//! oop 形负载归因：每迭代 4 次 `setmetatable` FASTCALL2 走 call_fallback 回落解释器
//! （≈19% 样本）+ `luaB_setmetatable` 内 `lua_pushlstring("__metatable")` 字符串驻留
//! （≈6%）。此处内联为 `SetMetatableChecked` 直调 RT 快速通道（`lua_codegen_setmetatable`）：
//! 类型守卫与 `__metatable` 保护判定都在快速通道内完成；任一前置不满足返回 0 跳
//! fallback 块，由解释器路径按原语义抛错（消息与时机保真）。

use ulua_common::fflag;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind, lowering::BuiltinImplType},
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_setmetatable(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  fallback: IrOp,
) -> BuiltinImplResult {
  // 仅双参形态（setmetatable(obj, mt)）；其余（变参/FASTCALL2K 常量元表等）走 fallback。
  if nparams != 2 || args.kind() != IrOpKind::VmReg {
    return BuiltinImplResult::NONE_FALLBACK;
  }
  // A/B 旗标：关闭即整体回退解释器 fallback 形态
  if !fflag::LuauJitSetmetatableFastcall.get() {
    return BuiltinImplResult::NONE_FALLBACK;
  }
  // setmetatable 至多返回 1 值；>1 的请求形态非法，交回解释器。
  if nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let obj_reg = build.vm_reg(arg as u8);
  let mt_reg = args;

  // 赋值段直调（含 obj/mt 类型守卫、__metatable 保护检查、readonly 抛错、
  // 字段写 + objbarrier）；返回 0 → fallback。
  let reg_ra = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::SetMetatableChecked,
    reg_ra,
    obj_reg,
    mt_reg,
    fallback,
  );

  // 结果：R(A) = obj（setmetatable 首参回传；obj tag 已由快速通道确认为 Table，
  // 全 TValue 拷贝含 tag）。
  let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, obj_reg);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, tva);

  // 错误路径须保留 fallback 块（解释器重执行 FASTCALL 序列，错误保真）。
  BuiltinImplResult::new(BuiltinImplType::UsesFallback, 1)
}
