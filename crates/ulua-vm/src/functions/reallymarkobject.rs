//! Source: `VM/src/lgc.cpp` (lgc.cpp:239-309, hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{
    gray_2_black::gray2black, isdead::isdead, iswhite::iswhite, markobject::markobject,
    markvalue::markvalue, upisopen::upisopen, white_2_gray::white2gray,
  },
  records::{
    gc_object::{GCObject, GcViewMut},
    global_state::global_State,
    lua_table::LuaTable,
    up_val::UpVal,
  },
};

/// # Safety
/// `g` 须为存活 `global_State`（读写其 `gray`/`grayagain` 链表头），`o` 须为存活 `GCObject` 且处于白色
/// （`iswhite!(o)` 成立、`!isdead!(g,o)`，由 LUAU_ASSERT 前置断言）；按类型解构时经安全视图 `as_view_mut` 分派，
/// 闭包/表/线程/proto/class/object 会被串入 `(*g).gray` 须保证 `gclist` 字段可写；markobject/markvalue 再入置灰，
/// 全程处于 GC 原子/标记阶段、不分配、不抛错。cpp VM/src/lgc.cpp:241
pub(crate) unsafe fn reallymarkobject(g: *mut global_State, o: *mut GCObject) {
  unsafe {
    LUAU_ASSERT!(iswhite!(o) && !isdead!(g, o));
    white2gray!(o);
    match (*o).as_view_mut() {
      Some(GcViewMut::String(_)) => {
        gray2black!(o); // strings are never gray（lgc.cpp:247，颜色不变式）
      }
      Some(GcViewMut::UserData(u)) => {
        let mt: *mut LuaTable = u.metatable;
        gray2black!(o); // udata are never gray
        if !mt.is_null() {
          markobject!(g, mt);
        }
      }
      Some(GcViewMut::UpVal(uv)) => {
        markvalue!(g, uv.v);
        if !upisopen!(uv as *mut UpVal) {
          // closed?
          gray2black!(o); // open upvalues are never black
        }
      }
      Some(GcViewMut::Closure(cl)) => {
        cl.gclist = (*g).gray;
        (*g).gray = o;
      }
      Some(GcViewMut::Table(h)) => {
        h.gclist = (*g).gray;
        (*g).gray = o;
      }
      Some(GcViewMut::Thread(th)) => {
        th.gclist = (*g).gray;
        (*g).gray = o;
      }
      Some(GcViewMut::Buffer(_)) => {
        gray2black!(o); // buffers are never gray
      }
      Some(GcViewMut::Proto(p)) => {
        p.gclist = (*g).gray;
        (*g).gray = o;
      }
      Some(GcViewMut::Class(classobject)) => {
        classobject.gclist = (*g).gray;
        (*g).gray = o;
      }
      Some(GcViewMut::Object(classinst)) => {
        classinst.gclist = (*g).gray;
        (*g).gray = o;
      }
      None => {
        LUAU_ASSERT!(false);
      }
    }
  }
}
