//! x64 指令编码族：VEX 三字节前缀位段（avx_3_x 及 avx_b/r/w/x 位助手）、
//! ModRM/SIB、REX 位段与 REX_W_BIT 宏、opcode+cc/+reg 坍缩助手。
//! （r7-macros98 合并票：逐字保真自原一文件一宏碎片；本波把位段魔法数字收口为
//! 下方具名 `const`，算式与 `cpp/CodeGen/src/AssemblyBuilderX64.cpp:36-56` 宏逐位等价。）

use crate::{
  enums::size_x_64::SizeX64, functions::assembly::x_64::get_scale_encoding,
  records::register_x_64::RegisterX64,
};

/// ModRM/SIB reg/rm 字段的 3 位寄存器号掩码（cpp `:36/:55/:56` 的 `& 0x7`）。
const REG3_MASK: u8 = 0x7;

/// VEX byte3 VVVV 字段的 4 位寄存器号掩码（cpp `:53` 的 `~(v.index) & 0xf`）。
const VEX_V_MASK: u8 = 0xf;

/// 寄存器索引扩展位（bit3）：REX 与 VEX 的 R·X·B 三位都由它搬移而来
/// （cpp 各宏的 `& 0x8` / `~… & 0x8`）。
const REG_EXT_BIT: u8 = 0x8;

/// VEX byte3 的 W 位落点（cpp `AVX_W` `:46` 的 0x80 分支）。
const VEX_W_BIT: u8 = 0x80;

/// ModRM/SIB mod 字段位位置（bit7..6，cpp `<< 6`）。
const MOD_SHIFT: u32 = 6;

/// ModRM/SIB reg 字段与 VEX VVVV 字段的公共位位置（cpp `<< 3`）。
const REG_FIELD_SHIFT: u32 = 3;

/// VEX byte3 L 位位位置（bit2，cpp `:53` 的 `(l) << 2`）。
const VEX_L_SHIFT: u32 = 2;

/// REG_EXT_BIT(bit3) 上移至 VEX.R(byte2 bit7) 的距离（cpp `AVX_R` `:47` `<< 4`）。
const VEX_R_SHIFT: u32 = 4;

/// REG_EXT_BIT 上移至 VEX.X(byte2 bit6) 的距离（cpp `AVX_X` `:48` `<< 3`）。
const VEX_X_SHIFT: u32 = 3;

/// REG_EXT_BIT 上移至 VEX.B(byte2 bit5) 的距离（cpp `AVX_B` `:49` `<< 2`）。
const VEX_B_SHIFT: u32 = 2;

/// REG_EXT_BIT(bit3) 下移至 REX.R(prefix bit2) 的距离（cpp `REX_R` `:42` `>> 1`）。
const REX_R_SHIFT: u32 = 1;

/// REG_EXT_BIT 下移至 REX.X(prefix bit1) 的距离（cpp `REX_X` `:43` `>> 2`）。
const REX_X_SHIFT: u32 = 2;

/// REG_EXT_BIT 下移至 REX.B(prefix bit0) 的距离（cpp `REX_B` `:44` `>> 3`）。
const REX_B_SHIFT: u32 = 3;

/// REX 前缀字节固定高位（0100WRXB，cpp `REX_FORCE` `:41` 的 0x40 分支）。
const REX_BASE: u8 = 0x40;

/// SPL/BPL/SIL/DIL 起始索引：不小于该值的字节寄存器必须带 REX 才可寻址
/// （cpp `:41` 的 `index >= 4`）。
const REX_FORCED_MIN_INDEX: u8 = 4;

/// 位段落点编译期自检：具名移位与 cpp 宏展开的字节值逐位一致，常量间
/// 关系（REG3 掩码 = 扩展位低邻域、REX 三右移各落到 bit2/1/0）静态固化。
const _: () = assert!(
  REG3_MASK + 1 == REG_EXT_BIT
    && REG_EXT_BIT == 1 << REG_FIELD_SHIFT
    && VEX_W_BIT == 0x80
    && REX_BASE == 0x40
    && REG_EXT_BIT << VEX_R_SHIFT == 0x80
    && REG_EXT_BIT << VEX_X_SHIFT == 0x40
    && REG_EXT_BIT << VEX_B_SHIFT == 0x20
    && REG_EXT_BIT >> REX_R_SHIFT == 0x4
    && REG_EXT_BIT >> REX_X_SHIFT == 0x2
    && REG_EXT_BIT >> REX_B_SHIFT == 0x1
);

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
  avx_w(w) | ((!(v.index() & VEX_V_MASK) & VEX_V_MASK) << REG_FIELD_SHIFT) | (l << VEX_L_SHIFT) | p
}

pub const fn avx_b(reg: RegisterX64) -> u8 {
  (!reg.index() & REG_EXT_BIT) << VEX_B_SHIFT
}

pub const fn avx_r(reg: RegisterX64) -> u8 {
  (!reg.index() & REG_EXT_BIT) << VEX_R_SHIFT
}

#[inline(always)]
pub const fn avx_w(value: bool) -> u8 {
  if value { VEX_W_BIT } else { 0 }
}

pub const fn avx_x(reg: RegisterX64) -> u8 {
  (!reg.index() & REG_EXT_BIT) << VEX_X_SHIFT
}

#[inline(always)]
pub const fn mod_rm(mod_: u8, reg: u8, rm: u8) -> u8 {
  (mod_ << MOD_SHIFT) | ((reg & REG3_MASK) << REG_FIELD_SHIFT) | (rm & REG3_MASK)
}

#[inline(always)]
pub fn sib(scale: u8, index: u8, base: u8) -> u8 {
  (get_scale_encoding(scale) << MOD_SHIFT)
    | ((index & REG3_MASK) << REG_FIELD_SHIFT)
    | (base & REG3_MASK)
}

pub const fn rex_b(reg: RegisterX64) -> u8 {
  (reg.index() & REG_EXT_BIT) >> REX_B_SHIFT
}

pub const fn rex_r(reg: RegisterX64) -> u8 {
  (reg.index() & REG_EXT_BIT) >> REX_R_SHIFT
}

pub const fn rex_x(reg: RegisterX64) -> u8 {
  (reg.index() & REG_EXT_BIT) >> REX_X_SHIFT
}

pub const fn rex_force(reg: RegisterX64) -> u8 {
  // 注：RegisterX64::size() 非 const，但可以从 bits 计算。
  // 依据 RegisterX64 的内部结构（bits）提取 size。
  let size_bits = reg.bits & RegisterX64::SIZE_MASK;

  // SizeX64::Byte 是 enum 变体。impl 非 const 时 const if 里不能用 ==，
  // 且 size() 非 const，故拿原始 bits 与 SizeX64::Byte 的判别值比较。
  if size_bits == SizeX64::Byte as u8 && reg.index() >= REX_FORCED_MIN_INDEX {
    REX_BASE
  } else {
    0
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
  op.wrapping_add(reg & REG3_MASK)
}
