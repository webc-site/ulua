use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    validateclass::validateclass, validateclosure::validateclosure, validateobject::validateobject,
    validateobjref::validateobjref, validateproto::validateproto, validateref::validateref,
    validatestack::validatestack, validatetable::validatetable,
  },
  macros::{isdead::isdead, obj_2_gco::obj2gco},
  records::{
    gc_object::{GCObject, GcView},
    global_state::global_State,
  },
};

/// # Safety
/// `g` 须指向存活 global_State；`o` 须为存活 GCObject 且类型已知。tag 分派经安全视图 `as_view`
/// 解构，下游 validate* 均为只读访问器，故引用源句柄（`*const`）即够用，无须升 `*mut`。
pub(crate) unsafe fn validateobj(g: *mut global_State, o: *const GCObject) {
  unsafe {
    if isdead!(g, o) {
      LUAU_ASSERT!((*g).gcstate == 4);
      return;
    }

    // tag 分派经安全视图 `as_view` 解构，消除逐臂裸指针强转
    match (*o).as_view() {
      // String/Buffer 无 GC 引用字段，与原空臂一致
      Some(GcView::String(_)) | Some(GcView::Buffer(_)) => {}
      Some(GcView::Table(t)) => validatetable(g, t),
      Some(GcView::Closure(cl)) => validateclosure(g, cl),
      Some(GcView::UserData(u)) => {
        if !u.metatable.is_null() {
          validateobjref(g, o, obj2gco!(u.metatable));
        }
      }
      Some(GcView::Thread(th)) => validatestack(g, th),
      Some(GcView::Proto(p)) => validateproto(g, p),
      Some(GcView::UpVal(uv)) => {
        validateref(g, o, &*uv.v);
      }
      Some(GcView::Class(c)) => validateclass(g, c),
      Some(GcView::Object(obj)) => validateobject(g, obj),
      None => LUAU_ASSERT!(false),
    }
  }
}
