use core::{ffi::c_char, ptr::null};

use crate::{
  functions::lua_getlocal::resolve_local,
  macros::{api_check::api_check, getstr::getstr, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 须为有效存活的 `LuaState`，`level`/`n` 须满足 C 参考实现的前置条件，
/// 且待写入值已压在栈顶。
pub unsafe fn lua_setlocal(l: *mut LuaState, level: i32, n: i32) -> *const c_char {
  // Safety: 契约保证 `l` 当前帧至少 1 个可写槽且 level 在调用栈深度内，变量名/valid 读取受 proto 局部信息界定
  unsafe {
    api_check!(l, (*l).top.offset_from((*l).base) >= 1);

    // var 为 null 时同样弹栈：cpp 原版无条件 pop，返回值仅为变量名
    let Some((ci, var)) = resolve_local(l, level, n) else {
      return null();
    };

    // 栈顶待写入值的单槽窗口收一次预绑定（resolve_local 纯读帧信息，不改 `(*l).top`）
    let value = (*l).top.offset(-1);
    if !var.is_null() {
      setobj_2_s!(l, (*ci).base.offset((*var).reg as isize), value);
    }

    (*l).top = value; // pop value

    if !var.is_null() {
      getstr((*var).varname)
    } else {
      null()
    }
  }
}
