use core::ffi::c_void;

use crate::{
  functions::{
    index_2_addr::index_2_addr, lua_g_readonlyerror::check_writable, lua_h_setp::lua_h_setp,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_c_barriert::luaC_barriert,
    setobj_2_t::setobj2t,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).top` 之下留有 1 个值元素（`api_checknelems 1`）作为待写入值；
/// `idx` 经 `index_2_addr` 解析出的槽须为 table（`api_check is_table`）且非只读（否则 `lua_g_readonlyerror` 抛错）；
/// `p` 为用作 light-C 指针键的裸地址（不被 GC 追踪），`tag` 为其标签整数；`setobj2t`/屏障可触发 GC，须受保护帧。
/// cpp `lapi.cpp:1055`。
pub unsafe fn lua_rawsetptagged(l: *mut LuaState, idx: i32, p: *mut c_void, tag: i32) {
  unsafe {
    api_checknelems!(l, 1);
    let o: StkId = index_2_addr(l, idx);
    api_check!(l, (*o).is_table());
    check_writable(l, (*o).as_table_ptr());
    let val = (*l).top.offset(-1);
    setobj2t!(l, lua_h_setp(l, (*o).as_table_ptr(), p, tag), val);
    luaC_barriert!(l, (*o).as_table_ptr(), val);
    (*l).top = (*l).top.offset(-1);
  }
}
