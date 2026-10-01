//! x64 AVX 助记符宏表与 cpp oracle 的逐格对账（abs-r127 `condition_const_tables.rs`
//! 路线的 x64 对偶）。
//!
//! 表体由 `crate::x64_avx_*_impls!` 宏生成并同步导出 `X64_AVX_*_ROWS` 常量
//! （src/methods/assembly_builder_x_64_avx_table.rs）；本测试把表行逐格对照
//! `cpp/CodeGen/src/AssemblyBuilderX64.cpp` 各 `placeAvx(...)` 调用点。
//!
//! 记号说明：cpp 侧 `AVX_0F/AVX_0F38/AVX_0F3A` 为窄 VEX mmmmm 字段（0b1/0b2/0b3），
//! `AVX_NP/AVX_66/AVX_F3/AVX_F2` 为窄 pp 字段（0b00/0b01/0b10/0b11）（cpp
//! AssemblyBuilderX64.h）；Rust 表允许裸 x86 前缀字节拼写（0x0F/0x38/0x3A 与
//! 0x66/0xF3/0xF2），两者经 `place_vex` 归一化（src/methods/
//! assembly_builder_x_64_place_vex.rs）等价，故下方 `norm_mode/norm_prefix` 先归一
//! 再比较，两种拼写均合法。
//!
//! 例外（不参与对账，表内以注释标记「疑似与 cpp 偏差」，本测试同时锁定它们
//! 未被悄悄改动）：`vcmpeqps / vcmpltss / vfmadd213pd / vfmadd213ps` 四行——
//! cpp（:1039/:1034/:1090/:1085）走的是 imm8 重载或 mode=AVX_0F38/prefix=AVX_66，
//! 与坍缩前 Rust 字面量本就不同；合表纪律是逐位保留原行为、不顺手修正，待专门
//! 的行为修复轮处理（见 abs-r131 收尾报告）。

use ulua_code_gen::methods::assembly_builder_x_64_avx_table::{
  X64_AVX_RM_REV_ROWS, X64_AVX_RM_ROWS, X64_AVX_RRM_ROWS,
};

/// 裸 x86 opcode-map 字节 → 窄 VEX mmmmm 字段；其余原样（与 place_vex 同式）。
const fn norm_mode(m: u8) -> u8 {
  match m {
    0x0F => 0b00001,
    0x38 => 0b00010,
    0x3A => 0b00011,
    other => other,
  }
}

/// 裸强制前缀字节 → 窄 VEX pp 字段；其余原样（与 place_vex 同式）。
const fn norm_prefix(p: u8) -> u8 {
  match p {
    0x66 => 0b01,
    0xF3 => 0b10,
    0xF2 => 0b11,
    other => other,
  }
}

/// cpp 常量记号（窄字段值）——写成函数避免每行重复注释翻译。
const AVX_NP: u8 = 0b00;
const AVX_66: u8 = 0b01;
const AVX_F3: u8 = 0b10;
const AVX_F2: u8 = 0b11;
const AVX_0F: u8 = 0b1;

