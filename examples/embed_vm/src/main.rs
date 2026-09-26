//! 直接嵌入 VM：编译、加载到新线程、运行、读取返回值。当需要比 `ulua::eval`
//! 更多的控制（自定义全局、检查结果、多个 chunk 等）时，用这个低层（C 风格）
//! API。
//!
//!     cargo run -p ulua-example-embed-vm

use core::{ffi::c_int, ptr::null_mut};
use std::error::Error;

use ulua::{
  common::set_luau_bool_flags,
  vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
      lua_newthread::lua_newthread, lua_resume::lua_resume, lua_tonumberx::lua_tonumberx,
      luau_load::luau_load,
    },
    records::lua_state_guard::LuaStateGuard,
  },
};

/// 被编译并执行的脚本。
const SOURCE: &str = "return 6 * 7";
/// chunk 名，出现在错误回溯首列。
const CHUNK_NAME: &str = "=embed";
/// `luau_load` 的 env 槽位取 0：不指定自定义环境，用线程自身的全局表。
const DEFAULT_ENV: c_int = 0;
/// C 契约的「成功」返回值。
const OK: c_int = LuaStatus::Ok as c_int;

fn main() -> Result<(), Box<dyn Error>> {
  let bytecode = ulua::compile(SOURCE)?;

  // v11+ 字节码需要默认 Luau flags（与 CLI 一致）。
  set_luau_bool_flags(true);

  // `lua_l_newstate` 是 safe 包装（内部收口 C 分配器边界），null 表示分配失败。
  let l = lua_l_newstate();
  if l.is_null() {
    return Err("could not create Lua state".into());
  }
  // RAII 守卫：本帧退出（含下面每条早退路径）时 `lua_close`，句柄不外泄。
  let _state = LuaStateGuard(l);

  // Safety: 块内每个调用都只作用于上方新建、本帧独占且直到块结束后才关闭的
  // 状态机 `l` 及其派生线程 `t`；单线程顺序执行，满足各 C-API 例程对
  // `lua_State` 的前置条件。`lua_resume` 的 `from = null_mut()` 是「协程首次
  // 启动」的 C 契约，不是空指针解引用。
  unsafe {
    lua_l_openlibs(l);

    // 在新线程上运行，对齐参考 CLI 的 runCode。
    let t = lua_newthread(l);
    if t.is_null() {
      return Err("could not create thread".into());
    }

    if luau_load(t, CHUNK_NAME, &bytecode, DEFAULT_ENV) != OK {
      return Err("luau_load failed".into());
    }
    let status = lua_resume(t, null_mut(), 0);
    if status != OK {
      return Err(format!("script raised an error (status={status})").into());
    }

    // chunk 的返回值留在线程栈上；栈索引本身就是数据，故用 1-based 区间。
    let n = lua_gettop(t);
    println!("script returned {n} value(s):");
    for i in 1..=n {
      if let Some(v) = lua_tonumberx(t, i) {
        println!("  [{i}] = {v}");
      }
    }
  }
  Ok(())
}
