//! Port of `cpp/tests/DirectFieldAccess.test.cpp`（313 行，6 个 TEST_CASE）。
//!
//! 被测对象：`ulua_vm` 的 userdata 直接字段访问
//! （`lua_registeruserdatadirectfieldget` 及 VM 侧派发，对应 C++
//! `VM/src/lvmutils.cpp` / `Compiler` 的 `LuauDirectFieldGet` 支持）。
//!
//! 移植说明：
//! - C++ `handlerHitCount` 是文件级 `static int`；Rust 用 `AtomicI32` 镜像，
//!   并以 `Mutex` 让用到它的用例串行（同一进程内 nextest 并行跑线程，
//!   与 C++ 单测试进程串行语义对齐）。
//! - C++ 的 `std::unique_ptr<lua_State, lua_close>` 镜像为 `StateGuard`。
//! - `Luau::compile` + `luau_load` 组合镜像为 `luau_compile` + `luau_load`。
//! - 各 C 回调（createVec2 / handler 等）按 conformance 同款拆为
//!   `extern "C-unwind"` 自由函数。

use core::{
  ffi::{c_char, c_int, c_void},
  mem::size_of,
  ptr::{NonNull, null_mut},
  sync::atomic::{AtomicI32, Ordering},
};
use std::sync::{Mutex, MutexGuard};

use ulua_compiler::functions::luau_compile::luau_compile;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_close::lua_close, lua_gettop::lua_gettop, lua_isnumber::lua_isnumber,
    lua_l_checknumber::lua_l_checknumber, lua_l_newmetatable::lua_l_newmetatable,
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
    lua_newuserdatatagged::lua_newuserdatatagged,
    lua_newuserdatataggedwithmetatable::lua_newuserdatataggedwithmetatable, lua_pcall::lua_pcall,
    lua_pushnumber::lua_pushnumber,
    lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
    lua_setfield::lua_setfield, lua_setuserdatametatable::lua_setuserdatametatable,
    lua_toboolean::lua_toboolean,
    lua_userdatadirectfield_setboolean::lua_userdatadirectfield_setboolean,
    lua_userdatadirectfield_setnumber::lua_userdatadirectfield_setnumber, luau_load::luau_load,
  },
  macros::{
    lua_isboolean::lua_isboolean, lua_multret::LUA_MULTRET, lua_pushcfunction::LUA_PUSHCFUNCTION,
    lua_setglobal::lua_setglobal, lua_tonumber::lua_tonumber,
  },
  records::lua_state::lua_State,
};

const K_TAG_VEC2: c_int = 42;
const K_TAG_OTHER: c_int = 43;

/// C++ 测试用的 `Vec2` userdata 载荷
#[repr(C)]
struct Vec2 {
  x: f64,
  y: f64,
}

/// cpp `static int handlerHitCount`：文件级命中计数
static HANDLER_HIT_COUNT: AtomicI32 = AtomicI32::new(0);

/// 用到 `HANDLER_HIT_COUNT` 的用例须串行（cpp 为单线程顺序执行）
static HIT_COUNT_MUTEX: Mutex<()> = Mutex::new(());

fn lock_hit_count() -> MutexGuard<'static, ()> {
  // 锁中毒只可能来自同文件其他用例的断言失败，恢复锁继续跑即可
  HIT_COUNT_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

fn handler_hit_count() -> i32 {
  HANDLER_HIT_COUNT.load(Ordering::SeqCst)
}

fn reset_handler_hit_count() {
  HANDLER_HIT_COUNT.store(0, Ordering::SeqCst);
}

fn bump_handler_hit_count() {
  HANDLER_HIT_COUNT.fetch_add(1, Ordering::SeqCst);
}

/// `std::unique_ptr<lua_State, void (*)(lua_State*)>` 的镜像
struct StateGuard(NonNull<lua_State>);

