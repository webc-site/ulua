use core::ffi::c_void;

use crate::{
  functions::loadsafe::loadsafe,
  records::{load_context::LoadContext, lua_state::LuaState},
};

pub trait LoadContextRun {
  /// # Safety
  ///
  /// 仅作为 `lua_d_pcall` 的 `Pfunc` 中转回调使用：`l` 必须指向存活 `LuaState`；`ud` 必须
  /// 为空或指向调用期间全程存活、独占借出的 `LoadContext` 实例地址。
  unsafe fn run(l: *mut LuaState, ud: *mut c_void);
}

impl LoadContextRun for LoadContext<'_> {
  /// # Safety
  ///
  /// 同 trait 声明：`l` 存活、`ud` 为空或指向唯一独占存活的 `LoadContext` 实例地址。
  unsafe fn run(l: *mut LuaState, ud: *mut c_void) {
    // Safety: 契约保证 ud 为空或独占活实例地址（as_mut 判空后按可变引用绑定不相交字段）
    unsafe {
      // ud 由调用方以本类型实例地址回填；判空一次后按可变引用绑定，
      // result 回写与 strings/protos 借用为不相交字段借用
      let Some(ctx) = (ud as *mut LoadContext<'_>).as_mut() else {
        return;
      };

      ctx.result = loadsafe(
        l,
        &mut ctx.strings,
        &mut ctx.protos,
        ctx.chunkname,
        ctx.data,
        ctx.env,
      );
    }
  }
}
