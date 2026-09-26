use core::{
  ffi::c_char,
  mem::size_of,
  ptr::{addr_of, null},
};

use crate::{
  functions::{
    enumedge::enum_edge,
    enumedges::enumedges,
    enumnode::enumnode,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{getstr::getstr, obj_2_gco::obj2gco},
  records::{
    call_info::CallInfo,
    closure::{Closure, LClosure},
    enum_context::EnumContext,
    gc_object::GCObject,
    lua_state::LuaState,
    proto::Proto,
  },
  type_aliases::t_value::TValue,
};

/// NUL 结尾字节串（边名指针由 `enumedge::edge_name` 取；§10 不引入 `CStr`/`c"…"`）。
const EDGE_GLOBALS: &[u8] = b"globals\0";
const EDGE_STACK: &[u8] = b"stack\0";

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn enumthread(ctx: *mut EnumContext, th: *mut LuaState) {
  unsafe {
    let size = size_of::<LuaState>()
      + size_of::<TValue>() * (*th).stacksize as usize
      + size_of::<CallInfo>() * (*th).size_ci as usize;

    // 首个 Lua 函数帧的闭包：`Option` 承载「未找到」，不用 null 哨兵（§2）。
    let mut tcl: Option<*mut Closure> = None;
    let mut ci: *mut CallInfo = (*th).base_ci;
    while ci <= (*th).ci {
      if (*(*ci).func).is_function() {
        tcl = Some((*(*ci).func).as_closure_ptr());
        break;
      }
      ci = ci.wrapping_add(1);
    }

    // is_c 判别位读取收口于 filter 谓词（tcl 为活闭包，契约见函数头 Safety）。
    if let Some(tcl) = tcl.filter(|tcl| (*(*tcl)).is_c == 0) {
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

    enum_edge(
      ctx,
      obj2gco!(th as *mut GCObject),
      obj2gco!((*th).gt as *mut GCObject),
      EDGE_GLOBALS,
    );

    if (*th).top > (*th).stack {
      enumedges(
        ctx,
        obj2gco!(th as *mut GCObject),
        (*th).stack,
        (*th).top.offset_from((*th).stack) as usize,
        EDGE_STACK,
      );
    }
  }
}