impl StateGuard {
  /// 测试环境内存充足，`luaL_newstate` 失败即环境失效，集中一处 expect
  fn new() -> Self {
    Self(NonNull::new(lua_l_newstate()).expect("lua state allocation failed"))
  }

  fn as_ptr(&self) -> *mut lua_State {
    self.0.as_ptr()
  }
}

impl Drop for StateGuard {
  fn drop(&mut self) {
    unsafe {
      lua_close(self.as_ptr());
    }
  }
}

unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

/// cpp `runCode`：编译 + 载入 + `lua_pcall(0, LUA_MULTRET, 0)`
///
/// # Safety
/// `l` 须为 `luaL_newstate` 产出的存活状态。
unsafe fn run_code(l: *mut lua_State, source: &str) -> c_int {
  unsafe {
    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );

    if luau_load(l, c"test".as_ptr(), bytecode, bytecode_size, 0) != 0 {
      free(bytecode as *mut c_void);
      return -1; // load failed
    }

    free(bytecode as *mut c_void);
    lua_pcall(l, 0, LUA_MULTRET, 0)
  }
}

/// cpp `lua_createVec2`
unsafe extern "C-unwind" fn create_vec_2(l: *mut lua_State) -> c_int {
  unsafe {
    let x = lua_l_checknumber(l, 1);
    let y = lua_l_checknumber(l, 2);

    let p = lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_VEC2) as *mut Vec2;
    (*p).x = x;
    (*p).y = y;
    1
  }
}

/// cpp `lua_createOtherWithMt`
unsafe extern "C-unwind" fn create_other_with_mt(l: *mut lua_State) -> c_int {
  unsafe {
    lua_newuserdatataggedwithmetatable(l, size_of::<Vec2>(), K_TAG_OTHER);
    1
  }
}

/// cpp `lua_createOtherWithoutMt`
unsafe extern "C-unwind" fn create_other_without_mt(l: *mut lua_State) -> c_int {
  unsafe {
    lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_OTHER);
    1
  }
}

/// cpp 用例 1 的 handler：`setnumber(res, ud->x)`
unsafe extern "C-unwind" fn get_x_number(ud: *mut c_void, result: *mut c_void) {
  unsafe {
    lua_userdatadirectfield_setnumber(result, (*(ud as *mut Vec2)).x);
  }
}

/// cpp 用例 5 的 handler：`setnumber(res, ud->y)`
unsafe extern "C-unwind" fn get_y_number(ud: *mut c_void, result: *mut c_void) {
  unsafe {
    lua_userdatadirectfield_setnumber(result, (*(ud as *mut Vec2)).y);
  }
}

/// cpp 用例 2 的 handler：`setboolean(res, ud->x != 0 || ud->y != 0)`
unsafe extern "C-unwind" fn get_non_zero_boolean(ud: *mut c_void, result: *mut c_void) {
  unsafe {
    let vec = &*(ud as *mut Vec2);
    let non_zero = (vec.x != 0.0 || vec.y != 0.0) as c_int;
    lua_userdatadirectfield_setboolean(result, non_zero);
  }
}

/// cpp 用例 3 的 handler：计数 + `setnumber(res, ud->x)`
unsafe extern "C-unwind" fn counted_get_x_number(ud: *mut c_void, result: *mut c_void) {
  unsafe {
    bump_handler_hit_count();
    lua_userdatadirectfield_setnumber(result, (*(ud as *mut Vec2)).x);
  }
}

/// cpp 用例 6 的 kTagOther handler：`setnumber(res, 999)` + 计数
unsafe extern "C-unwind" fn counted_get_999_number(_ud: *mut c_void, result: *mut c_void) {
  unsafe {
    lua_userdatadirectfield_setnumber(result, 999.0);
    bump_handler_hit_count();
  }
}

/// cpp 用例 4 的 `__index`：恒返回 -1
unsafe extern "C-unwind" fn push_minus_one(l: *mut lua_State) -> c_int {
  unsafe {
    lua_pushnumber(l, -1.0);
    1
  }
}

