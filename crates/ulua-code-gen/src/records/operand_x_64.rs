use core::ops::{Add, Mul, Sub};

use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  functions::{
    operator_add_operand_x_64::{
      operator_add_operand_x_64_i32, operator_add_operand_x_64_register_x_64,
      operator_add_register_x_64_i32, operator_add_register_x_64_operand_x_64,
      operator_add_register_x_64_register_x_64,
    },
    operator_deref::operator_deref,
    operator_sub::operator_sub,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::register_x_64::RegisterX64,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct OperandX64 {
  pub(crate) cat: CategoryX64,
  pub(crate) index: RegisterX64,
  pub(crate) base: RegisterX64,
  pub(crate) mem_size: SizeX64,
  pub(crate) scale: u8,
  // 在 C++ 的 `OperandX64` struct 中是 public；常量缓存测试会读 `.imm`。
  pub imm: i32,
}

impl OperandX64 {
  pub const fn reg(reg: RegisterX64) -> Self {
    Self {
      cat: CategoryX64::Reg,
      index: RegisterX64::NOREG,
      base: reg,
      mem_size: SizeX64::None,
      scale: 1,
      imm: 0,
    }
  }

  pub const fn imm(imm: i32) -> Self {
    Self {
      cat: CategoryX64::Imm,
      // C++ 使用 `index(noreg), base(noreg)`——不是寄存器 index 0。误译
      // 成 `{bits:0}` 会让 `qword[imm]` 绝对寻址看起来带有 base/index
      // 寄存器（产生错误的 sib/ModRM）。
      index: RegisterX64::NOREG,
      base: RegisterX64::NOREG,
      mem_size: SizeX64::None,
      scale: 1,
      imm,
    }
  }

  pub const fn mem(
    size: SizeX64,
    index: RegisterX64,
    scale: u8,
    base: RegisterX64,
    disp: i32,
  ) -> Self {
    Self {
      cat: CategoryX64::Mem,
      index,
      base,
      mem_size: size,
      scale,
      imm: disp,
    }
  }

  pub const fn operator_bracket(&self, mut address: OperandX64) -> OperandX64 {
    // CODEGEN_ASSERT(cat == CategoryX64::mem);
    // CODEGEN_ASSERT(index == noreg && scale == 1 && base == noreg && imm == 0);
    // CODEGEN_ASSERT(address.memSize == SizeX64::None);

    address.cat = CategoryX64::Mem;
    address.mem_size = self.mem_size;
    address
  }

  pub fn operand_x_64_register_x_64(reg: RegisterX64) -> Self {
    Self {
      cat: CategoryX64::Reg,
      index: RegisterX64::NOREG,
      base: reg,
      mem_size: SizeX64::None,
      scale: 1,
      imm: 0,
    }
  }

  pub fn operand_x_64_i32(imm: i32) -> Self {
    OperandX64 {
      cat: CategoryX64::Imm,
      index: RegisterX64 { bits: 0xFF },
      base: RegisterX64 { bits: 0xFF },
      mem_size: SizeX64::None,
      scale: 1,
      imm,
    }
  }

  pub fn operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
    size: SizeX64,
    index: RegisterX64,
    scale: u8,
    base: RegisterX64,
    disp: i32,
  ) -> Self {
    OperandX64 {
      cat: CategoryX64::Mem,
      index,
      base,
      mem_size: size,
      scale,
      imm: disp,
    }
  }

  pub fn operand_x_64_operator_index(&self, mut addr: OperandX64) -> OperandX64 {
    CODEGEN_ASSERT!(self.cat == CategoryX64::Mem);
    CODEGEN_ASSERT!(
      self.index == RegisterX64::NOREG
        && self.scale == 1
        && self.base == RegisterX64::NOREG
        && self.imm == 0
    );
    CODEGEN_ASSERT!(addr.mem_size == SizeX64::None);

    addr.cat = CategoryX64::Mem;
    addr.mem_size = self.mem_size;
    addr
  }
}

