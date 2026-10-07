/// GC 枚举边名（NUL 结尾字节串，经 [`enum_edge`] 收口取指针；§10 不引入 C 字符串类型）。
use core::{ffi::c_char, ptr::null};

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
const EDGE_ENV: &[u8] = b"env\0";
const EDGE_UPVALUE: &[u8] = b"upvalue\0";
const EDGE_PROTO: &[u8] = b"proto\0";

/// # Safety
/// `ctx` 须为存活 `EnumContext`（其节点/边缓冲有效）；`cl` 须为存活 `Closure`，按 `is_c` 分支
/// 读取 `inner.c.upvals[0..nupvalues]` 或 `inner.l.p` 及 `inner.l.uprefs[0..nupvalues]`、`env`；
/// Lclosure 分支要求 `inner.l.p` 非空，`p.debugname/p.source` 允许 NULL（内部已判空）。
/// 只读枚举并向 ctx 回调上报，不改对象。cpp `lgcdebug.cpp:847`。
pub(crate) unsafe fn enumclosure(ctx: *mut EnumContext, cl: &Closure) {
  unsafe {
    // 枚举身份取对象首字节地址：`obj2gco!` 要求裸指针入参，共享句柄降 `*const` 即够用
    let obj = (cl as *const Closure).cast::<GCObject>();

    if cl.is_c != 0 {
      // debugname 为 intern TString 锚（traverseclosure 标记边保证 GC 存活）；
      // 空名传 null，与 cpp `cl->c.debugname ? getstr(...) : nullptr` 逐位一致
      let name = if !cl.inner.c.debugname.is_null() {
        getstr(cl.inner.c.debugname)
      } else {
        null()
      };
      enumnode(ctx, obj, size_cclosure(cl.nupvalues as i32), name);
    } else {
      let p: *mut Proto = cl.inner.l.p;
      let mut buf = [0u8; LUA_IDSIZE as usize];

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
        size_lclosure(cl.nupvalues as usize),
        buf.as_ptr().cast::<c_char>(),
      );
    }

    enum_edge(ctx, obj, cl.env, EDGE_ENV);

    if cl.is_c != 0 {
      if cl.nupvalues > 0 {
        enumedges(
          ctx,
          obj,
          cl.inner.c.upvals.as_ptr(),
          cl.nupvalues as usize,
          EDGE_UPVALUE,
        );
      }
    } else {
      enum_edge(ctx, obj, cl.inner.l.p, EDGE_PROTO);

      if cl.nupvalues > 0 {
        enumedges(
          ctx,
          obj,
          cl.inner.l.uprefs.as_ptr(),
          cl.nupvalues as usize,
          EDGE_UPVALUE,
        );
      }
    }
  }
}
