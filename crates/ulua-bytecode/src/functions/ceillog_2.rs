use ulua_common::LUAU_ASSERT;

use crate::functions::log_2::log2;

/// C++ `ceillog2`（BytecodeBuilder.cpp）。
pub(crate) fn ceillog2(v: i32) -> i32 {
  LUAU_ASSERT!(v > 0);

  if v == 1 { 0 } else { log2(v - 1) + 1 }
}
