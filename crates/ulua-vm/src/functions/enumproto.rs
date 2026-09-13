use core::{
  ffi::{c_char, c_int},
  mem::size_of,
  ptr::null,
};

use crate::{
  enums::lua_type::LUA_TNONE,
  functions::{
    c_slice, enumedge::enumedge, enumedges::enumedges, enumnode::enumnode,
    enumtopointer::enumtopointer,
  },
  macros::{getstr::getstr, lua_idsize::LUA_IDSIZE},
  records::{enum_context::EnumContext, gc_object::GCObject, proto::Proto},
  type_aliases::{instruction::Instruction, loc_var::LocVar, t_string::tstring, t_value::TValue},
};

pub(crate) unsafe fn enumproto(ctx: *mut EnumContext, p: *mut Proto) {
  unsafe {
    let p_ref = &*p;

    let size = size_of::<Proto>()
      + size_of::<Instruction>() * p_ref.sizecode as usize
      + size_of::<*mut Proto>() * p_ref.sizep as usize
      + size_of::<TValue>() * p_ref.sizek as usize
      + p_ref.sizelineinfo as usize
      + size_of::<LocVar>() * p_ref.sizelocvars as usize
      + size_of::<*mut tstring>() * p_ref.sizeupvalues as usize;

    let ctx_ref = &*ctx;

    // Manual expansion of obj2gco for Proto because Proto is not a union member of GCObject
    // and does not have a .tt() method, but its hdr (GCheader) is at offset 0.
    let p_gco = p as *mut GCObject;

    if !p_ref.execdata.is_null() {
      let global = (*ctx_ref.l).global;
      if let Some(getmemorysize) = (*global).ecb.getmemorysize {
        let nativesize = getmemorysize(ctx_ref.l, p);

        if let Some(node_cb) = ctx_ref.node {
          node_cb(
            ctx_ref.context,
            p_ref.execdata,
            LUA_TNONE as u8,
            p_ref.hdr.memcat,
            nativesize,
            null(),
          );
        }

        if let Some(edge_cb) = ctx_ref.edge {
          edge_cb(
            ctx_ref.context,
            enumtopointer(p_gco),
            p_ref.execdata,
            c"[native]".as_ptr(),
          );
        }
      }
    }

    let mut buf = [0 as c_char; LUA_IDSIZE as usize];

    let debugname = if !p_ref.debugname.is_null() {
      getstr(p_ref.debugname)
    } else {
      c"unnamed".as_ptr()
    };

    unsafe extern "C" {
      fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    }

    if !p_ref.source.is_null() {
      snprintf(
        buf.as_mut_ptr(),
        buf.len(),
        c"proto %s:%d %s".as_ptr(),
        debugname,
        p_ref.linedefined,
        getstr(p_ref.source),
      );
    } else {
      snprintf(
        buf.as_mut_ptr(),
        buf.len(),
        c"proto %s:%d".as_ptr(),
        debugname,
        p_ref.linedefined,
      );
    }

    enumnode(ctx, p_gco, size, buf.as_ptr());

    if p_ref.sizek > 0 {
      enumedges(
        ctx,
        p_gco,
        p_ref.k,
        p_ref.sizek as usize,
        c"constants".as_ptr(),
      );
    }

    for &sub_proto in c_slice(p_ref.p, p_ref.sizep as usize) {
      enumedge(ctx, p_gco, sub_proto as *mut GCObject, c"protos".as_ptr());
    }
  }
}
