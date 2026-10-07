use crate::records::lua_state::LuaState;

/// 协程 resume 收尾：把状态布尔插到返回值之前。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票把首参收形
/// 为引用形后，重排栈全经 `push_boolean`/`insert` 安全门面，体内已无裸指针解引用，故本体降为安全
/// `fn`）：`r >= 0` 时栈顶须有 `r` 个协程 resume 返回值，供 `push_boolean(true)` + `insert(-(r+1))`
/// 把状态标志插到返回值之前（要求 `top-base >= r`）；`r < 0` 时 -1 须为已移入的错误值。仅重排栈、
/// 不分配。cpp `lcorolib.cpp:200`。
#[inline(always)]
pub fn coresumefinish(l: &mut LuaState, r: i32) -> i32 {
  if r < 0 {
    l.push_boolean(false);
    l.insert(-2);
    2
  } else {
    l.push_boolean(true);
    // r==0 时栈顶仅剩刚压入的 boolean 单元素段，insert(-1) 是恒等旋转
    // （index2addr + 段距计算全付、零内存写）——跳过。cpp lcorolib.cpp:200
    // 的 insert 在此同样无效果。
    if r != 0 {
      l.insert(-(r + 1));
    }
    r + 1
  }
}
