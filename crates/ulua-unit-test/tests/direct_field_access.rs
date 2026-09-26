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
//! - C++ 的 `std::unique_ptr<LuaState, lua_close>` 镜像为 `StateGuard`。
//! - `Luau::compile` + `luau_load` 组合镜像为 `luau_compile` + `luau_load`。
//! - 各 C 回调（createVec2 / handler 等）按 conformance 同款拆为
//!   `extern "C-unwind"` 自由函数。

use core::{
  ffi::{c_char, c_int, c_void},
  mem::size_of,
  ptr::{NonNull, null_mut},
  slice::from_raw_parts,
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
  records::lua_state::LuaState,
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

/// 用到 `HANDLER_HIT_COUNT` 的用例经此互斥锁串行（libtest 并行跑用例；
/// cpp 侧为单线程故无需此锁）
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

/// `std::unique_ptr<LuaState, void (*)(LuaState*)>` 的镜像
struct StateGuard(NonNull<LuaState>);

impl StateGuard {
  /// 测试环境内存充足，`luaL_newstate` 失败即环境失效，集中一处 expect
  fn new() -> Self {
    Self(NonNull::new(lua_l_newstate()).expect("lua state allocation failed"))
  }

  fn as_ptr(&self) -> *mut LuaState {
    self.0.as_ptr()
  }
}

impl Drop for StateGuard {
  fn drop(&mut self) {
    // Safety: self.0 为 NonNull（判空构造期已排除）且是本 guard 独占存活的
    // luaL_newstate 状态，落栈尾一次性关闭。
    unsafe {
      lua_close(self.as_ptr());
    }
  }
}

unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

/// cpp `lua_pushcfunction + lua_setglobal` 成对样板的 (c) 类 vm C-API 最小收口。
///
/// Safety: `l` 须为存活 LuaState；`name` 为调用帧内有效的 NUL 结尾 C 串名；
/// `f` 须符合 `lua_CFunction` 契约（extern "C-unwind"、只操作栈顶约定槽位）。
fn push_global(
  l: *mut LuaState,
  name: &[u8],
  f: unsafe extern "C-unwind" fn(*mut LuaState) -> c_int,
) {
  unsafe {
    LUA_PUSHCFUNCTION(l, Some(f), name.as_ptr().cast());
    lua_setglobal(l, name.as_ptr().cast());
  }
}

/// cpp `runCode`：编译 + 载入 + `lua_pcall(0, LUA_MULTRET, 0)`。
/// 原「整函数 unsafe」已拆为逐个带契约的最小 (c) 类 C-API 边界，函数体本身安全。
///
/// `l` 按调用方契约须为 `luaL_newstate` 产出的存活状态。
fn run_code(l: *mut LuaState, source: &str) -> c_int {
  let mut bytecode_size = 0usize;
  // Safety: luau_compile 为 C API——source.as_ptr()/len 恒为合法 UTF-8 缓冲
  //（空串时 as_ptr 仍对齐有效、按长度 0 读取），options 传 null 表默认选项
  //（callee 契约容忍），outsize 指向本帧可写局部。
  let bytecode = unsafe {
    luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    )
  };

  // Safety: luau_compile 交付的缓冲区 bytecode_size 个字节可读；切片最后一次使用
  // 在 luau_load 内（载入即消化），free 排在其后，无 use-after-free。
  let load_status = unsafe {
    let bytes = from_raw_parts(bytecode.cast::<u8>(), bytecode_size);
    luau_load(l, "test", bytes, 0)
  };

  // Safety: bytecode 是 luau_compile 交出的分配器块，两分支均在使用完毕后
  // 一次性 free（与原逐分支 free 时序等价）。
  unsafe { free(bytecode.cast::<c_void>()) };

  if load_status != 0 {
    return -1; // load failed
  }

  // Safety: l 按本 fn 契约为存活状态；lua_pcall 遵循 C 栈契约（0 入参、
  // LUA_MULTRET 返回、无消息处理器）。
  unsafe { lua_pcall(l, 0, LUA_MULTRET, 0) }
}

/// cpp `lua_createVec2`
unsafe extern "C-unwind" fn create_vec_2(l: *mut LuaState) -> c_int {
  // Safety: l 为 VM 传入的存活状态（(c) 类回调契约）；lua_newuserdatatagged
  // 返回 K_TAG_VEC2  sized 对齐的 Vec2 载荷块，非空且本帧可写。
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
unsafe extern "C-unwind" fn create_other_with_mt(l: *mut LuaState) -> c_int {
  // Safety: l 为 VM 传入的存活状态（(c) 类回调契约），仅 C-API 压栈。
  unsafe {
    lua_newuserdatataggedwithmetatable(l, size_of::<Vec2>(), K_TAG_OTHER);
    1
  }
}

/// cpp `lua_createOtherWithoutMt`
unsafe extern "C-unwind" fn create_other_without_mt(l: *mut LuaState) -> c_int {
  // Safety: l 为 VM 传入的存活状态（(c) 类回调契约），仅 C-API 压栈。
  unsafe {
    lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_OTHER);
    1
  }
}

