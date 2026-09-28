//! sizeudata 与 cpp oracle（`cpp/VM/src/ludata.h:17`）的对账：魔法数字 16/15 在
//! 此处以字面宏式保留为对账基准，证明 src 侧 UDATA_ALIGN/align_up 具名 const
//! 收口后输出一字不差。

use ulua_vm::macros::sizeudata::sizeudata;

/// sizeudata(0) 即 `offsetof(Udata, data)`，用作对齐基点而不外泄私有布局。
const BASE: usize = sizeudata(0);

/// cpp 宏原式（`(len > 16 ? (len + 15) & ~15 : len)`）。
const fn cpp_sizeudata(len: usize) -> usize {
  BASE + if len > 16 { (len + 15) & !15 } else { len }
}

#[test]
fn matches_cpp_macro_over_representative_lengths() {
  for len in 0..=80usize {
    assert_eq!(sizeudata(len), cpp_sizeudata(len), "len={len}");
  }
  for len in [127, 128, 129, 1000, 1024, 65535, 65536, usize::MAX / 2] {
    assert_eq!(sizeudata(len), cpp_sizeudata(len), "len={len}");
  }
}

#[test]
fn align_threshold_semantics() {
  // 阈值语义（cpp）：≤16 原样保留，>16 才向上取整到 16 的倍数
  assert_eq!(sizeudata(16) - BASE, 16);
  assert_eq!(sizeudata(17) - BASE, 32);
  assert_eq!(sizeudata(32) - BASE, 32);
  assert_eq!(sizeudata(33) - BASE, 48);
}
