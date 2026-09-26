use alloc::string::String;
use core::ffi::c_char;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_vm::{
  functions::{
    lua_call::lua_call, lua_getfield::lua_getfield, lua_is_lfunction::lua_is_lfunction,
    lua_newthread::lua_newthread,
  },
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::{lua_exception::lua_exception, lua_state::LuaState},
};

use crate::common::{functions::cstr_text::cstr_text, records::exception_result::ExceptionResult};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn conformance_exception_object_capture_exception(
  l: *mut LuaState,
  function_to_run: *const c_char,
) -> ExceptionResult {
  // Safety: `l` 为本用例存活的 LuaState；闭包内全为 C ABI 调用——`lua_newthread` 在 `l`
  // 上创建线程态（对象由 `l` 的 GC 持有，跨本帧使用），随后在该线程上取全局函数、
  // 断言可调用并 `lua_call`。Lua 错误经 `lua_exception` 以 unwind 抛出，由 catch_unwind 收。
  let result = unsafe {
    catch_unwind(AssertUnwindSafe(|| {
      let thread_state = lua_newthread(l);
      lua_getfield(thread_state, LUA_GLOBALSINDEX, function_to_run);
      assert_ne!(lua_is_lfunction(thread_state, -1), 0);
      lua_call(thread_state, 0, 0);
    }))
  };

  match result {
    Ok(()) => ExceptionResult {
      exception_generated: false,
      description: String::new(),
    },
    Err(payload) => {
      if let Some(e) = payload.downcast_ref::<lua_exception>() {
        // `what()` 是 safe 访问器；空指针先按 cpp 断言拦下。
        let what = e.what();
        assert!(!what.is_null());
        // Safety: 上一断言保证 `what` 非空；`lua_exception` 保证其为 NUL 结尾串
        // （lossy 渲染仅用于诊断文本）。
        let description = unsafe { cstr_text(what) }.into_owned();
        ExceptionResult {
          exception_generated: true,
          description,
        }
      } else {
        resume_unwind(payload);
      }
    }
  }
}
