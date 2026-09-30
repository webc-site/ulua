use crate::{
  enums::{size_x_64::SizeX64, x_64::CategoryX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

pub const fn add_reg_disp(reg: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, reg, disp)
}

pub fn add_reg_base_index(base: RegisterX64, index: RegisterX64) -> OperandX64 {
  CODEGEN_ASSERT!(index.index() != 4, "sp cannot be used as index");
  CODEGEN_ASSERT!(base.size() == index.size());

  OperandX64::mem(SizeX64::None, index, 1, base, 0)
}

pub fn add_operand_disp(mut op: OperandX64, disp: i32) -> OperandX64 {
  debug_assert!(op.cat == CategoryX64::Mem);
  debug_assert!(op.mem_size == SizeX64::None);

  op.imm += disp;
  op
}

pub fn add_operand_base(op: OperandX64, base: RegisterX64) -> OperandX64 {
  debug_assert!(op.cat == CategoryX64::Mem);
  debug_assert!(op.mem_size == SizeX64::None);
  debug_assert!(op.base == RegisterX64::NOREG);
  debug_assert!(op.index == RegisterX64::NOREG || op.index.size() == base.size());

  let mut op = op;
  op.base = base;
  op
}

/// `operator+(RegisterX64, OperandX64)`（OperandX64.h:132）：cpp 中与
/// `operator+(OperandX64, RegisterX64)`（:121）逐行同定义，实现收敛到 alt_d，
/// 此处仅保留参数序差异的公开入口。
pub fn add_reg_operand(base: RegisterX64, op: OperandX64) -> OperandX64 {
  add_operand_base(op, base)
}
