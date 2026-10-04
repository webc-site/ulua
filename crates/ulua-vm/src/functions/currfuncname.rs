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
/// r19-w4 收形并降 safe：首参 `*mut LuaState` → `&'a LuaState`（存活由类型承载，返回值
/// 字节随该状态借用存续），本函数只读帧/闭包/namecall 字段、不写穿也不分配、不抛错，
/// 参数收为引用后签名不再出现调用方裸指针，依判例（dev `SubtypingEnvironment::get_mapped_type_bounds`）
/// 由 `unsafe fn` 降为 `fn`；体内对帧字段 `*mut CallInfo`/`*mut Closure` 与 `cstr_bytes`/`getstr`
/// 的解引用皆源自 `l` 的自有字段而非调用方入参，包于单一 `unsafe` 块并附契约。
pub(crate) fn currfuncname<'a>(l: &'a LuaState) -> Option<&'a [u8]> {
  // SAFETY: `l` 存活，`(*l).ci`/`(*l).base_ci` 同处一 CallInfo 数组且 `base_ci <= ci`
  // （`curr_func!` 读当前帧闭包）；C 闭包时 `inner.c.debugname` 为可读 C 串或 NULL，
  // `(*l).namecall` 若非空须为存活 TString（`getstr` 取字节，Lua 串恒 NUL 结尾）。
  // cstr_bytes 各调用点均已先证指针非空且指向 NUL 结尾存活缓冲区。
  unsafe {
    // 既有约定（review.md §2）：VM c-API 边界签名折返——`cl` 空表示无当前闭包帧，null 为该边界合法返回，保留裸指针哨兵
    let cl = if l.ci > l.base_ci {
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

    let namecall = l.namecall;
    if namecall.is_null() {
      None
    } else {
      Some(cstr_bytes(getstr(namecall as *const tstring)))
    }
  }
}