mod handler_setnumber_result {
  //! Source: `tests/DirectFieldAccess.test.cpp:63-91`

  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  use super::*;

  #[test]
  fn handler_setnumber_result() {
    let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
    let state = StateGuard::new();
    let l = state.as_ptr();

    unsafe {
      lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(get_x_number));

      LUA_PUSHCFUNCTION(l, Some(create_vec_2), c"createVec2".as_ptr());
      lua_setglobal(l, c"createVec2".as_ptr());

      let status = run_code(
        l,
        r#"
        local v = createVec2(3.5, 0)
        return v.X
    "#,
      );
      assert_eq!(status, LuaStatus::Ok as i32);

      assert_ne!(lua_isnumber(l, -1), 0);
      assert_eq!(lua_tonumber!(l, -1), 3.5);
    }
  }
}

mod handler_setboolean_result {
  //! Source: `tests/DirectFieldAccess.test.cpp:93-131`

  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  use super::*;

  #[test]
  fn handler_setboolean_result() {
    let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
    let state = StateGuard::new();
    let l = state.as_ptr();

    unsafe {
      lua_registeruserdatadirectfieldget(
        l,
        K_TAG_VEC2,
        c"NonZero".as_ptr(),
        Some(get_non_zero_boolean),
      );

      LUA_PUSHCFUNCTION(l, Some(create_vec_2), c"createVec2".as_ptr());
      lua_setglobal(l, c"createVec2".as_ptr());

      {
        let status = run_code(
          l,
          r#"
            local v = createVec2(1, 0)
            return v.NonZero
        "#,
        );
        assert_eq!(status, LuaStatus::Ok as i32);
        assert!(lua_isboolean!(l, -1));
        assert_eq!(lua_toboolean(l, -1), 1);
      }
      {
        let status = run_code(
          l,
          r#"
            local v = createVec2(0, 0)
            return v.NonZero
        "#,
        );
        assert_eq!(status, LuaStatus::Ok as i32);
        assert!(lua_isboolean!(l, -1));
        assert_eq!(lua_toboolean(l, -1), 0);
      }
    }
  }
}

mod repeated_access_handler_called_every_iteration {
  //! Source: `tests/DirectFieldAccess.test.cpp:133-168`

  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  use super::*;

  #[test]
  fn repeated_access_handler_called_every_iteration() {
    let _lock = lock_hit_count();
    let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
    let state = StateGuard::new();
    let l = state.as_ptr();

    reset_handler_hit_count();

    unsafe {
      lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(counted_get_x_number));

      LUA_PUSHCFUNCTION(l, Some(create_vec_2), c"createVec2".as_ptr());
      lua_setglobal(l, c"createVec2".as_ptr());

      let status = run_code(
        l,
        r#"
        local v = createVec2(7, 0)
        local sum = 0
        for i = 1, 5 do
            sum = sum + v.X
        end
        return sum
    "#,
      );
      assert_eq!(status, LuaStatus::Ok as i32);
      assert_ne!(lua_isnumber(l, -1), 0);
      assert_eq!(lua_tonumber!(l, -1), 35.0);

      assert_eq!(handler_hit_count(), 5);
    }
  }
}

mod unregistered_tag_falls_through_to_index_metamethod {
  //! Source: `tests/DirectFieldAccess.test.cpp:170-224`

  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  use super::*;

