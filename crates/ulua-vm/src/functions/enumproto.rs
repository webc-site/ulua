/// NUL 结尾字节串（边名指针经 `cstr` 写入门面取；§10 不引入 C 字符串类型）。
use core::{ffi::c_char, mem::size_of, ptr::null};

use crate::{
  enums::lua_type::LUA_TNONE,
  functions::{
    c_slice, cstr,
    enumedge::enum_edge,
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
const EDGE_NATIVE: &[u8] = b"[native]\0";
const EDGE_CONSTANTS: &[u8] = b"constants\0";
const EDGE_PROTOS: &[u8] = b"protos\0";

/// # Safety
/// `ctx` 须指向存活 EnumContext（其 `l` 为存活 lua_State 且 `(*l).global` 有效，`node`/`edge` 回调可用）；
/// `p` 须指向存活 Proto：`k`/`p`/`code` 等数组指针与对应 size 字段（`sizek`/`sizep`/`sizecode`…）自洽，
/// `execdata` 非空时 `ecb.getmemorysize` 回调可用，`debugname`/`source` 为空或可读 TString。只读遍历并回调上报。
/// cpp/VM/src/lgcdebug.cpp:962 enumproto。
pub(crate) unsafe fn enumproto(ctx: *mut EnumContext, p: &Proto) {
  unsafe {
    let size = size_of::<Proto>()
      + size_of::<Instruction>() * p.sizecode as usize
      + size_of::<*mut Proto>() * p.sizep as usize
      + size_of::<TValue>() * p.sizek as usize
      + p.sizelineinfo as usize
      + size_of::<LocVar>() * p.sizelocvars as usize
      + size_of::<*mut tstring>() * p.sizeupvalues as usize;

    let ctx_ref = &*ctx;

    // Manual expansion of obj2gco for Proto because Proto is not a union member of GCObject
    // and does not have a .tt() method, but its hdr (GCheader) is at offset 0.
    let p_gco = (p as *const Proto).cast::<GCObject>();

    if !p.execdata.is_null() {
      let global = (*ctx_ref.l).global;
      if let Some(getmemorysize) = (*global).ecb.getmemorysize {
        // SAFETY(review §2): `getmemorysize` 为 C ABI 宿主回调，签名固定收 `*mut Proto`；
        // 其契约为「只量取原生码大小、不改 Proto」，故此处的可变指针只跨越该外部调用边界，
        // 本函数不据此解写。
        let nativesize = getmemorysize(ctx_ref.l, (p as *const Proto).cast_mut());

        if let Some(node_cb) = ctx_ref.node {
          node_cb(
            ctx_ref.context,
            p.execdata,
            LUA_TNONE as u8,
            p.hdr.memcat,
            nativesize,
            null(),
          );
        }

        if let Some(edge_cb) = ctx_ref.edge {
          edge_cb(
            ctx_ref.context,
            enumtopointer(&*p_gco),
            p.execdata,
            cstr(EDGE_NATIVE),
          );
        }
      }
    }

    let mut buf = [0u8; LUA_IDSIZE as usize];

    let name = if !p.debugname.is_null() {
      cstr_display(getstr(p.debugname))
    } else {
      "unnamed"
    };

    if !p.source.is_null() {
      let src = cstr_display(getstr(p.source));
      fmt_cstr_buf(
        &mut buf,
        format_args!("proto {name}:{} {src}", p.linedefined),
      );
    } else {
      fmt_cstr_buf(&mut buf, format_args!("proto {name}:{}", p.linedefined));
    }

    enumnode(ctx, p_gco, size, buf.as_ptr().cast::<c_char>());

    if p.sizek > 0 {
      enumedges(ctx, p_gco, p.k, p.sizek as usize, EDGE_CONSTANTS);
    }

    for &sub_proto in c_slice(p.p, p.sizep as usize) {
      enum_edge(ctx, p_gco, sub_proto, EDGE_PROTOS);
    }
  }
}
