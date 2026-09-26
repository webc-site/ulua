use core::ptr::from_ref;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    enumbuffer::enumbuffer, enumclass::enumclass, enumclosure::enumclosure, enumobject::enumobject,
    enumproto::enumproto, enumstring::enumstring, enumtable::enumtable, enumthread::enumthread,
    enumudata::enumudata, enumupval::enumupval,
  },
  records::{
    enum_context::EnumContext,
    gc_object::{GCObject, GcView},
  },
};

/// # Safety
/// `ctx` 须为存活 `EnumContext`（内含有效 `l`/目标 `FILE*`），`o` 须为存活 `GCObject`：
/// 经安全视图 `as_view` 解构分派，只读遍历并回调 ctx。
/// cpp VM/src/lgcdebug.cpp:1080
pub(crate) unsafe fn enumobj(ctx: *mut EnumContext, o: *mut GCObject) {
  // Safety: 契约保证 `o` 的 tt 与 union 实际类型一致，各分支仅按该类型做只读遍历并回调 ctx
  unsafe {
    match (*o).as_view() {
      Some(GcView::String(ts)) => enumstring(ctx, from_ref(ts).cast_mut().cast()),
      Some(GcView::Table(t)) => enumtable(ctx, from_ref(t).cast_mut()),
      Some(GcView::Closure(cl)) => enumclosure(ctx, from_ref(cl).cast_mut()),
      Some(GcView::UserData(u)) => enumudata(ctx, from_ref(u).cast_mut()),
      Some(GcView::Thread(th)) => enumthread(ctx, from_ref(th).cast_mut()),
      Some(GcView::Buffer(buf)) => enumbuffer(ctx, from_ref(buf).cast_mut().cast()),
      Some(GcView::Class(c)) => enumclass(ctx, from_ref(c).cast_mut()),
      Some(GcView::Object(obj)) => enumobject(ctx, from_ref(obj).cast_mut()),
      Some(GcView::Proto(p)) => enumproto(ctx, from_ref(p).cast_mut()),
      Some(GcView::UpVal(uv)) => enumupval(ctx, from_ref(uv).cast_mut()),
      None => LUAU_ASSERT!(false),
    }
  }
}
