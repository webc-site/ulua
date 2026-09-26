use core::ptr::null_mut;

use crate::{
  functions::cstr_bytes,
  macros::{curr_func::curr_func, getstr::getstr},
  records::{lua_state::LuaState, t_string::tstring},
};

/// 当前帧函数名（cpp/VM/src/laux.cpp:23 currfuncname）：可空 C 串返回值收口为
/// `Option<&'a [u8]>`——`None` 即原 NULL 哨兵（无当前帧 / 非 C 闭包 / debugname 为空 /
/// `__namecall` 但 namecall 未落名），`Some` 为不含终止 NUL 的原字节。
///
/// # Safety
/// `l` 须为存活 lua_State；`(*l).ci`、`(*l).base_ci` 指向同一 CallInfo 数组且 `base_ci <= ci`
/// （`curr_func!` 读当前帧闭包），C 闭包时 `inner.c.debugname` 为可读 C 串或 NULL，`(*l).namecall` 若
/// 非空须为存活 TString（`getstr` 取字节，Lua 串恒 NUL 结尾）。返回的字节区仅在对应
/// 对象（debugname 串或 namecall TString）存活期间有效。不抛错/不分配。
pub(crate) unsafe fn currfuncname<'a>(l: *mut LuaState) -> Option<&'a [u8]> {
  // Safety: 契约保证 l 存活、帧与闭包字段可读；cstr_bytes 各调用点均已先证指针非空
  // 且指向 NUL 结尾存活缓冲区（debugname C 串 / Lua 串 payload 恒带终止 NUL）
  unsafe {
    let cl = if (*l).ci > (*l).base_ci {
      curr_func!(l)
    } else {
      null_mut()
    };

    if cl.is_null() || (*cl).is_c == 0 || (*cl).inner.c.debugname.is_null() {
      return None;
    }

    let debugname = cstr_bytes((*cl).inner.c.debugname);
    if debugname != b"__namecall" {
      return Some(debugname);
    }

    let namecall = (*l).namecall;
    if namecall.is_null() {
      None
    } else {
      Some(cstr_bytes(getstr(namecall as *const tstring)))
    }
  }
}
