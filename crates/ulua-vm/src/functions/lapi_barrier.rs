//! LAPI 线程写屏障：全部 lapi 推栈函数共用的 `luaC_threadbarrier`。
//! 独立成文件以消除「公共工具寄居在 lua_concat.rs」的组织错位。

use crate::{
  functions::lua_c_barrierback::lua_c_barrierback,
  macros::{isblack::isblack, obj_2_gco::obj2gco},
  records::lua_state::LuaState,
};

/// cpp `lgc.h:115` 的 `luaC_threadbarrier(L)` 宏。
///
/// # Safety
/// `l` 须为存活的 `LuaState`（主线程或协程皆可）；本函数读 `(*l).gclist` 并
/// 在其为黑色时挂写屏障，调用方须保证该线程对象仍在 GC 存活集内、屏障字段可写。
pub(crate) unsafe fn lua_c_threadbarrier_lapi(l: *mut LuaState) {
  unsafe {
    let obj = obj2gco!(l);
    if isblack!(obj) {
      lua_c_barrierback(l, obj, &mut (*l).gclist);
    }
  }
}
