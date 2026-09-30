use core::ptr::null;

use crate::{
  functions::enumnode::enumnode,
  macros::sizebuffer::sizebuffer,
  records::{enum_context::EnumContext, gc_object::GCObject, luau_buffer::LuauBuffer as Buffer},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn enumbuffer(ctx: *mut EnumContext, b: &Buffer) {
  unsafe {
    let gco = (b as *const Buffer).cast::<GCObject>();
    enumnode(ctx, gco, sizebuffer(b.len as usize), null());
  }
}
