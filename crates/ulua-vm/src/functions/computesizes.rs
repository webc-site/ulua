use ulua_common::macros::luau_assert::LUAU_ASSERT;

/// cpp `computesizes`（ltable.cpp）：由区间计数 `nums` 推出数组部分的最优大小。
///
/// cpp 用 `int* narray` 出参回写新数组大小、返回 `na`，Rust 版折叠为
/// `(na, nasize)` 元组返回。
pub(crate) fn computesizes(nums: &[i32], narray: i32) -> (i32, i32) {
  let mut twotoi: i32 = 1;
  let mut a: i32 = 0;
  let mut na: i32 = 0;
  let mut n: i32 = 0;
  let mut i: usize = 0;

  // `i < nums.len()` 恒成立（nasize <= MAXSIZE = 2^MAXBITS），显式写出以替代
  // cpp 的裸指针越界读
  while twotoi / 2 < narray && i < nums.len() {
    let count = nums[i];
    if count > 0 {
      a += count;
      if a > twotoi / 2 {
        n = twotoi;
        na = a;
      }
    }
    if a == narray {
      break;
    }
    i += 1;
    twotoi *= 2;
  }

  LUAU_ASSERT!(n / 2 <= na && na <= n);
  (na, n)
}
