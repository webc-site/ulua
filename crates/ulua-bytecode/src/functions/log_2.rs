use ulua_common::LUAU_ASSERT;

/// C++ `log2`（BytecodeBuilder.cpp）：v>0 时返回 floor(log2(v))，v<=0 返回 0。
/// 用前导零位计数把 O(log v) 循环降为单条 CPU 指令。
///
/// DELIBERATE DEVIATION：cpp 的 `2 << r` 在 v ≥ 2^30 时有符号移位溢出（UB，
/// 实测退化为死循环）；本实现全程 defined，该区间给出数学正确的 30。实际
/// 调用方（span/常量长度）远达不到该域。
pub(crate) fn log2(v: i32) -> i32 {
  LUAU_ASSERT!(v != 0);

  if v <= 0 {
    return 0;
  }

  (i32::BITS - 1 - (v as u32).leading_zeros()) as i32
}

/// C++ `ceillog2`（BytecodeBuilder.cpp）。
pub(crate) fn ceillog2(v: i32) -> i32 {
  LUAU_ASSERT!(v > 0);

  if v == 1 { 0 } else { log2(v - 1) + 1 }
}
