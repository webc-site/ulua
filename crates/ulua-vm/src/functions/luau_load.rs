use core::{
  ffi::{c_char, c_int, c_void},
  ptr::null_mut,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_rawrunprotected_ldo::luaD_rawrunprotected, lua_pushstring::lua_pushstring},
  macros::lua_c_check_gc::luaC_checkGC,
  methods::load_context_run::LoadContextRun,
  records::{
    load_context::LoadContext, scoped_set_gc_threshold::ScopedSetGcThreshold,
    temp_buffer::TempBuffer,
  },
  type_aliases::lua_state::lua_State,
};

// trait 方法不能声明 `extern "C"`,以 C ABI 包装转发(供 Pfunc 回调使用)
unsafe extern "C-unwind" fn load_context_run(l: *mut lua_State, ud: *mut c_void) {
  unsafe { <LoadContext as LoadContextRun>::run(l, ud) }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_load(
  l: *mut lua_State,
  chunkname: *const c_char,
  data: *const c_char,
  size: usize,
  env: c_int,
) -> c_int {
  unsafe {
    // we will allocate a fair amount of memory so check GC before we do
    luaC_checkGC!(l);

    // pause GC for the duration of deserialization - some objects we're creating aren't rooted
    let mut pause_gc = ScopedSetGcThreshold {
      global: null_mut(),
      original_threshold: 0,
    };
    pause_gc.scoped_set_gc_threshold_global_state_usize((*l).global, usize::MAX);

    let mut ctx = LoadContext {
      strings: TempBuffer::new(),
      protos: TempBuffer::new(),
      chunkname,
      data,
      size,
      env,
      result: 0,
    };

    let status = luaD_rawrunprotected(
      l,
      Some(load_context_run),
      &mut ctx as *mut LoadContext as *mut c_void,
    );

    // load can either succeed or get an OOM error, any other errors should be handled internally
    LUAU_ASSERT!(status == LuaStatus::Ok as c_int || status == LuaStatus::ErrMem as c_int);

    let result = if status == LuaStatus::ErrMem as c_int {
      lua_pushstring(l, c"not enough memory".as_ptr() as *const c_char);
      1
    } else {
      ctx.result
    };

    drop(pause_gc);

    result
  }
}
