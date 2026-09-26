//! x64 内存操作数快捷形式：`mem(size, base, disp)` = `OperandX64::mem(size,
//! noreg, /*scale*/ 1, base, disp)` 的定参包装（cpp 各 X64 翻译单元里同款
//! `mem(...)` 简写的单一来源）。此前 9 个文件各抄一份私有 `fn mem`，收口于此。
use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

pub(crate) fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}
