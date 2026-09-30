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
/// 经安全视图 `as_view` 解构分派，只读遍历并回调 ctx。下游 enum* 均为只读访问器，故直接透传
/// `GcView` 的共享借用，无须（也不得）升 `*mut`。
/// cpp VM/src/lgcdebug.cpp:1080
pub(crate) unsafe fn enumobj(ctx: *mut EnumContext, o: *const GCObject) {
  // SAFETY: 契约保证 `o` 的 tt 与 union 实际类型一致，各分支仅按该类型做只读遍历并回调 ctx
  unsafe {
    match (*o).as_view() {
      Some(GcView::String(ts)) => enumstring(ctx, ts),
      Some(GcView::Table(t)) => enumtable(ctx, t),
      Some(GcView::Closure(cl)) => enumclosure(ctx, cl),
      Some(GcView::UserData(u)) => enumudata(ctx, u),
      Some(GcView::Thread(th)) => enumthread(ctx, th),
      Some(GcView::Buffer(buf)) => enumbuffer(ctx, buf),
      Some(GcView::Class(c)) => enumclass(ctx, c),
      Some(GcView::Object(obj)) => enumobject(ctx, obj),
      Some(GcView::Proto(p)) => enumproto(ctx, p),
      Some(GcView::UpVal(uv)) => enumupval(ctx, uv),
      None => LUAU_ASSERT!(false),
    }
  }
}
