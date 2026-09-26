//! 与 `records::call_context_lgc` 对应：lgc.cpp 三个同名局部
//! `CallContext::run` 中转函数，类型区分见 records 模块头注释。
use core::ffi::c_void;

use crate::{
  functions::{
    lua_h_resizehash::lua_h_resizehash, lua_s_resize::lua_s_resize, shrinkstack::shrinkstack,
  },
  records::{
    call_context_lgc::{CallContext, StringResizeCallContext, TableResizeCallContext},
    lua_state::LuaState,
  },
};

impl CallContext {
  /// # Safety
  ///
  /// 仅作为 `lua_d_pcall` 的 `Pfunc` 中转回调使用：`l` 必须指向待收缩栈的存活 `LuaState`；
  /// `_ud` 未被读取，可传任意值（含空）。
  pub unsafe extern "C-unwind" fn run(l: *mut LuaState, _ud: *mut c_void) {
    // Safety: 契约保证 `l` 指向存活 LuaState，shrinkstack 仅在其上重排栈
    unsafe {
      shrinkstack(l);
    }
  }
}

impl TableResizeCallContext {
  /// # Safety
  ///
  /// 仅作为 `lua_d_pcall` 的 `Pfunc` 中转回调使用：`l` 必须指向存活 `LuaState`；`ud` 必须
  /// 为空或指向调用期间全程存活的 `TableResizeCallContext` 实例，其 `t` 须为该状态管理的存活表。
  pub(crate) unsafe extern "C-unwind" fn run(l: *mut LuaState, ud: *mut c_void) {
    // Safety: 契约保证 ud 为空或活实例地址（as_ref 判空后按引用读 t/nhsize），lua_h_resizehash 随 l 管理该表
    unsafe {
      // ud 由调用方以本类型实例地址回填；判空一次后按引用读取字段
      let Some(ctx) = (ud as *mut TableResizeCallContext).as_ref() else {
        return;
      };
      lua_h_resizehash(l, ctx.t, ctx.nhsize);
    }
  }
}

impl StringResizeCallContext {
  /// # Safety
  ///
  /// 仅作为 `lua_d_pcall` 的 `Pfunc` 中转回调使用：`l` 必须指向存活 `LuaState`；`ud` 必须
  /// 为空或指向调用期间全程存活的 `StringResizeCallContext` 实例地址。
  pub(crate) unsafe extern "C-unwind" fn run(l: *mut LuaState, ud: *mut c_void) {
    // Safety: 契约保证 ud 为空或活实例地址（as_ref 判空后按引用读 newsize），lua_s_resize 随 l 调整字符串桶
    unsafe {
      // ud 由调用方以本类型实例地址回填；判空一次后按引用读取字段
      let Some(ctx) = (ud as *mut StringResizeCallContext).as_ref() else {
        return;
      };
      lua_s_resize(l, ctx.newsize);
    }
  }
}