// OperandX64.h:75-81——命名空间级的 size 前缀内存操作数全局量，
// 在汇编器/测试中以 `qword[rax]` 使用：`[]` 即 `operator_bracket`，
// 它把本操作数的 `memSize` 盖到地址表达式上。它们是 C++ 的
// `inline constexpr OperandX64 name{size, noreg, 1, noreg, 0}`（`mem` ctor）。
pub const ADDR: OperandX64 =
  OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, RegisterX64::NOREG, 0);

pub const BYTE: OperandX64 =
  OperandX64::mem(SizeX64::Byte, RegisterX64::NOREG, 1, RegisterX64::NOREG, 0);

pub const WORD: OperandX64 =
  OperandX64::mem(SizeX64::Word, RegisterX64::NOREG, 1, RegisterX64::NOREG, 0);

pub const DWORD: OperandX64 =
  OperandX64::mem(SizeX64::Dword, RegisterX64::NOREG, 1, RegisterX64::NOREG, 0);

pub const QWORD: OperandX64 =
  OperandX64::mem(SizeX64::Qword, RegisterX64::NOREG, 1, RegisterX64::NOREG, 0);

pub const XMMWORD: OperandX64 = OperandX64::mem(
  SizeX64::Xmmword,
  RegisterX64::NOREG,
  1,
  RegisterX64::NOREG,
  0,
);

pub const YMMWORD: OperandX64 = OperandX64::mem(
  SizeX64::Ymmword,
  RegisterX64::NOREG,
  1,
  RegisterX64::NOREG,
  0,
);

/// 对应 C++ 隐式构造函数 `OperandX64(RegisterX64 reg)`。
impl From<RegisterX64> for OperandX64 {
  fn from(reg: RegisterX64) -> Self {
    Self::reg(reg)
  }
}

/// 对应 C++ 隐式构造函数 `OperandX64(int32_t imm)`。
impl From<i32> for OperandX64 {
  fn from(imm: i32) -> Self {
    Self::imm(imm)
  }
}

// 便捷的寻址模式运算符，委托给已翻译的自由函数，使
// `qword[rax + r12 * 2 + 0x1b]` 风格的操作数与 C++（OperandX64.h
// `operator*`/`operator+`/`operator-`）写法一致。`[]` 本身保留为
// `operator_bracket`（Rust 的 `Index` 必须返回引用，无法建模
// 汇编器所依赖的按值盖 size 的行为）。
impl Mul<i32> for RegisterX64 {
  type Output = OperandX64;
  fn mul(self, scale: i32) -> OperandX64 {
    operator_deref(self, scale as u8)
  }
}

impl Add<i32> for RegisterX64 {
  type Output = OperandX64;
  fn add(self, disp: i32) -> OperandX64 {
    operator_add_register_x_64_i32(self, disp)
  }
}

impl Sub<i32> for RegisterX64 {
  type Output = OperandX64;
  fn sub(self, disp: i32) -> OperandX64 {
    operator_sub(self, disp)
  }
}

impl Add<RegisterX64> for RegisterX64 {
  type Output = OperandX64;
  fn add(self, index: RegisterX64) -> OperandX64 {
    operator_add_register_x_64_register_x_64(self, index)
  }
}

impl Add<OperandX64> for RegisterX64 {
  type Output = OperandX64;
  fn add(self, op: OperandX64) -> OperandX64 {
    operator_add_register_x_64_operand_x_64(self, op)
  }
}

impl Add<i32> for OperandX64 {
  type Output = OperandX64;
  fn add(self, disp: i32) -> OperandX64 {
    operator_add_operand_x_64_i32(self, disp)
  }
}

impl Add<RegisterX64> for OperandX64 {
  type Output = OperandX64;
  fn add(self, base: RegisterX64) -> OperandX64 {
    operator_add_operand_x_64_register_x_64(self, base)
  }
}
