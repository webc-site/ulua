use core::{ffi::c_void, ptr::null};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::tms::TMS,
  functions::lua_h_getstr::lua_h_getstr,
  records::{lua_table::LuaTable, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 调用方须保证：`events` 为存活事件表（lua_h_getstr 读其 array/node 界内）、`ename` 为存活 TString、
/// `event` ≤ TmEq（未命中时在 events.tmcache 写该事件缓存位，位宽受限）；违反即越界读写 UB。
/// cpp ltm.cpp:99 `luaT_gettm`
pub(crate) unsafe fn lua_t_gettm(
  events: *mut LuaTable,
  event: TMS,
  ename: *mut tstring,
) -> *const TValue {
  // Safety: 契约保证 `events` 为空（走 dummynode）或指向覆盖至 event 索引的存活事件数组，`ename` 为存活串
  unsafe {
    let tm = lua_h_getstr(events, ename);

    LUAU_ASSERT!((event as u32) <= (TMS::TmEq as u32));

    if (*tm).is_nil() {
      // no tag method? cache this fact
      (*events).tmcache |= (1u32 << (event as u32)) as u8;
      null()
    } else {
      tm
    }
  }
}

/// # Safety
/// 与 [`lua_t_gettm`] 同契约：events 为存活事件表、ename 存活 TString、event ≤ TmEq；C 边界把
/// `c_void` 指针 cast 回 LuaTable/tstring，指针有效性由外部调用方负责。cpp ltm.cpp:99
pub unsafe extern "C-unwind" fn lua_t_gettm_export(
  events: *mut c_void,
  event: TMS,
  ename: *mut c_void,
) -> *const TValue {
  // Safety: 直接转发同契约 `lua_t_gettm`，events/event/ename 透传满足其前置
  unsafe { lua_t_gettm(events as *mut LuaTable, event, ename as *mut tstring) }
}
