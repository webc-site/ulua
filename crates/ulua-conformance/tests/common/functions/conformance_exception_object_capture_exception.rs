use alloc::string::String;
use core::ffi::c_char;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_common::functions::c_str::cstr_bytes;
use ulua_vm::{
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::{lua_exception::lua_exception, lua_state::LuaState},
};

use crate::common::{
  functions::{cstr_text::cstr_text, safe_api::{is_lfunction, newthread, state_mut}},
  records::exception_result::ExceptionResult,
};
pub fn conformance_exception_object_capture_exception(
  l: *mut LuaState,
  function_to_run: *const c_char,
) -> ExceptionResult {
  // 闭包内全经 safe_api 门面：`newthread` 在 `l` 上创建线程态（对象由 `l` 的 GC
  // 持有，跨本帧使用），随后在该线程上取全局函数、断言可 callable 并 `lua_call`。
  // Lua 错误经 `lua_exception` 以 unwind 抛出，由 catch_unwind 收。
  let result = catch_unwind(AssertUnwindSafe(|| {
    let thread_state = newthread(l);
    // Safety: `function_to_run` 为调用方传入的 NUL 结尾 C 串（cstr 产物），仅本行解码一次。
    let name = unsafe { cstr_bytes(function_to_run) };
    state_mut(thread_state).get_field_bytes(LUA_GLOBALSINDEX, name);
    assert_ne!(is_lfunction(thread_state, -1), 0);
    state_mut(thread_state).call(0, 0);
  }));

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
