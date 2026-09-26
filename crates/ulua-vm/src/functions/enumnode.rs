use core::ffi::c_char;

use crate::{
  functions::enumtopointer::enumtopointer,
  records::{enum_context::EnumContext, gc_object::GCObject},
};

/// # Safety
/// `ctx` 须指向存活 `EnumContext` 且其 `node` 回调与 `context` 配套（可空则跳过）；`gco` 须为存活
/// GCObject（要读 `gch.tt`/`gch.memcat` 头字段），`objname` 为有效 C 字符串。违反则悬垂回调调用/头字段越界读。
/// cpp lgcdebug.cpp:760。
pub(crate) unsafe fn enumnode(
  ctx: *mut EnumContext,
  gco: *mut GCObject,
  size: usize,
  objname: *const c_char,
) {
  // Safety: 契约保证 `ctx` 存活且 node 回调与其 context 配套，`gco` 头字段可读，块内仅追加输出不写对象
  unsafe {
    let ctx_ref = &*ctx;
    if let Some(node_fn) = ctx_ref.node {
      node_fn(
        ctx_ref.context,
        enumtopointer(gco),
        (*gco).tt(),
        (*gco).memcat(),
        size,
        objname,
      );
    }
  }
}
