use core::ffi::c_char;

use crate::{
  functions::{c_slice, enumedge::enumedge},
  macros::{gcvalue::gcvalue, iscollectable::iscollectable},
  records::{enum_context::EnumContext, gc_object::GCObject},
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn enumedges(
  ctx: *mut EnumContext,
  from: *mut GCObject,
  data: *mut TValue,
  size: usize,
  edgename: *const c_char,
) {
  unsafe {
    // SAFETY：data 指向 size 个 TValue（枚举边遍历只读，来源对象已在枚举前入队）。
    for val in c_slice(data, size) {
      if iscollectable!(val) {
        enumedge(ctx, from, gcvalue!(val), edgename);
      }
    }
  }
}
