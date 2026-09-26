/// 上游 `lnumutils.h:18` 的 `luai_inteq(a, b) ((a) == (b))`：对 int64 做精确相等。
/// 必须先于任何浮点转换比较，否则 |值| > 2^53 的两个不同整数会被 f64 塌缩成误判相等。
#[inline(always)]
pub const fn luai_inteq(a: i64, b: i64) -> bool {
  a == b
}
