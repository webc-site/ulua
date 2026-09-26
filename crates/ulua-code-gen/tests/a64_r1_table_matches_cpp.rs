// A64 placeR1 单源浮点表与 cpp 逐格对账。
// 权威源：cpp/CodeGen/src/AssemblyBuilderA64.cpp（下称 AB）。
// 行 token 与生成体同源（见 crate::macros::a64_r1_impls），本测试锁定
// 「表 = cpp 位串」，即「生成函数发射 = cpp 发射」。

use ulua_code_gen::methods::assembly_builder_a_64_r1_table::A64_R1_ROWS;

// cpp 原文位串（按 Rust 4 bit 约定分组，位值与 cpp 源码逐位一致），行序同 AB 内出现顺序。
const R1_ORACLE: &[(&str, u32, u32, u32)] = &[
  // AB:860-864 fabs Q/D/S
  (
    "fabs",
    0b01_0011_1010_1000_0011_1110,
    0b00_0111_1001_1000_0011_0000,
    0b00_0111_1000_1000_0011_0000,
  ),
  // AB:1171-1175 frinta Q/D/S
  (
    "frinta",
    0b01_1011_1000_1000_0110_0010,
    0b00_0111_1001_1001_1001_0000,
    0b00_0111_1000_1001_1001_0000,
  ),
  // AB:1184-1188 frintm Q/D/S
  (
    "frintm",
    0b01_0011_1000_1000_0110_0110,
    0b00_0111_1001_1001_0101_0000,
    0b00_0111_1000_1001_0101_0000,
  ),
  // AB:1197-1201 frintp Q/D/S
  (
    "frintp",
    0b01_0011_1010_1000_0110_0010,
    0b00_0111_1001_1001_0011_0000,
    0b00_0111_1000_1001_0011_0000,
  ),
];

#[test]
fn r1_table_matches_cpp() {
  assert_eq!(A64_R1_ROWS.len(), R1_ORACLE.len());
  for (row, want) in A64_R1_ROWS.iter().zip(R1_ORACLE) {
    assert_eq!(row.0, want.0);
    assert_eq!(row.1, want.1, "{} Q 臂 op 漂移", row.0);
    assert_eq!(row.2, want.2, "{} D 臂 op 漂移", row.0);
    assert_eq!(row.3, want.3, "{} S 臂 op 漂移", row.0);
  }
}

#[test]
fn r1_mnemonic_names_unique() {
  let mut names: Vec<&str> = A64_R1_ROWS.iter().map(|r| r.0).collect();
  names.sort_unstable();
  let uniq = names.len();
  names.dedup();
  assert_eq!(names.len(), uniq);
  assert_eq!(uniq, 4);
}
