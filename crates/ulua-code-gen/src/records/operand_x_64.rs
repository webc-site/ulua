use core::ops::{Add, Mul, Sub};

use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  functions::{
    operator_add_operand_x_64::operator_add_register_x_64_i32,
    operator_add_operand_x_64_alt_b::operator_add_register_x_64_register_x_64,
    operator_add_operand_x_64_alt_c::operator_add_operand_x_64_i32,
    operator_add_operand_x_64_alt_d::operator_add_operand_x_64_register_x_64,
    operator_add_operand_x_64_alt_e::operator_add_register_x_64_operand_x_64,
    operator_deref::operator_deref, operator_sub::operator_sub,
  },
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
  // Public in the C++ `OperandX64` struct; the constant-cache tests read `.imm`.
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
      // C++ uses `index(noreg), base(noreg)` — NOT register index 0. The
      // `{bits:0}` mistranslation made `qword[imm]` absolute addressing look
      // like it had base/index registers (wrong sib/ModRM).
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
}

// OperandX64.h:75-81 — namespace-level size-prefix memory-operand globals. Used
// as `qword[rax]` in the assembler/tests: the `[]` is `operator_bracket`, which
// stamps this operand's `memSize` onto the address expression. They are C++
// `inline constexpr OperandX64 name{size, noreg, 1, noreg, 0}` (the `mem` ctor).
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

/// C++ implicit `OperandX64(RegisterX64 reg)` constructor.
impl From<RegisterX64> for OperandX64 {
  fn from(reg: RegisterX64) -> Self {
    Self::reg(reg)
  }
}

/// C++ implicit `OperandX64(int32_t imm)` constructor.
impl From<i32> for OperandX64 {
  fn from(imm: i32) -> Self {
    Self::imm(imm)
  }
}

// Ergonomic addressing-mode operators, delegating to the translated free
// functions so `qword[rax + r12 * 2 + 0x1b]`-style operands read like the C++
// (OperandX64.h `operator*`/`operator+`/`operator-`). The `[]` itself stays
// `operator_bracket` (Rust `Index` must return a reference, so it can't model
// the by-value size-stamping the assembler relies on).
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
