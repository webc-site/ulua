use crate::{
  functions::lua_d_reallocstack::lua_d_reallocstack,
  macros::{
    condhardstacktests::condhardstacktests, getgrownstacksize::getgrownstacksize,
    stacklimitreached::stacklimitreached,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活的 `lua_State`；`n` 为待预留的 `TValue` 槽数。本函数可能经
/// `lua_d_reallocstack` 重分配其栈，调用方缓存的 `top`/栈指针在返回后须重新取。
/// cpp `ldo.h:15` `luaD_checkstackfornewci` 同名宏对应。
#[inline]
pub unsafe fn lua_d_checkstackfornewci(l: *mut LuaState, n: i32) {
  // Safety: 契约保证 `l` 存活——stacklimitreached 读其 stack_last/top，扩容分支就地更新栈
  unsafe {
    if stacklimitreached(&*l, n) {
      lua_d_reallocstack(l, getgrownstacksize(l, n), 1);
    } else {
      condhardstacktests!(lua_d_reallocstack(l, (*l).stacksize - EXTRA_STACK, 1));
    }
  }
}
