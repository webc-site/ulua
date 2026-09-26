use crate::{
  functions::vm_reg_op::vm_reg_op,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

/// 1..=5 参展开型 builtin（min/max、band/bor/bxor 等）的检查梯子：对 arg、args、
/// arg3 与第 4..=nparams 个步进寄存器（`vm_reg_op(args) + i - 2`）按序执行 `check`。
/// 调用方须先保证 `nparams >= 1`（各站守卫已含）。`vm_reg_op(args)` 只在步进臂内
/// 解引用，保持 cpp 的惰性求值——args 可为 VmConst 常量（FASTCALL2K），
/// 不得提前解引用。
pub(crate) fn check_unrolled(
  build: &mut IrBuilder,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nparams: i32,
  pcpos: i32,
  check: fn(&mut IrBuilder, IrOp, i32),
) {
  let arg_reg = build.vm_reg(arg as u8);
  check(build, arg_reg, pcpos);
  if nparams >= 2 {
    check(build, args, pcpos);
  }
  if nparams >= 3 {
    check(build, arg3, pcpos);
  }
  for i in 4..=nparams {
    let reg = build.vm_reg((vm_reg_op(args) + (i - 2)) as u8);
    check(build, reg, pcpos);
  }
}

/// 加载-折叠梯子：首参 `load(op0)` 为初值，其后每参 `load(opK)` 经
/// `combine(build, acc, 新值)` 逐个累积。`load` 可携带 per-参变换
/// （bit32 族的 NumToUint 包装）。操作数发射次序与各站原逐臂展开一致。
pub(crate) fn fold_unrolled(
  build: &mut IrBuilder,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nparams: i32,
  load: fn(&mut IrBuilder, IrOp) -> IrOp,
  mut combine: impl FnMut(&mut IrBuilder, IrOp, IrOp) -> IrOp,
) -> IrOp {
  let arg_reg = build.vm_reg(arg as u8);
  let mut acc = load(build, arg_reg);
  if nparams >= 2 {
    let v = load(build, args);
    acc = combine(build, acc, v);
  }
  if nparams >= 3 {
    let v = load(build, arg3);
    acc = combine(build, acc, v);
  }
  for i in 4..=nparams {
    let reg = build.vm_reg((vm_reg_op(args) + (i - 2)) as u8);
    let v = load(build, reg);
    acc = combine(build, acc, v);
  }
  acc
}
