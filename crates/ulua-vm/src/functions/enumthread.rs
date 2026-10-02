/// NUL 结尾字节串（边名指针经 [`enum_edge`] 收口取；§10 不引入 C 字符串类型）。
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
  macros::getstr::getstr,
  records::{
    call_info::CallInfo, closure::LClosure, enum_context::EnumContext, gc_object::GCObject,
    lua_state::LuaState, proto::Proto,
  },
  type_aliases::t_value::TValue,
};
const EDGE_GLOBALS: &[u8] = b"globals\0";
const EDGE_STACK: &[u8] = b"stack\0";

/// # Safety
/// `ctx` 须为存活 `EnumContext`（其 `node`/`edge` 回调可用）；`th` 须为存活线程 LuaState，其
/// `base_ci..=ci`、`stack..top` 为同数组内合法区间，栈槽与帧内闭包/Proto 字段可读。只读枚举并回调上报。
pub(crate) unsafe fn enumthread(ctx: *mut EnumContext, th: &LuaState) {
  unsafe {
    let size = size_of::<LuaState>()
      + size_of::<TValue>() * th.stacksize as usize
      + size_of::<CallInfo>() * th.size_ci as usize;

    // 枚举身份取对象首字节地址：`obj2gco!` 要求裸指针入参，共享句柄降 `*const` 即够用
    let obj = (th as *const LuaState).cast::<GCObject>();

    // 首个函数帧的闭包：base_ci..=ci 为连续 CallInfo 数组，切片 find_map 取代
    // `while ci <= ci` 手工裸偏移 + break；「未找到」由 None 承载，不用 null 哨兵（§2）。
    let frames = c_slice(th.base_ci, th.ci.offset_from(th.base_ci) as usize + 1);
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

        enumnode(ctx, obj, size, buf.as_ptr().cast::<c_char>());
      } else {
        enumnode(ctx, obj, size, null());
      }
    } else {
      enumnode(ctx, obj, size, null());
    }

    enum_edge(ctx, obj, th.gt, EDGE_GLOBALS);

    if th.top > th.stack {
      enumedges(
        ctx,
        obj,
        th.stack,
        // r16-b2 收编：stack 锚槽距读数落 slot_distance 边界原语（stack.rs:53，本票升
        // pub(crate) 解锁）——本体即被替代式 `to.offset_from(from) as i32` 的同址镜像；
        // 上方守卫已证窗非负（top>stack），isize→i32 折形在 stacksize 约束域无截差，
        // 非负值 `as usize` 收窄与原式逐位同值（本点位原形无 max，不补）
        LuaState::slot_distance(th.stack, th.top) as usize,
        EDGE_STACK,
      );
    }
  }
}