/// cpp 用例 1 的 handler：`setnumber(res, ud->x)`
unsafe extern "C-unwind" fn get_x_number(ud: *mut c_void, result: *mut c_void) {
  // Safety: 直接字段派发契约——ud 指向存活 Vec2 载荷、result 为其字段结果槽
  //（(c) 类，VM 派发器保证两指针非空且帧内有效）。
  unsafe {
    lua_userdatadirectfield_setnumber(result, (*(ud as *mut Vec2)).x);
  }
}

/// cpp 用例 5 的 handler：`setnumber(res, ud->y)`
unsafe extern "C-unwind" fn get_y_number(ud: *mut c_void, result: *mut c_void) {
  // Safety: 同 get_x_number 的派发契约（ud 存活 Vec2、result 帧内结果槽）。
  unsafe {
    lua_userdatadirectfield_setnumber(result, (*(ud as *mut Vec2)).y);
  }
}

/// cpp 用例 2 的 handler：`setboolean(res, ud->x != 0 || ud->y != 0)`
unsafe extern "C-unwind" fn get_non_zero_boolean(ud: *mut c_void, result: *mut c_void) {
  // Safety: 派发契约下 ud 指向存活 Vec2，& 共享读仅拷出两个 f64 标量；result 帧内有效。
  unsafe {
    let vec = &*(ud as *mut Vec2);
    let non_zero = (vec.x != 0.0 || vec.y != 0.0) as c_int;
    lua_userdatadirectfield_setboolean(result, non_zero);
  }
}

/// cpp 用例 3 的 handler：计数 + `setnumber(res, ud->x)`
unsafe extern "C-unwind" fn counted_get_x_number(ud: *mut c_void, result: *mut c_void) {
  // Safety: 同 get_x_number 的派发契约；计数走原子，与本解引用无耦合。
  unsafe {
    bump_handler_hit_count();
    lua_userdatadirectfield_setnumber(result, (*(ud as *mut Vec2)).x);
  }
}

/// cpp 用例 6 的 kTagOther handler：`setnumber(res, 999)` + 计数
unsafe extern "C-unwind" fn counted_get_999_number(_ud: *mut c_void, result: *mut c_void) {
  // Safety: 派发契约下 result 为帧内有效结果槽；不触碰 ud。
  unsafe {
    lua_userdatadirectfield_setnumber(result, 999.0);
    bump_handler_hit_count();
  }
}

/// cpp 用例 4 的 `__index`：恒返回 -1
unsafe extern "C-unwind" fn push_minus_one(l: *mut LuaState) -> c_int {
  // Safety: l 为 VM 传入的存活状态（(c) 类回调契约），仅压入一个 number。
  unsafe {
    lua_pushnumber(l, -1.0);
    1
  }
}

// Source: `tests/DirectFieldAccess.test.cpp:63-91`
#[test]
fn handler_setnumber_result() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = StateGuard::new();
  let l = state.as_ptr();

  // Safety: l 为刚建存活状态；c"X" 为 NUL 结尾静态名串，handler 签名符合契约。
  unsafe {
    lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(get_x_number));
  }
  push_global(l, b"createVec2\0", create_vec_2);

  let status = run_code(
    l,
    r#"
        local v = createVec2(3.5, 0)
        return v.X
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);

  // Safety: l 存活，栈顶为返回值；lua_isnumber/lua_tonumber 为只读栈操作。
  unsafe {
    assert_ne!(lua_isnumber(l, -1), 0);
    assert_eq!(lua_tonumber!(l, -1), 3.5);
  }
}

// Source: `tests/DirectFieldAccess.test.cpp:93-131`
#[test]
fn handler_setboolean_result() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = StateGuard::new();
  let l = state.as_ptr();

  // Safety: l 为刚建存活状态；c"NonZero" 为 NUL 结尾静态名串，handler 签名符合契约。
  unsafe {
    lua_registeruserdatadirectfieldget(
      l,
      K_TAG_VEC2,
      c"NonZero".as_ptr(),
      Some(get_non_zero_boolean),
    );
  }
  push_global(l, b"createVec2\0", create_vec_2);

  let status = run_code(
    l,
    r#"
        local v = createVec2(1, 0)
        return v.NonZero
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  // Safety: l 存活，栈顶为返回值；只读栈操作。
  unsafe {
    assert!(lua_isboolean!(l, -1));
    assert_eq!(lua_toboolean(l, -1), 1);
  }

  let status = run_code(
    l,
    r#"
        local v = createVec2(0, 0)
        return v.NonZero
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  // Safety: 同上轮，l 存活、只读栈操作。
  unsafe {
    assert!(lua_isboolean!(l, -1));
    assert_eq!(lua_toboolean(l, -1), 0);
  }
}

// Source: `tests/DirectFieldAccess.test.cpp:133-168`
#[test]
fn repeated_access_handler_called_every_iteration() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _lock = lock_hit_count();
  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = StateGuard::new();
  let l = state.as_ptr();

  reset_handler_hit_count();

  // Safety: l 为刚建存活状态；c"X" 为 NUL 结尾静态名串，handler 签名符合契约。
  unsafe {
    lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(counted_get_x_number));
  }
  push_global(l, b"createVec2\0", create_vec_2);

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
  // Safety: l 存活，栈顶为返回值；只读栈操作。
  unsafe {
    assert_ne!(lua_isnumber(l, -1), 0);
    assert_eq!(lua_tonumber!(l, -1), 35.0);
  }

  assert_eq!(handler_hit_count(), 5);
}

