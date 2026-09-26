use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_op::IrOp, register_a_64::RegisterA64, register_x_64::RegisterX64},
  type_aliases::ir_ops::IrOps,
};

#[derive(Debug, Clone)]
#[repr(C)]
// C++ 中为 `IrInst` 的 public；暴露它是为了让跨 crate 测试 harness（以及已
// 引用它的 pub `apply_substitutions`/`IrFunction::instructions`）
// 能给出类型名。
pub struct IrInst {
  pub cmd: IrCmd,
  pub ops: IrOps,
  pub last_use: u32,
  pub use_count: u16,
  pub reg_x64: RegisterX64,
  pub reg_a64: RegisterA64,
  pub reused_reg: bool,
  pub spilled: bool,
  pub needs_reload: bool,
}

impl IrInst {
  /// 由命令与操作数列表构造 `IrInst`，对应 C++ 聚合初始化
  /// `IrInst{cmd, {ops...}}`。供测试 fixture 的 `checkEq`
  /// 构造期望指令以做比较。
  pub fn ir_inst_new(cmd: IrCmd, ops: &[IrOp]) -> Self {
    Self {
      cmd,
      ops: ops.iter().cloned().collect(),
      ..Self::default()
    }
  }

  /// 读取第 `idx` 个操作数；若越界返回缺省操作数（`IrOp::default()`）。
  #[inline]
  pub fn op(&self, idx: u32) -> IrOp {
    if idx < self.ops.size() {
      self.ops[idx as usize]
    } else {
      IrOp::default()
    }
  }
}

impl Default for IrInst {
  fn default() -> Self {
    Self {
      cmd: IrCmd::NOP,
      ops: IrOps::new(),
      last_use: 0,
      use_count: 0,
      reg_x64: RegisterX64::NOREG,
      reg_a64: RegisterA64::NOREG,
      reused_reg: false,
      spilled: false,
      needs_reload: false,
    }
  }
}
