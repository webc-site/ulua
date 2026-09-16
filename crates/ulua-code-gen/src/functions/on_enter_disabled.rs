use ulua_vm::{
  macros::lua_callinfo_native::LUA_CALLINFO_NATIVE,
  records::{lua_state::lua_State, proto::Proto},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
///
/// 未从原生代码进入的函数之后也无法原生恢复：清掉 `l->ci->flags` 的
/// `LUA_CALLINFO_NATIVE` 位，恒返回 1（C++ `onEnterDisabled`）。
pub unsafe extern "C-unwind" fn on_enter_disabled(l: *mut lua_State, _proto: *mut Proto) -> i32 {
  unsafe {
    (*(*l).ci).flags &= !(LUA_CALLINFO_NATIVE as u32);
  }

  1
}
