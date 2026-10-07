use ulua_common::dfflag;

/// 测量期间堆可能收缩（完整 GC 后或标记期缩栈），此时记为 0 增长。
/// 对应 cpp/VM/src/lgc.cpp:1282 getheapgrowth。
#[inline]
pub(crate) fn getheapgrowth(current: usize, previous: usize) -> usize {
  if dfflag::LuauGcHeapShrinkFix.get() {
    current.saturating_sub(previous)
  } else {
    current.wrapping_sub(previous)
  }
}
