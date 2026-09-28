use core::{ffi::c_void, ptr::addr_of_mut};

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_rawrunprotected_ldo::lua_d_rawrunprotected,
  macros::{
    api_check::api_check, condhardstacktests::condhardstacktests,
    expandstacklimit::expandstacklimit, luai_maxcstack::LUAI_MAXCSTACK,
    stacklimitreached::stacklimitreached,
  },
  records::{call_context_lapi::CallContext, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_checkstack(l: *mut LuaState, size: i32) -> i32 {
  unsafe {
    api_check!(l, size >= 0);

    let mut res = 1;
    if size > LUAI_MAXCSTACK || ((*l).top.offset_from((*l).base) as i32 + size) > LUAI_MAXCSTACK {
      res = 0; // stack overflow
    } else if size > 0 {
      if stacklimitreached(&*l, size) {
        let mut ctx = CallContext { size };
        // there could be no memory to extend the stack
        if lua_d_rawrunprotected(
          l,
          Some(CallContext::run_mut),
          addr_of_mut!(ctx) as *mut c_void,
        ) != LuaStatus::Ok as i32
        {
          return 0;
        }
      } else {
        condhardstacktests!(lua_d_reallocstack(l, (*l).stacksize - EXTRA_STACK, 0));
      }

      expandstacklimit!(l, (*l).top.add(size as usize));
    }
    res
  }
}
