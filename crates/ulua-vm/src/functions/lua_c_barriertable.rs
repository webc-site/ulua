//! Source: `VM/src/lgc.cpp` (lgc.cpp:1296-1314, hand-ported)

use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::reallymarkobject::reallymarkobject,
  macros::{
    black_2_gray::black2gray, gc_spause::GCSPAUSE, gc_spropagate::GCSPROPAGATEAGAIN,
    isblack::isblack, isdead::isdead, iswhite::iswhite, obj_2_gco::obj2gco,
  },
  records::{gc_object::GCObject, lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
/// `l` 须存活且 GC 非 GCSpause；`t` 是黑色存活表，`v` 是刚写入 `t` 的白色存活 GCObject（未 dead）。
/// 二次传播阶段（GCSPROPAGATEAGAIN）退化为前向屏障。违反会把 gray 链挂错对象或标记已清扫内存。
/// cpp lgc.cpp:1453。
pub unsafe fn lua_c_barriertable(l: *mut LuaState, t: *mut LuaTable, v: *mut GCObject) {
  unsafe {
    let g = (*l).global;
    let o = obj2gco!(t);

    // in the second propagation stage, table assignment barrier works as a forward barrier
    if (*g).gcstate as i32 == GCSPROPAGATEAGAIN {
      LUAU_ASSERT!(isblack!(o) && iswhite!(v) && !isdead!(g, v) && !isdead!(g, o));
      reallymarkobject(g, v);
      return;
    }

    LUAU_ASSERT!(isblack!(o) && !isdead!(g, o));
    LUAU_ASSERT!((*g).gcstate as i32 != GCSPAUSE);
    black2gray!(o); // make table gray (again)
    (*t).gclist = (*g).grayagain;
    (*g).grayagain = o;
  }
}

/// # Safety
/// C ABI 导出壳：`t`/`v` 必须是写屏障协议擦除为 `c_void` 的真实 `LuaTable*`（黑色）与被写入的白色
/// `GCObject*`，其余前提同内层 `lua_c_barriertable`；指针失真将按表布局改写 GC 头。cpp lgc.cpp:1453。
pub unsafe extern "C-unwind" fn lua_c_barriertable_export(
  l: *mut LuaState,
  t: *mut c_void,
  v: *mut c_void,
) {
  // Safety: 契约保证 `t` 存活且 `v` 为写入它的引用，表写屏障只触达 t 的 gch 标签与 grayagain 链
  unsafe {
    lua_c_barriertable(l, t as *mut LuaTable, v as *mut GCObject);
  }
}