/// 三操作数表：`(mnem, code, w, mode, prefix, cpp行号)`，期望值逐条取自
/// cpp `AssemblyBuilderX64.cpp` 对应 `placeAvx` 调用行（行号在元组末位）。
const RRM_ORACLE: &[(&str, u8, bool, u8, u8, u32)] = &[
  ("vaddpd", 0x58, false, AVX_0F, AVX_66, 745),
  ("vaddps", 0x58, false, AVX_0F, AVX_NP, 750),
  ("vaddsd", 0x58, false, AVX_0F, AVX_F2, 755),
  ("vaddss", 0x58, false, AVX_0F, AVX_F3, 760),
  ("vandnpd", 0x55, false, AVX_0F, AVX_66, 820),
  ("vandpd", 0x54, false, AVX_0F, AVX_66, 815),
  ("vandps", 0x54, false, AVX_0F, AVX_NP, 810),
  ("vdivps", 0x5E, false, AVX_0F, AVX_NP, 805),
  ("vdivsd", 0x5E, false, AVX_0F, AVX_F2, 795),
  ("vdivss", 0x5E, false, AVX_0F, AVX_F3, 800),
  ("vmaxps", 0x5F, false, AVX_0F, AVX_NP, 994),
  ("vmaxsd", 0x5F, false, AVX_0F, AVX_F2, 999),
  ("vmaxss", 0x5F, false, AVX_0F, AVX_F3, 1004),
  ("vminps", 0x5D, false, AVX_0F, AVX_NP, 1009),
  ("vminsd", 0x5D, false, AVX_0F, AVX_F2, 1014),
  ("vminss", 0x5D, false, AVX_0F, AVX_F3, 1019),
  ("vmulps", 0x59, false, AVX_0F, AVX_NP, 790),
  ("vmulsd", 0x59, false, AVX_0F, AVX_F2, 780),
  ("vmulss", 0x59, false, AVX_0F, AVX_F3, 785),
  // cpp:939 `vmovsd(dst, src1, src2)` 三参重载
  ("vmovsd", 0x10, false, AVX_0F, AVX_F2, 939),
  // cpp:949 `vmovss(dst, src1, src2)` 三参重载
  ("vmovss", 0x10, false, AVX_0F, AVX_F3, 949),
  ("vorpd", 0x56, false, AVX_0F, AVX_66, 840),
  ("vorps", 0x56, false, AVX_0F, AVX_NP, 835),
  ("vsubps", 0x5C, false, AVX_0F, AVX_NP, 775),
  ("vsubsd", 0x5C, false, AVX_0F, AVX_F2, 765),
  ("vsubss", 0x5C, false, AVX_0F, AVX_F3, 770),
  ("vsqrtsd", 0x51, false, AVX_0F, AVX_F2, 924),
  ("vsqrtss", 0x51, false, AVX_0F, AVX_F3, 929),
  ("vxorpd", 0x57, false, AVX_0F, AVX_66, 830),
  ("vxorps", 0x57, false, AVX_0F, AVX_NP, 825),
];

/// 双操作数无 coderev 表对账行。
const RM_ORACLE: &[(&str, u8, bool, u8, u8, u32)] = &[
  ("vsqrtpd", 0x51, false, AVX_0F, AVX_66, 914),
  ("vsqrtps", 0x51, false, AVX_0F, AVX_NP, 919),
  ("vucomisd", 0x2E, false, AVX_0F, AVX_66, 845),
  ("vucomiss", 0x2E, false, AVX_0F, AVX_NP, 850),
];

/// 双操作数带 coderev 表对账行：`(mnem, code, coderev, w, mode, prefix, cpp行)`。
const RM_REV_ORACLE: &[(&str, u8, u8, bool, u8, u8, u32)] = &[
  ("vmovapd", 0x28, 0x29, false, AVX_0F, AVX_66, 954),
  ("vmovaps", 0x28, 0x29, false, AVX_0F, AVX_NP, 959),
  ("vmovupd", 0x10, 0x11, false, AVX_0F, AVX_66, 964),
  ("vmovups", 0x10, 0x11, false, AVX_0F, AVX_NP, 969),
  // cpp:934/:944 `vmovsd/vmovss` 双参重载（Rust 无重载，后缀名区分）
  ("vmovsd", 0x10, 0x11, false, AVX_0F, AVX_F2, 934),
  ("vmovss", 0x10, 0x11, false, AVX_0F, AVX_F3, 944),
];

/// 疑似偏差行（不参与 cpp 对账），但必须仍在表内且字面量未被改动：
/// `(mnem, code, w, mode, prefix)` 逐字来自坍缩前的手写展开体。
const RRM_LOCKED_DRIFTS: &[(&str, u8, bool, u8, u8)] = &[
  ("vcmpeqps", 0x00, false, 0xc2, 0x00),
  ("vcmpltss", 0x01, false, 0xc2, 0x0f),
  ("vfmadd213pd", 0xA8, true, 0x0F, 0x38),
  ("vfmadd213ps", 0xA8, false, 0x0F, 0x38),
];

