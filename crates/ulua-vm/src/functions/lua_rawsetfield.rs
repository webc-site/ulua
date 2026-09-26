use core::ffi::c_char;

use crate::{
  functions::{
    cstr_bytes, index_2_addr::index_2_addr, lua_g_readonlyerror::check_writable,
    lua_h_setstr::lua_h_setstr,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_c_barriert::luaC_barriert,
    lua_s_new::lua_s_new, setobj_2_t::setobj2t,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`api_checknelems!(l,1)` 要求 `(*l).top` 前已压 value 一槽（读 `top-1`）；
/// `idx` 为合法索引且 `index_2_addr` 所得槽为 table（is_table），其 `(*hvalue).readonly` 须为 0 否则
/// `lua_g_readonlyerror` 抛错回退；`k` 须为指向 NUL 结尾 C 串的存活指针（`lua_s_new` 读取至首个 NUL，
/// 可触发字符串 intern/GC）。`lua_h_setstr` 可能 rehash：取得槽后到 `setobj2t` 写值之间不得再有对同表
/// 的结构性写（本函数内成立）。可触发 GC。cpp VM/src/lapi.cpp:1020。
pub unsafe fn lua_rawsetfield(l: *mut LuaState, idx: i32, k: *const c_char) {
  unsafe {
    api_checknelems!(l, 1);
    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());
    check_writable(l, (*t).as_table_ptr());
    // ⇔ cpp lapi.cpp:1026-1028 逐位同序：:1026 `setobj2t(L, luaH_setstr(...), top-1)`
    // （intern 串→取槽→写值三步嵌套在同一语句内，Rust 保持同样的表达式嵌套与求值序：
    // lua_s_new 可触发 GC，但发生在槽取得之前，无失效窗口）、:1027 `luaC_barriert`
    // （屏障在值落槽之后）、:1028 `top--`。屏障与写分步保留，不收敛单函数。
    // 栈顶 value 槽的三连裸重读收为一次预绑定（lua_s_new/intern/GC 均不改写 `(*l).top`）
    let value = (*l).top.offset(-1);
    setobj2t!(
      l,
      // k 为 NUL 结尾 C 串，经 cstr_bytes 扫首个 NUL 得字节切片（保持原 lua_s_new 的 strlen 语义）
      lua_h_setstr(l, (*t).as_table_ptr(), lua_s_new(l, cstr_bytes(k))),
      value
    );
    luaC_barriert!(l, (*t).as_table_ptr(), value);
    (*l).top = value;
  }
}
