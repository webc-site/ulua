use crate::records::lua_state::LuaState;

/// 判断 `LuaState` 距栈顶余量是否已达 `n` 个 `TValue`。仅读 `stack_last`/`top` 两个
/// 指针字段的地址做有符号差值，不解引用，故以 `&LuaState` 接收者替代原 `*mut` 裸指针。
#[inline(always)]
pub fn stacklimitreached(l: &LuaState, n: i32) -> bool {
  // SAFETY: stack_last 与 top 均分配自同一个连续 Lua 栈缓冲区，并在栈生命周期内有效存活。
  // C++: `(L)->stack_last - (L)->top <= (n)`（有符号指针减法）
  let diff = unsafe { l.stack_last.offset_from(l.top) };
  diff <= n as isize
}
