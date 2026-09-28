use core::ptr::from_ref;

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
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn validateobj(g: *mut global_State, o: *mut GCObject) {
  unsafe {
    if isdead!(g, o) {
      LUAU_ASSERT!((*g).gcstate == 4);
      return;
    }

    // tag 分派经安全视图 `as_view` 解构，消除逐臂裸指针强转
    match (*o).as_view() {
      // String/Buffer 无 GC 引用字段，与原空臂一致
      Some(GcView::String(_)) | Some(GcView::Buffer(_)) => {}
      Some(GcView::Table(t)) => validatetable(g, from_ref(t).cast_mut()),
      Some(GcView::Closure(cl)) => validateclosure(g, from_ref(cl).cast_mut()),
      Some(GcView::UserData(u)) => {
        if !u.metatable.is_null() {
          validateobjref(g, o, obj2gco!(u.metatable));
        }
      }
      Some(GcView::Thread(th)) => validatestack(g, from_ref(th).cast_mut()),
      Some(GcView::Proto(p)) => validateproto(g, from_ref(p).cast_mut()),
      Some(GcView::UpVal(uv)) => {
        validateref(g, o, &*uv.v);
      }
      Some(GcView::Class(c)) => validateclass(g, from_ref(c).cast_mut()),
      Some(GcView::Object(obj)) => validateobject(g, from_ref(obj).cast_mut()),
      None => LUAU_ASSERT!(false),
    }
  }
}
