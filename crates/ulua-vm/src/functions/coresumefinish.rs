use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 `LuaState`；`r>=0` 时栈顶须有 `r` 个协程 resume 返回值，供
/// `lua_pushboolean`+`lua_insert(-(r+1))` 把状态标志插到返回值之前（要求 `top-base>=r`）；
/// `r<0` 时 -1 须为已移入的错误值。仅重排栈、不分配。cpp `lcorolib.cpp:200`。
pub unsafe fn coresumefinish(l: *mut LuaState, r: i32) -> i32 {
  unsafe {
    if r < 0 {
      (*l).push_boolean(false);
      (*l).insert(-2);
      2
    } else {
      (*l).push_boolean(true);
      // r==0 时栈顶仅剩刚压入的 boolean 单元素段，insert(-1) 是恒等旋转
      // （index2addr + 段距计算全付、零内存写）——跳过。cpp lcorolib.cpp:200
      // 的 insert 在此同样无效果。
      if r != 0 {
        (*l).insert(-(r + 1));
      }
      r + 1
    }
  }
}
