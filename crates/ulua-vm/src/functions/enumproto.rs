use core::{ffi::c_char, mem::size_of, ptr::null};

use crate::{
  enums::lua_type::LUA_TNONE,
  functions::{
    c_slice,
    enumedge::{edge_name, enum_edge},
    enumedges::enumedges,
    enumnode::enumnode,
    enumtopointer::enumtopointer,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{getstr::getstr, lua_idsize::LUA_IDSIZE},
  records::{
    enum_context::EnumContext, gc_object::GCObject, loc_var::LocVar, proto::Proto,
    t_string::tstring,
  },
  type_aliases::{instruction::Instruction, t_value::TValue},
};

/// NUL 结尾字节串（边名指针由 [`edge_name`] 取；§10 不引入 `CStr`/`c"…"`）。
const EDGE_NATIVE: &[u8] = b"[native]\0";
const EDGE_CONSTANTS: &[u8] = b"constants\0";
const EDGE_PROTOS: &[u8] = b"protos\0";

/// # Safety
/// `ctx` 须指向存活 EnumContext（其 `l` 为存活 lua_State 且 `(*l).global` 有效，`node`/`edge` 回调可用）；
/// `p` 须指向存活 Proto：`k`/`p`/`code` 等数组指针与对应 size 字段（`sizek`/`sizep`/`sizecode`…）自洽，
/// `execdata` 非空时 `ecb.getmemorysize` 回调可用，`debugname`/`source` 为空或可读 TString。只读遍历并回调上报。
/// cpp/VM/src/lgcdebug.cpp:962 enumproto。
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
            edge_name(EDGE_NATIVE),
          );
        }
      }
    }

    let mut buf = [0 as c_char; LUA_IDSIZE as usize];

    let name = if !p_ref.debugname.is_null() {
      cstr_display(getstr(p_ref.debugname))
    } else {
      "unnamed"
    };

    if !p_ref.source.is_null() {
      let src = cstr_display(getstr(p_ref.source));
      fmt_cstr_buf(
        &mut buf,
        format_args!("proto {name}:{} {src}", p_ref.linedefined),
      );
    } else {
      fmt_cstr_buf(&mut buf, format_args!("proto {name}:{}", p_ref.linedefined));
    }

    enumnode(ctx, p_gco, size, buf.as_ptr());

    if p_ref.sizek > 0 {
      enumedges(ctx, p_gco, p_ref.k, p_ref.sizek as usize, EDGE_CONSTANTS);
    }

    for &sub_proto in c_slice(p_ref.p, p_ref.sizep as usize) {
      enum_edge(ctx, p_gco, sub_proto, EDGE_PROTOS);
    }
  }
}
