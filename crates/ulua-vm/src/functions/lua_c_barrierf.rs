//! Source: `VM/src/lgc.cpp` (lgc.cpp:1461-1471, hand-ported)

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

/// `luaC_barrierf` 前向写屏障（cpp `lgc.cpp:1461`）。r16-v9b：首参仍收 `&mut LuaState`
/// （仿 v5 rawequal/v7 pusherror 形制），`l` 非空/对齐与存活/独占由接收者引用承载，
/// `l.global` 读数为 safe 位；GC 阶段与对象协议前提归 Safety 节承载（unsafe fn 语义）。
///
/// # Safety
/// 本函数须在 GC 处于非 GCSpause 的增量阶段调用，`o` 为黑色存活对象、`v` 为刚被 `o`
/// 引用的白色存活对象（二者均未 dead）。违反（如 v 已是黑对象或对 dead 对象调用）会
/// 重复标记/改写已清扫对象，cpp lgc.cpp:1461。
pub unsafe fn lua_c_barrierf(l: &mut LuaState, o: *mut GCObject, v: *mut GCObject) {
  // 接收者引用保证 `l` 有效，字段现读系 safe 位。
  let g = l.global;
  // SAFETY: 接收者保证 `l`（及由其派生的 `g`）非空与对齐；`o`/`v` 为 GC 协议裸形对象
  // （跨 crate 函数指针消费，不引用化），isblack/iswhite/isdead/keepinvariant/makewhite
  // 与 `reallymarkobject` 的裸读裸写触点均在对象协议界内，其成立前提（GC 阶段与
  // 黑白/存活不变式）系上方 Safety 节。
  unsafe {
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
/// 且 `l`/GC 阶段满足内层 `lua_c_barrierf` 契约；否则还原类型后按错误布局标记对象。cpp lgc.cpp:1461。
pub unsafe extern "C-unwind" fn lua_c_barrierf_export(
  l: *mut LuaState,
  o: *mut c_void,
  v: *mut c_void,
) {
  // SAFETY: 契约保证 `l` 存活且 `o`/`v` 相互一致（v 是新写入 o 的引用），`&mut *l` 引用重建
  // 仅收形（非空/对齐由本壳调用侧契约承载），前向屏障仅更新二者标签与 grayagain 链，
  // 不越出对象界
  unsafe {
    lua_c_barrierf(&mut *l, o as *mut GCObject, v as *mut GCObject);
  }
}
