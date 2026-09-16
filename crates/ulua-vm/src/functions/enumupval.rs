use core::{ffi::c_char, mem::size_of, ptr::null};

use crate::{
  functions::{enumedge::enumedge, enumnode::enumnode},
  macros::{gcvalue::gcvalue, iscollectable::iscollectable, obj_2_gco::obj2gco},
  records::{enum_context::EnumContext, gc_object::GCObject},
  type_aliases::up_val::UpVal,
};

pub(crate) unsafe fn enumupval(ctx: *mut EnumContext, uv: *mut UpVal) {
  unsafe {
    enumnode(
      ctx,
      obj2gco!(uv as *mut GCObject),
      size_of::<UpVal>(),
      null(),
    );

    if iscollectable!((*uv).v) {
      enumedge(
        ctx,
        obj2gco!(uv as *mut GCObject),
        gcvalue!((*uv).v),
        b"value\0" as *const _ as *const c_char,
      );
    }
  }
}
