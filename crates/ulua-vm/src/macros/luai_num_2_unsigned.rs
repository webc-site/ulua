/// cpp `lnumutils.h:luai_num2unsigned(i, n)` 宏（可移植分支 `i = (unsigned)(long long)(n)`）。
///
/// 宏就地写出参的形态在 Rust 里即为返回值。
/// 注：cpp 另有 `_MSC_VER && _M_IX86` 的 x87 `fistp` 分支（对不可表示值为 UB）；
/// Rust 的 `as` 对 f64→i64 是饱和转换，与本仓库既有 VM 行为一致，保持不变。
#[inline(always)]
pub const fn luai_num2unsigned(n: f64) -> u32 {
  (n as i64) as u32
}
