use core::ffi::c_char;

use crate::{
  functions::{
    enumedge::enum_edge,
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

/// GC 枚举边名（NUL 结尾字节串，由 [`enum_edge`] 取指针；§10 不用 `CStr`/`c"…"`）。
const EDGE_ENV: &[u8] = b"env\0";
const EDGE_UPVALUE: &[u8] = b"upvalue\0";
const EDGE_PROTO: &[u8] = b"proto\0";

/// # Safety
/// `ctx` 须为存活 `EnumContext`（其节点/边缓冲有效）；`cl` 须为存活 `Closure`，按 `is_c` 分支
/// 读取 `inner.c.upvals[0..nupvalues]` 或 `inner.l.p` 及 `inner.l.uprefs[0..nupvalues]`、`env`；
/// Lclosure 分支要求 `inner.l.p` 非空，`p.debugname/p.source` 允许 NULL（内部已判空）。
/// cpp `lgcdebug.cpp:847`。
pub(crate) unsafe fn enumclosure(ctx: *mut EnumContext, cl: *mut Closure) {
  unsafe {
    let cl_ref = &*cl;
    let obj = cl as *mut GCObject;

    if cl_ref.is_c != 0 {
      enumnode(
        ctx,
        obj,
        size_cclosure(cl_ref.nupvalues as i32),
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

    enum_edge(ctx, obj, cl_ref.env, EDGE_ENV);

    if cl_ref.is_c != 0 {
      if cl_ref.nupvalues > 0 {
        enumedges(
          ctx,
          obj,
          cl_ref.inner.c.upvals.as_ptr() as *mut _,
          cl_ref.nupvalues as usize,
          EDGE_UPVALUE,
        );
      }
    } else {
      enum_edge(ctx, obj, cl_ref.inner.l.p, EDGE_PROTO);

      if cl_ref.nupvalues > 0 {
        enumedges(
          ctx,
          obj,
          cl_ref.inner.l.uprefs.as_ptr() as *mut _,
          cl_ref.nupvalues as usize,
          EDGE_UPVALUE,
        );
      }
    }
  }
}