// Source: `tests/DirectFieldAccess.test.cpp:170-224`
#[test]
fn unregistered_tag_falls_through_to_index_metamethod() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _lock = lock_hit_count();
  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = StateGuard::new();
  let l = state.as_ptr();

  // Safety: l 为刚建存活状态，lua_l_openlibs 遵循 C 栈契约。
  unsafe {
    lua_l_openlibs(l);
  }

  reset_handler_hit_count();

  // Safety: l 存活；c"X"/c"metaOther"/c"__index" 均为 NUL 结尾静态串；
  // -2 槽为 newmetatable 压入的元表（lua_setfield/lua_setuserdatametatable
  // 依 C API 时序契约消费）。
  unsafe {
    lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(counted_get_x_number));

    // 给 kTagOther 挂元表，__index 对任意字段返回 -1
    lua_l_newmetatable(l, c"metaOther".as_ptr());
    LUA_PUSHCFUNCTION(l, Some(push_minus_one), c"__index".as_ptr());
    lua_setfield(l, -2, c"__index".as_ptr());
    lua_setuserdatametatable(l, K_TAG_OTHER);
  }
  push_global(l, b"createVec2\0", create_vec_2);
  push_global(l, b"createOther\0", create_other_with_mt);

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
  // Safety: l 存活，栈顶两值为返回；只读栈操作。
  unsafe {
    assert_eq!(lua_gettop(l), 2);

    assert_eq!(lua_tonumber!(l, -2), 1.0); // 直接派发生效
    assert_eq!(lua_tonumber!(l, -1), -1.0); // kTagOther 无派发表，回落 __index
  }

  assert_eq!(handler_hit_count(), 1); // handler 只命中 Vec2
}

// Source: `tests/DirectFieldAccess.test.cpp:226-264`
#[test]
fn multiple_fields_same_type_dispatch_independently() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = StateGuard::new();
  let l = state.as_ptr();

  // Safety: l 为刚建存活状态；c"X"/c"Y" 为 NUL 结尾静态名串，handler 签名符合契约。
  unsafe {
    lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(get_x_number));
    lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"Y".as_ptr(), Some(get_y_number));
  }
  push_global(l, b"createVec2\0", create_vec_2);

  let status = run_code(
    l,
    r#"
        local v = createVec2(1.5, 2.5)
        return v.X, v.Y
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  // Safety: l 存活，栈顶两值为返回；只读栈操作。
  unsafe {
    assert_eq!(lua_gettop(l), 2);

    assert_eq!(lua_tonumber!(l, -2), 1.5);
    assert_eq!(lua_tonumber!(l, -1), 2.5);
  }
}

mod same_field_name_different_tags_dispatch_independently {
  //! Source: `tests/DirectFieldAccess.test.cpp:266-311`

  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  use super::*;

  /// cpp 用例 6 的 kTagVec2 handler：`setnumber(res, ud->x)` + 计数
  unsafe extern "C-unwind" fn counted_get_x(ud: *mut c_void, result: *mut c_void) {
    // Safety: 同 get_x_number 的派发契约（ud 存活 Vec2、result 帧内结果槽）。
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

    // Safety: l 为刚建存活状态，lua_l_openlibs 遵循 C 栈契约。
    unsafe {
      lua_l_openlibs(l);
    }

    reset_handler_hit_count();

    // Safety: l 存活；两个 NUL 结尾 c"X" 名串分别挂 kTagVec2/kTagOther 派发表，
    // handler 签名符合契约。
    unsafe {
      lua_registeruserdatadirectfieldget(l, K_TAG_VEC2, c"X".as_ptr(), Some(counted_get_x));
      lua_registeruserdatadirectfieldget(
        l,
        K_TAG_OTHER,
        c"X".as_ptr(),
        Some(counted_get_999_number),
      );
    }
    push_global(l, b"createVec2\0", create_vec_2);
    push_global(l, b"createOther\0", create_other_without_mt);

    let status = run_code(
      l,
      r#"
        return createVec2(3, 0).X, createOther().X
    "#,
    );
    assert_eq!(status, LuaStatus::Ok as i32);
    // Safety: l 存活，栈顶两值为返回；只读栈操作。
    unsafe {
      assert_eq!(lua_gettop(l), 2);

      assert_eq!(lua_tonumber!(l, -2), 3.0);
      assert_eq!(lua_tonumber!(l, -1), 999.0);
    }

    assert_eq!(handler_hit_count(), 2);
  }
}