#[test]
fn rrm_table_matches_cpp() {
  // 参与对账的 30 行 + 4 行锁死偏差 = 34 行，一行不许多、不许少。
  assert_eq!(
    X64_AVX_RRM_ROWS.len(),
    RRM_ORACLE.len() + RRM_LOCKED_DRIFTS.len()
  );
  for (mnem, code, w, mode, prefix, line) in RRM_ORACLE {
    let row = X64_AVX_RRM_ROWS
      .iter()
      .find(|r| r.0 == *mnem)
      .unwrap_or_else(|| panic!("表内缺失 {mnem}（cpp AssemblyBuilderX64.cpp:{line}）"));
    assert_eq!(row.1, *code, "code({mnem}) ← cpp:{line}");
    assert_eq!(row.2, *w, "w({mnem}) ← cpp:{line}");
    assert_eq!(norm_mode(row.3), *mode, "mode({mnem}) ← cpp:{line}");
    assert_eq!(norm_prefix(row.4), *prefix, "prefix({mnem}) ← cpp:{line}");
  }
  for (mnem, code, w, mode, prefix) in RRM_LOCKED_DRIFTS {
    let row = X64_AVX_RRM_ROWS
      .iter()
      .find(|r| r.0 == *mnem)
      .unwrap_or_else(|| panic!("锁死偏差行 {mnem} 从表中消失"));
    assert_eq!(
      (row.1, row.2, row.3, row.4),
      (*code, *w, *mode, *prefix),
      "{mnem}"
    );
  }
}

#[test]
fn rm_table_matches_cpp() {
  assert_eq!(X64_AVX_RM_ROWS.len(), RM_ORACLE.len());
  for (mnem, code, w, mode, prefix, line) in RM_ORACLE {
    let row = X64_AVX_RM_ROWS
      .iter()
      .find(|r| r.0 == *mnem)
      .unwrap_or_else(|| panic!("表内缺失 {mnem}（cpp AssemblyBuilderX64.cpp:{line}）"));
    assert_eq!(row.1, *code, "code({mnem}) ← cpp:{line}");
    assert_eq!(row.2, *w, "w({mnem}) ← cpp:{line}");
    assert_eq!(norm_mode(row.3), *mode, "mode({mnem}) ← cpp:{line}");
    assert_eq!(norm_prefix(row.4), *prefix, "prefix({mnem}) ← cpp:{line}");
  }
}

#[test]
fn rm_rev_table_matches_cpp() {
  assert_eq!(X64_AVX_RM_REV_ROWS.len(), RM_REV_ORACLE.len());
  for (mnem, code, coderev, w, mode, prefix, line) in RM_REV_ORACLE {
    let row = X64_AVX_RM_REV_ROWS
      .iter()
      .find(|r| r.0 == *mnem)
      .unwrap_or_else(|| panic!("表内缺失 {mnem}（cpp AssemblyBuilderX64.cpp:{line}）"));
    assert_eq!(row.1, *code, "code({mnem}) ← cpp:{line}");
    assert_eq!(row.2, *coderev, "coderev({mnem}) ← cpp:{line}");
    assert_eq!(row.3, *w, "w({mnem}) ← cpp:{line}");
    assert_eq!(norm_mode(row.4), *mode, "mode({mnem}) ← cpp:{line}");
    assert_eq!(norm_prefix(row.5), *prefix, "prefix({mnem}) ← cpp:{line}");
  }
}

#[test]
fn mnemonic_names_unique_across_shapes() {
  // 同一 mnemonic 允许出现在不同重载形态（vmovsd/vmovss 各两行），但同一形态内不许重名。
  for names in [
    X64_AVX_RRM_ROWS.iter().map(|r| r.0).collect::<Vec<_>>(),
    X64_AVX_RM_ROWS.iter().map(|r| r.0).collect::<Vec<_>>(),
    X64_AVX_RM_REV_ROWS.iter().map(|r| r.0).collect::<Vec<_>>(),
  ] {
    for (i, a) in names.iter().enumerate() {
      assert!(
        names.iter().skip(i + 1).all(|b| a != b),
        "形态内重复 mnemonic {a}"
      );
    }
  }
}
