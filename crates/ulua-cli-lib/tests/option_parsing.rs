//! CLI 选项表（`-O/-g/-t` 一类）的表驱动对账：本文件是 `atoi` 前缀语义与
//! `parse_level_arg` 区间/拒落语义的**唯一**测试点（原先散落在
//! `file_utils.rs`/`options_defaults.rs` 的同名/同义用例已并入此处 union 表），
//! 另加 `apply_level_arg` 的落位闭环。
//!
//! oracle 出处（`cpp/CLI/src/Compile.cpp`）：
//! - `-O<n>`：`atoi(argv[i]+2)` 后校验 `[0,2]`，越界报 Optimization（:507-516）；
//! - `-g<n>`：校验 `[0,2]`，越界报 Debug（:517-526）；
//! - `-t<n>`：校验 `[0,1]`，越界报 Type info（:527-536）；
//! - Bytecode.cpp:56-76：字节码工具的 `-O/-g` 同样是 `0..=2` 闭区间；
//! - `atoi` 为 C 前缀语义（非数字得 0、忽略尾字符、先消费前导空白与符号）；
//!   溢出在 cpp 为 UB，本移植钳到 i32 端点（见 atoi.rs 文档）。

use ulua_cli_lib::functions::{
  atoi::atoi,
  parse_level_arg::{apply_level_arg, parse_level_arg},
};

/// C `atoi` 前缀语义与溢出钳位：覆盖原 `file_utils.rs` 与本文件两处表的全行。
#[test]
fn atoi_matches_c_prefix_semantics() {
  let table = [
    ("0", 0),
    ("2", 2),
    ("12", 12),
    // 前导空白后接数字：atoi 跳过空白
    (" 7", 7),
    ("\t42", 42),
    ("  -7", -7),
    // 显式符号
    ("-1", -1),
    ("+3", 3),
    ("-0", 0),
    // 无数字前缀一律得 0
    ("", 0),
    ("   ", 0),
    ("abc", 0),
    // 数字前缀之后的非数字字符被忽略
    ("12abc", 12),
    ("12 34", 12),
    ("3.14", 3),
    // 溢出：cpp 为 UB，本移植钳到 i32 端点（见 atoi.rs 文档）
    ("99999999999", i32::MAX),
    ("-99999999999", i32::MIN),
  ];
  for (input, expected) in table {
    assert_eq!(atoi(input), expected, "atoi({input:?})");
  }
}

/// `-O<n>`/`-g<n>`/`-t<n>` 级别参数：闭区间放行 + atoi 前缀组合，union 表
/// 全行驱动（并入原 `options_defaults.rs::parse_level_arg_bounds_and_prefix`
/// 与本文件 `parse_level_arg_accepts_only_inclusive_range` 两张表）。
#[test]
fn parse_level_arg_range_table() {
  let cases = [
    // (输入串, low, high, 期望)
    ("0", 0, 2, Some(0)),   // 下边界（Bytecode.cpp:57 level<0 才拒）
    ("1", 0, 2, Some(1)),   // 区间中值
    ("2", 0, 2, Some(2)),   // 上边界
    ("3", 0, 2, None),      // 越上界 → "Optimization level must be between 0 and 2"
    ("99", 0, 2, None),     // 同样越上界
    ("-1", 0, 2, None),     // atoi 得 -1 → 越下界
    ("1x", 0, 2, Some(1)),  // C atoi 前缀语义：尾垃圾忽略
    ("abc", 0, 2, Some(0)), // atoi("abc")=0 → 界内取 0
    ("", 0, 2, Some(0)),    // 裸 `-O`：argv+2 为空串，atoi=0 落界内（Compile.cpp:507）
    ("0", 0, 1, Some(0)),   // Type info 下界
    ("1", 0, 1, Some(1)),   // Type info 上界（Compile.cpp:530）
    ("2", 0, 1, None),      // -t2 越 Type info 界 → 拒
    ("", 0, 1, Some(0)),    // 裸 `-t` → 0
  ];
  for (src, low, high, want) in cases {
    let got = parse_level_arg(src, low, high, "Optimization");
    assert_eq!(got, want, "input={src} range={low}..={high}");
  }
}

/// `apply_level_arg` 闭环：解析成功才写入 sink 并返 `true`；越界不写并返
/// `false`（调用方据此按 cpp 返回码 1）。sink 用局部 Cell 观察是否被调用。
#[test]
fn apply_level_arg_only_writes_on_success() {
  use core::cell::Cell;

  // 成功：落位为解析值，返回 true
  let sink = Cell::new(i32::MIN);
  let ok = apply_level_arg("2", 0, 2, "Debug", |v| sink.set(v));
  assert!(ok, "-g2 合法应返回 true");
  assert_eq!(sink.get(), 2, "成功时 sink 收到解析出的级别");

  // 越界：sink 保持未写，返回 false
  let sink = Cell::new(i32::MIN);
  let ok = apply_level_arg("3", 0, 2, "Debug", |v| sink.set(v));
  assert!(!ok, "-g3 越界应返回 false");
  assert_eq!(sink.get(), i32::MIN, "越界时闭包不得被调用");
}
