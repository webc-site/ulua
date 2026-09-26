use core::{ffi::c_void, ptr::from_ref};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    dumpbuffer::dumpbuffer, dumpclass::dumpclass, dumpclosure::dumpclosure, dumpobject::dumpobject,
    dumpproto::dumpproto, dumpstring::dumpstring, dumptable::dumptable, dumpthread::dumpthread,
    dumpudata::dumpudata, dumpupval::dumpupval,
  },
  records::gc_object::{GCObject, GcView},
};

/// # Safety
/// `f` 须为可写且生命周期覆盖本次调用的输出目标；`o` 须为存活 GCObject 且类型已知。
/// 经安全视图 `as_view` 解构分派序列化。
/// cpp lgcdebug.cpp:653。
pub(crate) unsafe fn dumpobj(f: *mut c_void, o: *mut GCObject) {
  // Safety: 契约保证 `f` 为可写输出目标、`o` 为存活 GCObject，按类型分支序列化不越过各对象字段界
  unsafe {
    match (*o).as_view() {
      Some(GcView::String(ts)) => dumpstring(f, from_ref(ts).cast_mut().cast()),
      Some(GcView::Table(t)) => dumptable(f, from_ref(t).cast_mut().cast()),
      Some(GcView::Closure(cl)) => dumpclosure(f, from_ref(cl).cast_mut().cast()),
      Some(GcView::UserData(u)) => dumpudata(f, from_ref(u).cast_mut().cast()),
      Some(GcView::Thread(th)) => dumpthread(f, from_ref(th).cast_mut().cast()),
      Some(GcView::Buffer(buf)) => dumpbuffer(f, from_ref(buf).cast_mut().cast()),
      Some(GcView::Class(c)) => dumpclass(f, from_ref(c).cast_mut().cast()),
      Some(GcView::Object(obj)) => dumpobject(f, from_ref(obj).cast_mut().cast()),
      Some(GcView::Proto(p)) => dumpproto(f, from_ref(p).cast_mut().cast()),
      Some(GcView::UpVal(uv)) => dumpupval(f, from_ref(uv).cast_mut().cast()),
      None => LUAU_ASSERT!(false),
    }
  }
}
