use alloc::string::String;
use core::ptr::null_mut;

use ulua_vm::{
  enums::{lua_status::LuaStatus, lua_type::LuaType},
  functions::{
    lua_callbacks::lua_callbacks, lua_close::lua_close, lua_gettop::lua_gettop,
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
    lua_resume::lua_resume, lua_type::lua_type,
  },
  type_aliases::lua_state::lua_State,
};

use crate::{
  functions::{load::load, lua_string::lua_string, serialize_table::serialize_table},
  records::{config_table::ConfigTable, interrupt_callbacks::InterruptCallbacks},
};

/// 沙箱 VM 状态守卫，Drop 时自动 `lua_close`。
pub(crate) struct StateGuard(pub(crate) *mut lua_State);

impl Drop for StateGuard {
  fn drop(&mut self) {
    unsafe {
      if !self.0.is_null() {
        lua_close(self.0);
      }
    }
  }
}

/// 创建沙箱 VM（对应 C++ `luaL_newstate + luaL_openlibs + luaL_sandbox`）。
pub(crate) fn new_sandbox() -> StateGuard {
  let state = StateGuard(lua_l_newstate());

  unsafe {
    lua_l_openlibs(state.0);
    lua_l_sandbox(state.0);
  }

  state
}

/// 对应 C++ `executeAndExtractConfig`：跑中断回调、resume 并校验返回表。
pub(crate) fn execute_and_extract(
  l: *mut lua_State,
  callbacks: &InterruptCallbacks,
  error: &mut String,
) -> Option<ConfigTable> {
  if let Some(init_callback) = &callbacks.init_callback {
    init_callback(l);
  }

  unsafe {
    (*lua_callbacks(l)).interrupt = callbacks.interrupt_callback;

    match lua_resume(l, null_mut(), 0) {
      status if status == LuaStatus::Ok as i32 => {}
      // 暂不支持调试中断
      status if status == LuaStatus::Break as i32 || status == LuaStatus::Yield as i32 => {
        *error = String::from("configuration execution cannot yield");
        return None;
      }
      _ => {
        *error = lua_string(l, -1);
        return None;
      }
    }

    if lua_gettop(l) != 1 {
      *error = String::from("configuration must return exactly one value");
      return None;
    }

    if lua_type(l, -1) != LuaType::Table as i32 {
      *error = String::from("configuration did not return a table");
      return None;
    }

    serialize_table(l, error)
  }
}

/// 对应 C++ `extractConfig`：在沙箱 VM 中执行 `source` 并提取返回的配置表。
pub fn extract_config(
  source: &str,
  callbacks: &InterruptCallbacks,
  error: &mut String,
) -> Option<ConfigTable> {
  let state = new_sandbox();
  let l = state.0;

  if let Some(load_error) = load(l, source) {
    *error = load_error;
    return None;
  }

  execute_and_extract(l, callbacks, error)
}
