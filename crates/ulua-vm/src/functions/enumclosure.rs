use core::ffi::{c_char, c_int};

use crate::{
  functions::{
    enumedge::enumedge,
    enumedges::enumedges,
    enumnode::enumnode,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{
    getstr::getstr, lua_idsize::LUA_IDSIZE, size_cclosure::size_cclosure,
    size_lclosure::size_lclosure,
  },
  records::{closure::Closure, enum_context::EnumContext, gc_object::GCObject, proto::Proto},
};

pub(crate) unsafe fn enumclosure(ctx: *mut EnumContext, cl: *mut Closure) {
  unsafe {
    let cl_ref = &*cl;
    let obj = cl as *mut GCObject;

    if cl_ref.is_c != 0 {
      enumnode(
        ctx,
        obj,
        size_cclosure(cl_ref.nupvalues as c_int),
        cl_ref.inner.c.debugname,
      );
    } else {
      let p: *mut Proto = cl_ref.inner.l.p;
      let mut buf = [0 as c_char; LUA_IDSIZE as usize];

      let name = if !(*p).debugname.is_null() {
        cstr_display(getstr((*p).debugname))
      } else {
        "unnamed"
      };

      if !(*p).source.is_null() {
        let src = cstr_display(getstr((*p).source));
        fmt_cstr_buf(&mut buf, format_args!("{name}:{} {src}", (*p).linedefined));
      } else {
        fmt_cstr_buf(&mut buf, format_args!("{name}:{}", (*p).linedefined));
      }

      enumnode(
        ctx,
        obj,
        size_lclosure(cl_ref.nupvalues as usize),
        buf.as_ptr(),
      );
    }

    enumedge(ctx, obj, cl_ref.env as *mut GCObject, c"env".as_ptr());

    if cl_ref.is_c != 0 {
      if cl_ref.nupvalues > 0 {
        enumedges(
          ctx,
          obj,
          cl_ref.inner.c.upvals.as_ptr() as *mut _,
          cl_ref.nupvalues as usize,
          c"upvalue".as_ptr(),
        );
      }
    } else {
      enumedge(
        ctx,
        obj,
        cl_ref.inner.l.p as *mut GCObject,
        c"proto".as_ptr(),
      );

      if cl_ref.nupvalues > 0 {
        enumedges(
          ctx,
          obj,
          cl_ref.inner.l.uprefs.as_ptr() as *mut _,
          cl_ref.nupvalues as usize,
          c"upvalue".as_ptr(),
        );
      }
    }
  }
}