  #[test]
  fn unregistered_tag_falls_through_to_index_metamethod() {
    let _lock = lock_hit_count();
    let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
    let state = StateGuard::new();
    let l = state.as_ptr();

    unsafe {
      lua_l_openlibs(l);
    }

    reset_handler_hit_count();

    unsafe {
      lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(counted_get_x_number));

      // 给 kTagOther 挂元表，__index 对任意字段返回 -1
      lua_l_newmetatable(l, c"metaOther".as_ptr());
      LUA_PUSHCFUNCTION(l, Some(push_minus_one), c"__index".as_ptr());
      lua_setfield(l, -2, c"__index".as_ptr());
      lua_setuserdatametatable(l, K_TAG_OTHER);

      LUA_PUSHCFUNCTION(l, Some(create_vec_2), c"createVec2".as_ptr());
      lua_setglobal(l, c"createVec2".as_ptr());
      LUA_PUSHCFUNCTION(l, Some(create_other_with_mt), c"createOther".as_ptr());
      lua_setglobal(l, c"createOther".as_ptr());

      let status = run_code(
        l,
        r#"
        local uds = {createVec2(1, 0), createOther()}
        local results = {}
        for _, v in uds do
            results[#results + 1] = v.X
        end
        return table.unpack(results)
    "#,
      );
      assert_eq!(status, LuaStatus::Ok as i32);
      assert_eq!(lua_gettop(l), 2);

      assert_eq!(lua_tonumber!(l, -2), 1.0); // 直接派发生效
      assert_eq!(lua_tonumber!(l, -1), -1.0); // kTagOther 无派发表，回落 __index

      assert_eq!(handler_hit_count(), 1); // handler 只命中 Vec2
    }
  }
}

mod multiple_fields_same_type_dispatch_independently {
  //! Source: `tests/DirectFieldAccess.test.cpp:226-264`

  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  use super::*;

  #[test]
  fn multiple_fields_same_type_dispatch_independently() {
    let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
    let state = StateGuard::new();
    let l = state.as_ptr();

    unsafe {
      lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(get_x_number));
      lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"Y".as_ptr(), Some(get_y_number));

      LUA_PUSHCFUNCTION(l, Some(create_vec_2), c"createVec2".as_ptr());
      lua_setglobal(l, c"createVec2".as_ptr());

      let status = run_code(
        l,
        r#"
        local v = createVec2(1.5, 2.5)
        return v.X, v.Y
    "#,
      );
      assert_eq!(status, LuaStatus::Ok as i32);
      assert_eq!(lua_gettop(l), 2);

      assert_eq!(lua_tonumber!(l, -2), 1.5);
      assert_eq!(lua_tonumber!(l, -1), 2.5);
    }
  }
}

mod same_field_name_different_tags_dispatch_independently {
  //! Source: `tests/DirectFieldAccess.test.cpp:266-311`

  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  use super::*;

  /// cpp 用例 6 的 kTagVec2 handler：`setnumber(res, ud->x)` + 计数
  unsafe extern "C-unwind" fn counted_get_x(ud: *mut c_void, result: *mut c_void) {
    unsafe {
      lua_userdatadirectfield_setnumber(result, (*(ud as *mut Vec2)).x);
      bump_handler_hit_count();
    }
  }

  #[test]
  fn same_field_name_different_tags_dispatch_independently() {
    let _lock = lock_hit_count();
    let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
    let state = StateGuard::new();
    let l = state.as_ptr();

    unsafe {
      lua_l_openlibs(l);
    }

    reset_handler_hit_count();

    unsafe {
      lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(counted_get_x));
      lua_registeruserdatadirectfieldget(
        l,
        K_TAG_OTHER,
        c"X".as_ptr(),
        Some(counted_get_999_number),
      );

      LUA_PUSHCFUNCTION(l, Some(create_vec_2), c"createVec2".as_ptr());
      lua_setglobal(l, c"createVec2".as_ptr());

      LUA_PUSHCFUNCTION(l, Some(create_other_without_mt), c"createOther".as_ptr());
      lua_setglobal(l, c"createOther".as_ptr());

      let status = run_code(
        l,
        r#"
        return createVec2(3, 0).X, createOther().X
    "#,
      );
      assert_eq!(status, LuaStatus::Ok as i32);
      assert_eq!(lua_gettop(l), 2);

      assert_eq!(lua_tonumber!(l, -2), 3.0);
      assert_eq!(lua_tonumber!(l, -1), 999.0);
      assert_eq!(handler_hit_count(), 2);
    }
  }
}
