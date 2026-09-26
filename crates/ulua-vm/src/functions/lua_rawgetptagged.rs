use core::ffi::c_void;

use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_h_getp::lua_h_getp,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, setobj_2_s::setobj_2_s, ttype::ttype,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 指向存活 `LuaState`；`idx` 为当前帧可解析的合法索引且该槽必须是表
/// （cpp lapi.cpp:891 `api_check!(ttype_is_table(gct(ptr)))`）；`p/tag` 仅作 lightuserdata 键的
/// 位模式比较、不被解引用，返回的 ttype 与压栈槽 `top.sub(1)` 处的值配套对应。cpp lapi.cpp:886。
pub unsafe fn lua_rawgetptagged(l: *mut LuaState, idx: i32, p: *mut c_void, tag: i32) -> i32 {
  // Safety: 契约保证 `l` 存活、索引处为可读表，块内 rawget 取回的 userdata 引用与 tag 匹配性检查仅在栈槽界内进行
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());

    setobj_2_s!(l, (*l).top, lua_h_getp((*t).as_table_ptr(), p, tag));
    api_incr_top!(l);

    ttype!((*l).top.sub(1)) as i32
  }
}
