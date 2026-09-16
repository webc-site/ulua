use core::{
  ffi::c_char,
  mem::size_of,
  ptr::{addr_of, null, null_mut},
};

use crate::{
  functions::{
    enumedge::enumedge,
    enumedges::enumedges,
    enumnode::enumnode,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{clvalue::clvalue, getstr::getstr, obj_2_gco::obj2gco, ttisfunction::ttisfunction},
  records::{
    call_info::CallInfo, closure::LClosure, enum_context::EnumContext, gc_object::GCObject,
    lua_state::lua_State, proto::Proto,
  },
  type_aliases::{closure::Closure, t_value::TValue},
};

pub(crate) unsafe fn enumthread(ctx: *mut EnumContext, th: *mut lua_State) {
  unsafe {
    let size = size_of::<lua_State>()
      + size_of::<TValue>() * (*th).stacksize as usize
      + size_of::<CallInfo>() * (*th).size_ci as usize;

    let mut tcl: *mut Closure = null_mut();
    let mut ci: *mut CallInfo = (*th).base_ci;
    while ci <= (*th).ci {
      if ttisfunction!((*ci).func) {
        tcl = clvalue!((*ci).func);
        break;
      }
      ci = ci.wrapping_add(1);
    }

    if !tcl.is_null() && (*tcl).is_c == 0 {
      let tcl_l = addr_of!((*tcl).inner.l).cast::<LClosure>();
      let p: *mut Proto = (*tcl_l).p;
      if !p.is_null() {
        let mut buf: [c_char; 256] = [0; 256];

        let src = if !(*p).source.is_null() {
          cstr_display(getstr((*p).source))
        } else {
          "unnamed"
        };
        let name = if !(*p).debugname.is_null() {
          cstr_display(getstr((*p).debugname))
        } else {
          "unnamed"
        };

        fmt_cstr_buf(
          &mut buf,
          format_args!("thread at {name}:{} {src}", (*p).linedefined),
        );

        enumnode(ctx, obj2gco!(th as *mut GCObject), size, buf.as_ptr());
      } else {
        enumnode(ctx, obj2gco!(th as *mut GCObject), size, null());
      }
    } else {
      enumnode(ctx, obj2gco!(th as *mut GCObject), size, null());
    }

    enumedge(
      ctx,
      obj2gco!(th as *mut GCObject),
      obj2gco!((*th).gt as *mut GCObject),
      c"globals".as_ptr(),
    );

    if (*th).top > (*th).stack {
      enumedges(
        ctx,
        obj2gco!(th as *mut GCObject),
        (*th).stack,
        (*th).top.offset_from((*th).stack) as usize,
        c"stack".as_ptr(),
      );
    }
  }
}
