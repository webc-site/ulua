use core::ptr::null;

use crate::{
  functions::enumnode::enumnode,
  macros::{obj_2_gco::obj2gco, sizestring::sizestring},
  records::enum_context::EnumContext,
  type_aliases::t_string::tstring,
};

pub(crate) unsafe fn enumstring(ctx: *mut EnumContext, ts: *mut tstring) {
  unsafe {
    enumnode(ctx, obj2gco!(ts), sizestring((*ts).len as usize), null());
  }
}
