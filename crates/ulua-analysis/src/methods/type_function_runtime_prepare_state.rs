//! Faithful port of `void TypeFunctionRuntime::prepareState()`
//! (Analysis/src/TypeFunctionRuntime.cpp:230-248).
/// `extern "C"` closer thunk for the `StateRef` deleter slot. C++ stores
/// `lua_close` as the `unique_ptr` deleter; the Rust `StateRef` deleter is
/// `extern "C-unwind" fn(*mut analysis::lua_State)`, so bridge to the VM's `lua_close`.
use core::ffi::c_void;
use core::ptr::null_mut;

use ulua_vm::{
  functions::{
    lua_close::lua_close, lua_l_sandbox::lua_l_sandbox, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_newstate::lua_newstate, lua_setthreaddata::lua_setthreaddata,
  },
  records::lua_state,
};

use crate::{
  functions::{
    register_type_user_data::register_type_user_data,
    register_types_library::register_types_library,
    set_type_function_environment::set_type_function_environment,
    type_function_alloc::type_function_alloc,
  },
  records::type_function_runtime::TypeFunctionRuntime,
  type_aliases::lua_state::LuaState,
};
unsafe extern "C-unwind" fn lua_close_thunk(l: *mut LuaState) {
  unsafe {
    lua_close(l as *mut lua_state::LuaState);
  }
}

impl TypeFunctionRuntime {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn prepare_state(&mut self) {
    unsafe {
      // if (state) return;
      if !self.state.0.is_null() {
        return;
      }

      // state = StateRef(lua_newstate(typeFunctionAlloc, nullptr), lua_close);
      let new_state = lua_newstate(Some(type_function_alloc), null_mut()) as *mut LuaState;
      self.state = (new_state, Some(lua_close_thunk));

      // lua_State* l = state.get();
      let l = self.state.0;
      let vm_l = l as *mut lua_state::LuaState;

      // lua_setthreaddata(l, this);
      lua_setthreaddata(vm_l, self as *mut TypeFunctionRuntime as *mut c_void);

      // setTypeFunctionEnvironment(l);
      set_type_function_environment(l);

      // registerTypeUserData(l);
      register_type_user_data(l);

      // registerTypesLibrary(l);
      register_types_library(l);

      // luaL_sandbox(l);
      lua_l_sandbox(vm_l);
      // luaL_sandboxthread(l);
      lua_l_sandboxthread(vm_l);
    }
  }
}
