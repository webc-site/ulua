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

/// `luaC_barriert` 表写屏障（cpp `lgc.cpp:1296`）。r16-v9：首参收 `&mut LuaState`
/// （仿 v5 rawequal/v7 pusherror 形制），`l` 存活/独占由接收者引用承载，`l.global` 读数
/// 为 safe 位；其余前提转为 safe fn 调用序契约（正确性，非内存安全；文档断言，由调用方
/// 承载）：GC 须非 GCSpause；`t` 是黑色存活表，`v` 是刚写入 `t` 的白色存活 GCObject
/// （未 dead）。二次传播阶段（GCSPROPAGATEAGAIN）退化为前向屏障。违反会把 gray 链挂错
/// 对象或标记已清扫内存。cpp lgc.cpp:1453。
pub fn lua_c_barriertable(l: &mut LuaState, t: *mut LuaTable, v: *mut GCObject) {
  // 接收者引用保证 `l` 有效，字段现读系 safe 位。
  let g = l.global;
  // SAFETY: 接收者保证 `l`（及由其派生的 `g`）非空与对齐；`t`/`v`/`o` 为 GC 协议裸形
  // （跨 crate 函数指针消费，不引用化），obj2gco/isblack/iswhite/isdead/black2gray 裸读、
  // `(*g).gcstate` 判读与 `(*t).gclist`、grayagain 链写在 GC 阶段与对象协议界内，
  // 其成立前提系上方调用序契约。
  unsafe {
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
  // SAFETY: 契约保证 `t` 存活且 `v` 为写入它的引用，`&mut *l` 引用重建仅收形（非空/对齐
  // 由本壳 unsafe fn 契约承载），表写屏障只触达 t 的 gch 标签与 grayagain 链
  unsafe {
    lua_c_barriertable(&mut *l, t as *mut LuaTable, v as *mut GCObject);
  }
}
