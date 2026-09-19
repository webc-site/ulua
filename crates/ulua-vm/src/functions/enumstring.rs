use core::ptr::null;

use crate::{
  functions::enumnode::enumnode,
  macros::{obj_2_gco::obj2gco, sizestring::sizestring},
  records::{enum_context::EnumContext, t_string::tstring},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn enumstring(ctx: *mut EnumContext, ts: *mut tstring) {
  unsafe {
    enumnode(ctx, obj2gco!(ts), sizestring((*ts).len as usize), null());
  }
}
