use core::ffi::c_void;

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
  // SAFETY: 契约保证 `f` 为可写输出目标、`o` 为存活 GCObject，按类型分支序列化不越过各对象字段界
  unsafe {
    match (*o).as_view() {
      Some(GcView::String(ts)) => dumpstring(f, ts),
      Some(GcView::Table(t)) => dumptable(f, t),
      Some(GcView::Closure(cl)) => dumpclosure(f, cl),
      Some(GcView::UserData(u)) => dumpudata(f, u),
      Some(GcView::Thread(th)) => dumpthread(f, th),
      Some(GcView::Buffer(buf)) => dumpbuffer(f, buf),
      Some(GcView::Class(c)) => dumpclass(f, c),
      Some(GcView::Object(obj)) => dumpobject(f, obj),
      Some(GcView::Proto(p)) => dumpproto(f, p),
      Some(GcView::UpVal(uv)) => dumpupval(f, uv),
      None => LUAU_ASSERT!(false),
    }
  }
}
