use core::{mem::size_of, ptr::null};

use crate::{
  functions::{enumedge::enum_edge, enumnode::enumnode},
  macros::{gcvalue::gcvalue, iscollectable::iscollectable, obj_2_gco::obj2gco},
  records::{enum_context::EnumContext, gc_object::GCObject, up_val::UpVal},
};

/// # Safety
/// `ctx` 须为存活 `EnumContext`（节点/边缓冲有效）；`uv` 须为存活 `UpVal`，读其 `v`；当
/// `iscollectable(v)` 为真时 `gcvalue(v)` 须返回存活的被引用 GCObject 以登记边。cpp `lgcdebug.cpp:1018`。
pub(crate) unsafe fn enumupval(ctx: *mut EnumContext, uv: *mut UpVal) {
  // Safety: 契约保证 ctx/uv 存活，块内只做只读枚举与回调上报
  unsafe {
    let obj = obj2gco!(uv as *mut GCObject);

    enumnode(ctx, obj, size_of::<UpVal>(), null());

    if iscollectable!((*uv).v) {
      enum_edge(ctx, obj, gcvalue!((*uv).v), b"value\0");
    }
  }
}
