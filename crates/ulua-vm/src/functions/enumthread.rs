/// NUL 结尾字节串（边名指针经 `enum_edge` 收口取；§10 不引入 C 字符串类型）。
use core::{
  ffi::c_char,
  mem::size_of,
  ptr::{addr_of, null},
};

use crate::{
  functions::{
    c_slice,
    enumedge::enum_edge,
    enumedges::enumedges,
    enumnode::enumnode,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{getstr::getstr, obj_2_gco::obj2gco},
  records::{
    call_info::CallInfo, closure::LClosure, enum_context::EnumContext, lua_state::LuaState,
    proto::Proto,
  },
  type_aliases::t_value::TValue,
};
const EDGE_GLOBALS: &[u8] = b"globals\0";
const EDGE_STACK: &[u8] = b"stack\0";

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn enumthread(ctx: *mut EnumContext, th: *mut LuaState) {
  unsafe {
    let size = size_of::<LuaState>()
      + size_of::<TValue>() * (*th).stacksize as usize
      + size_of::<CallInfo>() * (*th).size_ci as usize;

    // 首个函数帧的闭包：base_ci..=ci 为连续 CallInfo 数组，切片 find_map 取代
    // `while ci <= ci` 手工裸偏移 + break；「未找到」由 None 承载，不用 null 哨兵（§2）。
    let frames = c_slice(
      (*th).base_ci,
      (*th).ci.offset_from((*th).base_ci) as usize + 1,
    );
    let tcl = frames.iter().find_map(|frame| {
      let func = &*frame.func;
      func.is_function().then(|| func.as_closure_ptr())
    });

    // is_c 判别位读取收口于 filter 谓词（tcl 为活闭包，契约见函数头 Safety）。
    if let Some(tcl) = tcl.filter(|tcl| (*(*tcl)).is_c == 0) {
      let tcl_l = addr_of!((*tcl).inner.l).cast::<LClosure>();
      let p: *mut Proto = (*tcl_l).p;
      if !p.is_null() {
        let mut buf = [0u8; 256];

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

        enumnode(ctx, obj2gco!(th), size, buf.as_ptr().cast::<c_char>());
      } else {
        enumnode(ctx, obj2gco!(th), size, null());
      }
    } else {
      enumnode(ctx, obj2gco!(th), size, null());
    }

    enum_edge(ctx, obj2gco!(th), obj2gco!((*th).gt), EDGE_GLOBALS);

    if (*th).top > (*th).stack {
      enumedges(
        ctx,
        obj2gco!(th),
        (*th).stack,
        (*th).top.offset_from((*th).stack) as usize,
        EDGE_STACK,
      );
    }
  }
}
