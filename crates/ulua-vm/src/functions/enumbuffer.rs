use core::{ffi::c_void, ptr::null};

use crate::{
  functions::enumnode::enumnode,
  macros::sizebuffer::sizebuffer,
  records::{enum_context::EnumContext, gc_object::GCObject},
  type_aliases::buffer::Buffer,
};

pub(crate) unsafe fn enumbuffer(ctx: *mut EnumContext, b: *mut Buffer) {
  unsafe {
    // Buffer is a collectable object; cast it to a GCObject for enumnode.
    // We must avoid using `obj2gco!` here because it expects a proper GCObject
    // layout (via `.tt()`), and a raw cast of the Buffer pointer into `c_void`
    // breaks that assumption during macro expansion.
    let gco = b as *mut c_void;

    enumnode(
      ctx,
      gco as *mut GCObject,
      sizebuffer((*b).len as usize),
      null(),
    );
  }
}
