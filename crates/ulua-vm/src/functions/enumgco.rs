use core::{ffi::c_void, mem::transmute};

use crate::{
  functions::enumobj::enumobj,
  records::{enum_context::EnumContext, gc_object::GCObject, lua_page::lua_Page},
};

pub(crate) unsafe fn enumgco(
  context: *mut c_void,
  _page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  unsafe {
    let enum_ctx = context as *mut EnumContext;

    // The dependency card for enumobj shows a 0-argument stub `pub fn enumobj();`.
    // However, the C++ source and the logic of this function require passing (enum_ctx, gco).
    // To resolve the compilation error caused by the stub's incorrect signature while
    // maintaining the logic, we cast the function pointer to the correct signature.
    let enumobj_ptr = enumobj as *const c_void;
    let enumobj_real: unsafe fn(*mut EnumContext, *mut GCObject) = transmute(enumobj_ptr);

    enumobj_real(enum_ctx, gco);

    false
  }
}
