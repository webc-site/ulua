use core::ptr::null;

use crate::{
  functions::enumnode::enumnode,
  macros::sizestring::sizestring,
  records::{enum_context::EnumContext, gc_object::GCObject, t_string::tstring},
};

/// # Safety
/// `ctx` 须为存活 `EnumContext`（节点缓冲有效）；`ts` 须为存活 `tstring`，读其 `len`
/// 计算 `sizestring` 并把对象本身登记为枚举节点（`name` 传 NULL）。cpp `lgcdebug.cpp:779`。
pub(crate) unsafe fn enumstring(ctx: *mut EnumContext, ts: &tstring) {
  unsafe {
    // 枚举身份取对象首字节地址：共享句柄降 `*const` 即可
    enumnode(
      ctx,
      (ts as *const tstring).cast::<GCObject>(),
      sizestring(ts.len as usize),
      null(),
    );
  }
}
