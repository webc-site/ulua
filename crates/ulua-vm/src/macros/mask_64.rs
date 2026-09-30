//! cpp lintlib.cpp `#define mask64(w) (0xFFFFFFFFFFFFFFFFULL >> (64 - (w)))`。
//! 上游宏在 `w == 0` 时是 `>> 64`（C++ 未定义行为），`int64_extract` /
//! `int64_replace` 两处调用点各自用 `luaL_argcheck` 把 w 收口到 [1, 64]。
//! 这里提供收紧版：越界宽度不移位，直接给 0 / 全 1，调用点因此可以共用一份实现。
#[inline]
pub const fn mask64(w: i32) -> u64 {
  if w <= 0 {
    0
  } else if w >= 64 {
    0xFFFFFFFFFFFFFFFFu64
  } else {
    0xFFFFFFFFFFFFFFFFu64 >> (64 - w)
  }
}
