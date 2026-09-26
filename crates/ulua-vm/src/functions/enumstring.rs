use core::ptr::null;

use crate::{
  functions::enumnode::enumnode,
  macros::{obj_2_gco::obj2gco, sizestring::sizestring},
  records::{enum_context::EnumContext, t_string::tstring},
};

/// # Safety
/// `ctx` 须为存活 `EnumContext`（节点缓冲有效）；`ts` 须为存活 `tstring`，读其 `len`
/// 计算 `sizestring` 并把对象本身登记为枚举节点（`name` 传 NULL）。cpp `lgcdebug.cpp:779`。
pub(crate) unsafe fn enumstring(ctx: *mut EnumContext, ts: *mut tstring) {
  unsafe {
    enumnode(ctx, obj2gco!(ts), sizestring((*ts).len as usize), null());
  }
}
