//! x64 指令编码族：VEX 三字节前缀位段（avx_3_x 及 avx_b/r/w/x 位助手）、
//! ModRM/SIB、REX 位段与 REX_W_BIT 宏、opcode+cc/+reg 坍缩助手。
//! （r7-macros98 合并票：逐字保真自原一文件一宏碎片，宏体/函数体未改动。）

use crate::{
  enums::size_x_64::SizeX64, functions::get_scale_encoding::get_scale_encoding,
  records::register_x_64::RegisterX64,
};

#[inline(always)]
pub const fn avx_3_1() -> u8 {
  0b11000100
}

#[inline(always)]
pub const fn avx_3_2(r: RegisterX64, x: RegisterX64, b: RegisterX64, m: u8) -> u8 {
  avx_r(r) | avx_x(x) | avx_b(b) | m
}

#[inline(always)]
pub const fn avx_3_3(w: bool, v: RegisterX64, l: u8, p: u8) -> u8 {
  avx_w(w) | ((!(v.index() & 0xf) & 0xf) << 3) | (l << 2) | p
}

pub const fn avx_b(reg: RegisterX64) -> u8 {
  (!reg.index() & 0x8) << 2
}

pub const fn avx_r(reg: RegisterX64) -> u8 {
  (!reg.index() & 0x8) << 4
}

#[inline(always)]
pub const fn avx_w(value: bool) -> u8 {
  if value { 0x80 } else { 0x0 }
}

pub const fn avx_x(reg: RegisterX64) -> u8 {
  (!reg.index() & 0x8) << 3
}

#[inline(always)]
pub const fn mod_rm(mod_: u8, reg: u8, rm: u8) -> u8 {
  (mod_ << 6) | ((reg & 0x7) << 3) | (rm & 0x7)
}

#[inline(always)]
pub fn sib(scale: u8, index: u8, base: u8) -> u8 {
  (get_scale_encoding(scale) << 6) | (((index) & 0x7) << 3) | ((base) & 0x7)
}

pub const fn rex_b(reg: RegisterX64) -> u8 {
  (reg.index() & 0x8) >> 3
}

pub const fn rex_r(reg: RegisterX64) -> u8 {
  (reg.index() & 0x8) >> 1
}

pub const fn rex_x(reg: RegisterX64) -> u8 {
  (reg.index() & 0x8) >> 2
}

pub const fn rex_force(reg: RegisterX64) -> u8 {
  // 注：RegisterX64::size() 非 const，但可以从 bits 计算。
  // 依据 RegisterX64 的内部结构（bits）提取 size。
  let size_bits = reg.bits & RegisterX64::SIZE_MASK;

  // SizeX64::Byte 是 enum 变体。impl 非 const 时 const if 里不能用 ==，
  // 且 size() 非 const，故拿原始 bits 与 SizeX64::Byte 的判别值比较。
  if size_bits == SizeX64::Byte as u8 && reg.index() >= 4 {
    0x40
  } else {
    0x00
  }
}

// Source: `CodeGen/src/AssemblyBuilderX64.cpp:39` (hand-ported)
// #define REX_W_BIT(value) (value ? 0x8 : 0x0)
#[macro_export]
macro_rules! REX_W_BIT {
  ($value:expr) => {
    if $value { 0x8u8 } else { 0x0u8 }
  };
}
pub use REX_W_BIT;

#[inline]
pub const fn op_plus_cc(op: u8, cc: u8) -> u8 {
  op.wrapping_add(cc)
}

#[inline]
pub const fn op_plus_reg(op: u8, reg: u8) -> u8 {
  op.wrapping_add(reg & 0x7)
}
