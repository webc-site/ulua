use core::ffi::c_char;

use crate::{
  functions::enumtopointer::enumtopointer,
  records::{enum_context::EnumContext, gc_object::GCObject},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn enumedge(
  ctx: *mut EnumContext,
  from: *mut GCObject,
  to: *mut GCObject,
  edgename: *const c_char,
) {
  unsafe {
    let ctx_ref = &*ctx;
    if let Some(edge_fn) = ctx_ref.edge {
      edge_fn(
        ctx_ref.context,
        enumtopointer(from),
        enumtopointer(to),
        edgename,
      );
    }
  }
}
