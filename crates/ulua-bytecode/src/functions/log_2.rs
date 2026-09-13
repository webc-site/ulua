use ulua_common::LUAU_ASSERT;

/// C++ `log2`（BytecodeBuilder.cpp）：v>0 时返回 floor(log2(v))，v<=0 返回 0。
/// 用前导零位计数把 O(log v) 循环降为单条 CPU 指令。
pub(crate) fn log2(v: i32) -> i32 {
  LUAU_ASSERT!(v != 0);

  if v <= 0 {
    return 0;
  }

  (i32::BITS - 1 - (v as u32).leading_zeros()) as i32
}
