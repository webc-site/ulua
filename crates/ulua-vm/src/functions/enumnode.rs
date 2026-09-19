use core::ffi::c_char;

use crate::{
  functions::enumtopointer::enumtopointer,
  records::{enum_context::EnumContext, gc_object::GCObject},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn enumnode(
  ctx: *mut EnumContext,
  gco: *mut GCObject,
  size: usize,
  objname: *const c_char,
) {
  unsafe {
    let ctx_ref = &*ctx;
    if let Some(node_fn) = ctx_ref.node {
      node_fn(
        ctx_ref.context,
        enumtopointer(gco),
        (*gco).gch.tt,
        (*gco).gch.memcat,
        size,
        objname,
      );
    }
  }
}
