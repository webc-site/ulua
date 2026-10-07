//! x64_encoding 具名 const 收口与 cpp 宏的逐位对账：oracle 处重述
//! `cpp/CodeGen/src/AssemblyBuilderX64.cpp:36-56` 的字面位段算式（魔法数字保留
//! 在对账点本身），证明 src 侧 `REG3_MASK`/`REG_EXT_BIT`/各移位 const 替换后输出不变。

use ulua_code_gen::{
  enums::size_x_64::SizeX64,
  macros::x64_encoding::{
    avx_3_1, avx_3_2, avx_3_3, avx_b, avx_r, avx_w, avx_x, mod_rm, op_plus_cc, op_plus_reg, rex_b,
    rex_force, rex_r, rex_x, sib,
  },
  records::register_x_64::RegisterX64,
};

/// index 覆盖 bit3 两侧（0..7 与 8..15）且尺寸覆盖 byte/qword/xmm/ymm 的公开寄存器样本。
const REGS: &[RegisterX64] = &[
  RegisterX64::RAX,
  RegisterX64::RCX,
  RegisterX64::RDX,
  RegisterX64::RBX,
  RegisterX64::RSP,
  RegisterX64::RBP,
  RegisterX64::RSI,
  RegisterX64::RDI,
  RegisterX64::R8,
  RegisterX64::R9,
  RegisterX64::R10,
  RegisterX64::R11,
  RegisterX64::R12,
  RegisterX64::R13,
  RegisterX64::R14,
  RegisterX64::R15,
  RegisterX64::AL,
  RegisterX64::BL,
  RegisterX64::SIL,
  RegisterX64::DIL,
  RegisterX64::R9B,
  RegisterX64::XMM8,
  RegisterX64::YMM14,
];

/// cpp `MOD_RM`（`:55`）原式。
const fn cpp_mod_rm(mod_: u8, reg: u8, rm: u8) -> u8 {
  (mod_ << 6) | ((reg & 0x7) << 3) | (rm & 0x7)
}

/// cpp `SIB`（`:56`）原式，scale 已代入 `getScaleEncoding` 值表（1/2/4/8 → 0/1/2/3）。
const fn cpp_sib(scale_enc: u8, index: u8, base: u8) -> u8 {
  (scale_enc << 6) | ((index & 0x7) << 3) | (base & 0x7)
}

/// cpp `REX_R/X/B`（`:42-44`）共用形状。
const fn cpp_rex(index: u8, shift: u32) -> u8 {
  (index & 0x8) >> shift
}

/// cpp `AVX_R/X/B`（`:47-49`）共用形状。
const fn cpp_avx_inv(index: u8, shift: u32) -> u8 {
  (!index & 0x8) << shift
}

#[test]
fn mod_rm_and_sib_match_cpp() {
  for m in 0..=0xffu8 {
    for reg in 0..=0x1fu8 {
      for rm in 0..=0x1fu8 {
        assert_eq!(mod_rm(m, reg, rm), cpp_mod_rm(m, reg, rm));
      }
    }
  }
  for (scale, enc) in [(1u8, 0u8), (2, 1), (4, 2), (8, 3)] {
    for index in 0..=0x1fu8 {
      for base in 0..=0x1fu8 {
        assert_eq!(sib(scale, index, base), cpp_sib(enc, index, base));
      }
    }
  }
}

#[test]
fn rex_and_avx_ext_bits_match_cpp() {
  for &reg in REGS {
    let index = reg.index();
    assert_eq!(rex_r(reg), cpp_rex(index, 1), "REX.R {:?}", reg);
    assert_eq!(rex_x(reg), cpp_rex(index, 2), "REX.X {:?}", reg);
    assert_eq!(rex_b(reg), cpp_rex(index, 3), "REX.B {:?}", reg);
    assert_eq!(avx_r(reg), cpp_avx_inv(index, 4), "AVX.R {:?}", reg);
    assert_eq!(avx_x(reg), cpp_avx_inv(index, 3), "AVX.X {:?}", reg);
    assert_eq!(avx_b(reg), cpp_avx_inv(index, 2), "AVX.B {:?}", reg);
  }
}

#[test]
fn avx_prefix_bytes_match_cpp() {
  for m in [0u8, 1, 0b111] {
    for &r in REGS {
      for &x in REGS {
        for &b in REGS {
          let cpp =
            cpp_avx_inv(r.index(), 4) | cpp_avx_inv(x.index(), 3) | cpp_avx_inv(b.index(), 2) | m;
          assert_eq!(avx_3_2(r, x, b, m), cpp);
        }
      }
    }
  }
  for w in [false, true] {
    for &v in REGS {
      for l in [0u8, 1] {
        for p in 0..=0b11u8 {
          let cpp = (if w { 0x80 } else { 0 }) | (((!v.index()) & 0xf) << 3) | (l << 2) | p;
          assert_eq!(
            avx_3_3(w, v, l, p),
            cpp,
            "AVX_3_3 w={w} v={v:?} l={l} p={p}"
          );
        }
      }
    }
  }
  assert_eq!(avx_3_1(), 0b11000100);
  assert_eq!(avx_w(true), 0x80);
  assert_eq!(avx_w(false), 0x0);
}

#[test]
fn rex_force_matches_cpp() {
  for &reg in REGS {
    let cpp = if reg.size() == SizeX64::Byte && reg.index() >= 4 {
      0x40
    } else {
      0x00
    };
    assert_eq!(rex_force(reg), cpp, "REX_FORCE {:?}", reg);
  }
}

#[test]
fn op_plus_collapse_matches_cpp() {
  for op in 0..=0xffu8 {
    for reg in 0..=0x1fu8 {
      assert_eq!(op_plus_reg(op, reg), op.wrapping_add(reg & 0x7));
    }
    assert_eq!(op_plus_cc(op, 5), op.wrapping_add(5));
  }
}
