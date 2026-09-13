use core::ffi::c_void;

use crate::{
  functions::loadsafe::loadsafe, records::load_context::LoadContext,
  type_aliases::lua_state::lua_State,
};

pub trait LoadContextRun {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  unsafe fn run(l: *mut lua_State, ud: *mut c_void);
}

impl LoadContextRun for LoadContext {
  unsafe fn run(l: *mut lua_State, ud: *mut c_void) {
    unsafe {
      let ctx = ud as *mut LoadContext;

      (*ctx).result = loadsafe(
        l,
        &mut (*ctx).strings,
        &mut (*ctx).protos,
        (*ctx).chunkname,
        (*ctx).data,
        (*ctx).size,
        (*ctx).env,
      );
    }
  }
}
