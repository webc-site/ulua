use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

/// `bit32.*` 无符号 32 位域宽，移位/位域抽取的越界上界
pub(crate) const K_UINT32_WIDTH: i32 = 32;

/// int64 位宽，`integer.*` 位域抽取的宽度上界
pub(crate) const K_INT64_WIDTH: i64 = 64;

/// int64 最大合法移位量（`K_INT64_WIDTH - 1`）
pub(crate) const K_INT64_MAX_SHIFT: i64 = K_INT64_WIDTH - 1;

/// 取值范围守卫的同形骨架：`CHECK lhs COND rhs`，不满足时回退 `vm_exit(pcpos)`。
/// cpp `IrTranslateBuiltins.cpp` 各翻译器里 `condOp/exitOp/4 操作数 inst` 三步内联展开的
/// 单点收口；操作数构造顺序（rhs 常量由调用方先建、其后 cond、再 exit）与原内联写法一致。
pub(crate) fn builtin_check_cmp(
  build: &mut IrBuilder,
  cmd: IrCmd,
  lhs: IrOp,
  rhs: IrOp,
  cond: IrCondition,
  pcpos: i32,
) {
  let cond_op = build.cond(cond);
  let exit_op = build.vm_exit(pcpos as u32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(cmd, lhs, rhs, cond_op, exit_op);
}

/// `CheckCmpInt` + `const_int` 字面量界的 32 位整数守卫
pub(crate) fn builtin_check_int_const(
  build: &mut IrBuilder,
  lhs: IrOp,
  rhs: i32,
  cond: IrCondition,
  pcpos: i32,
) {
  let bound = build.const_int(rhs);
  builtin_check_cmp(build, IrCmd::CheckCmpInt, lhs, bound, cond, pcpos);
}

/// `CheckCmpInt64` + `const_int_64` 字面量界的 int64 守卫
pub(crate) fn builtin_check_int_64_const(
  build: &mut IrBuilder,
  lhs: IrOp,
  rhs: i64,
  cond: IrCondition,
  pcpos: i32,
) {
  let bound = build.const_int_64(rhs);
  builtin_check_cmp(build, IrCmd::CheckCmpInt64, lhs, bound, cond, pcpos);
}

/// 除零守卫族（`/`、`%` 类整数运算）：`vb != 0` 才允许走快速路径
pub(crate) fn builtin_check_non_zero_int_64(build: &mut IrBuilder, rhs: IrOp, pcpos: i32) {
  builtin_check_int_64_const(build, rhs, 0, IrCondition::NotEqual, pcpos);
}
