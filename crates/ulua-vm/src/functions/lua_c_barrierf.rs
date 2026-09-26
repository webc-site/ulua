//! Source: `VM/src/lgc.cpp` (lgc.cpp:1284-1294, hand-ported)

use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::reallymarkobject::reallymarkobject,
  macros::{
    gc_spause::GCSPAUSE, isblack::isblack, isdead::isdead, iswhite::iswhite,
    keepinvariant::keepinvariant, makewhite::makewhite,
  },
  records::{gc_object::GCObject, lua_state::LuaState},
};

/// # Safety
/// `l` 须存活且 GC 处于非 GCSpause 的增量阶段；`o` 为黑色存活对象、`v` 为刚被 `o` 引用的白色存活对象
/// （二者均未 dead）。违反（如 v 已是黑对象或对 dead 对象调用）会重复标记/改写已清扫对象，cpp lgc.cpp:1441。
pub unsafe fn lua_c_barrierf(l: *mut LuaState, o: *mut GCObject, v: *mut GCObject) {
  unsafe {
    let g = (*l).global;
    LUAU_ASSERT!(isblack!(o) && iswhite!(v) && !isdead!(g, v) && !isdead!(g, o));
    LUAU_ASSERT!((*g).gcstate as i32 != GCSPAUSE);
    // must keep invariant?
    if keepinvariant(&*g) {
      reallymarkobject(g, v); // restore invariant
    } else {
      // don't mind
      makewhite!(g, o); // mark as white just to avoid other barriers
    }
  }
}

/// # Safety
/// C ABI 导出壳：`o`/`v` 必须是写屏障协议中擦除为 `c_void` 的真实 `GCObject*`（v 刚被 o 引用），
/// 且 `l`/GC 阶段满足内层 `lua_c_barrierf` 契约；否则还原类型后按错误布局标记对象。cpp lgc.cpp:1441。
pub unsafe extern "C-unwind" fn lua_c_barrierf_export(
  l: *mut LuaState,
  o: *mut c_void,
  v: *mut c_void,
) {
  // Safety: 契约保证 `l` 存活且 `o`/`v` 相互一致（v 是新写入 o 的引用），前向屏障仅更新二者标签与 grayagain 链，不越出对象界
  unsafe {
    lua_c_barrierf(l, o as *mut GCObject, v as *mut GCObject);
  }
}
