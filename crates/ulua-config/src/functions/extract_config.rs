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

// `lua_resume` 返回码（对应 C++ switch 分支）
const RESUME_OK: i32 = LuaStatus::Ok as i32;
const RESUME_BREAK: i32 = LuaStatus::Break as i32;
const RESUME_YIELD: i32 = LuaStatus::Yield as i32;

// 结果表类型码
const T_TABLE: i32 = LuaType::Table as i32;

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

  // SAFETY: l 为 new_sandbox 返回的有效 VM 状态
  unsafe { (*lua_callbacks(l)).interrupt = callbacks.interrupt_callback };

  // SAFETY: l 为有效 VM 状态且已加载配置脚本
  match unsafe { lua_resume(l, null_mut(), 0) } {
    RESUME_OK => {}
    // 暂不支持调试中断
    RESUME_BREAK | RESUME_YIELD => {
      *error = String::from("configuration execution cannot yield");
      return None;
    }
    // SAFETY: resume 出错时栈顶为错误字符串
    _ => {
      *error = unsafe { lua_string(l, -1) };
      return None;
    }
  }

  // SAFETY: l 为有效 VM 状态
  if unsafe { lua_gettop(l) } != 1 {
    *error = String::from("configuration must return exactly one value");
    return None;
  }

  // SAFETY: l 为有效 VM 状态
  if unsafe { lua_type(l, -1) } != T_TABLE {
    *error = String::from("configuration did not return a table");
    return None;
  }

  // SAFETY: 栈顶（-1）为已校验的返回表
  unsafe { serialize_table(l, error) }
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
