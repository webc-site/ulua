use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

pub const fn byte_reg(reg: RegisterX64) -> RegisterX64 {
  RegisterX64 {
    bits: SizeX64::Byte as u8 | (reg.bits & RegisterX64::INDEX_MASK),
  }
}

pub const fn dword_reg(reg: RegisterX64) -> RegisterX64 {
  RegisterX64 {
    bits: (SizeX64::Dword as u8) | (reg.bits & RegisterX64::INDEX_MASK),
  }
}

pub const fn qword_reg(reg: RegisterX64) -> RegisterX64 {
  RegisterX64 {
    bits: (SizeX64::Qword as u8) | (reg.bits & RegisterX64::INDEX_MASK),
  }
}

// x64 内存操作数快捷形式：`mem(size, base, disp)` = `OperandX64::mem(size,
// noreg, /*scale*/ 1, base, disp)` 的定参包装（cpp 各 X64 翻译单元里同款
// `mem(...)` 简写的单一来源）。此前 9 个文件各抄一份私有 `fn mem`，收口于此。

pub(crate) fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}

pub fn operator_deref(reg: RegisterX64, scale: u8) -> OperandX64 {
  if scale == 1 {
    return OperandX64::reg(reg);
  }

  CODEGEN_ASSERT!(matches!(scale, 1 | 2 | 4 | 8));
  CODEGEN_ASSERT!(reg.index() != 0b100);

  OperandX64::mem(SizeX64::None, reg, scale, RegisterX64::NOREG, 0)
}

pub fn operator_sub(reg: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, reg, -disp)
}

// See also: 本 crate `macros/codegen_assert.rs` 的 `CODEGEN_ASSERT!`——形似义异：
// 该宏走 ulua_common `assert_fail` C-ABI 上报通道，此处是同名局部替身（标准
// `assert!` panic 语义），刻意不统一，勿合并。
macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

pub fn get_scale_encoding(scale: u8) -> u8 {
  const SCALES: [u8; 9] = [0xff, 0, 1, 0xff, 2, 0xff, 0xff, 0xff, 3];

  CODEGEN_ASSERT!(scale < 9 && SCALES[scale as usize] != 0xff);
  SCALES[scale as usize]
}

pub fn same_underlying_register(a: RegisterX64, b: RegisterX64) -> bool {
  let underlying_size_a = if a.size() == SizeX64::Xmmword {
    SizeX64::Xmmword
  } else {
    SizeX64::Qword
  };

  let underlying_size_b = if b.size() == SizeX64::Xmmword {
    SizeX64::Xmmword
  } else {
    SizeX64::Qword
  };

  underlying_size_a == underlying_size_b && a.index() == b.index()
}
