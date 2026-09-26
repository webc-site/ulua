use core::ffi::c_void;

use crate::{
  functions::lua_d_growstack::lua_d_growstack,
  records::{call_context_lapi::CallContext, lua_state::LuaState},
};

impl CallContext {
  /// # Safety
  ///
  /// 仅作为 `lua_d_pcall` 的 `Pfunc` 中转回调使用：`l` 必须指向当前执行协程的存活
  /// `LuaState`；`ud` 必须为空或指向调用期间全程存活的 `CallContext` 实例地址。
  pub(crate) unsafe extern "C-unwind" fn run_mut(l: *mut LuaState, ud: *mut c_void) {
    // Safety: 契约保证 ud 为空或活 CallContext 地址（as_ref 判空后按引用读 size），l 可供 growstack 用
    unsafe {
      // ud 由调用方以本类型实例地址回填；判空一次后按引用读取字段
      let Some(ctx) = (ud as *mut CallContext).as_ref() else {
        return;
      };
      lua_d_growstack(l, ctx.size);
    }
  }
}
