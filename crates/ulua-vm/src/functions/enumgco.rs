use core::ffi::c_void;

use crate::{
  functions::enumobj::enumobj,
  records::{enum_context::EnumContext, gc_object::GCObject, lua_page::lua_Page},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn enumgco(
  context: *mut c_void,
  _page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  unsafe {
    let enum_ctx = context as *mut EnumContext;

    enumobj(enum_ctx, gco);

    false
  }
}
