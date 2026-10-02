use crate::records::lua_state::LuaState;

/// 是否可 yield（cpp `ldo.cpp:869` `lua_isyieldable`，r16-v3 前移为 `&LuaState` 引用形）：
/// 仅经引用读 `n_ccalls`/`base_ccalls` 两计数字段（除当前协程外无其它 C 帧在跑 → 可
/// yield），无裸指针解引用、不动栈/GC。调用序契约（正确性，非内存安全）：`l` 须为
/// 正被当前线程驱动的存活 `LuaState`——判读语义与该前置条件由本实现内部承接，
/// 调用方不再需要 unsafe。
pub fn lua_isyieldable(l: &LuaState) -> i32 {
  if l.n_ccalls <= l.base_ccalls {
    1
  } else {
    0
  }
}
