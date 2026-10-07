//! Port of `cpp/tests/DirectFieldAccess.test.cpp`（313 行，6 个 TEST_CASE）。
//!
//! 被测对象：`ulua_vm` 的 userdata 直接字段访问
//! （`lua_registeruserdatadirectfieldget` 及 VM 侧派发，对应 C++
//! `VM/src/lvmutils.cpp` / `Compiler` 的 `LuauDirectFieldGet` 支持）。
//!
//! 移植说明：
//! - C++ `handlerHitCount` 是文件级 `static int`；Rust 用 `AtomicI32` 镜像，
//!   并以 `Mutex` 让用到它的用例串行（同一进程内并行跑线程，与 C++ 单测试进程
//!   串行语义对齐）。
//! - C++ 的 `std::unique_ptr<LuaState, lua_close>` 镜像为 [`StateGuard`]：裸指针只
//!   活在守卫这一处，用例体一律经 `StateGuard::vm` 拿 `&mut LuaState` 走安全方法面。
//! - `Luau::compile` + `luau_load` 组合镜像为 safe `compile` + `luau_load`。
//! - C 回调（createVec2 / 直接字段 handler 等）按 conformance 同款拆为
//!   `extern "C-unwind"` 自由函数：它们的 `*mut LuaState` / `*mut c_void` 形参就是
//!   被测的 VM 派发契约面，故保留，只在函数体内一次性物化为引用再操作。

use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{NonNull, from_mut},
  sync::atomic::{AtomicI32, Ordering},
};
use std::sync::{Mutex, MutexGuard};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_common::functions::c_str::with_c_str;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_close::lua_close, lua_isnumber::lua_isnumber, lua_l_newstate::lua_l_newstate,
    lua_l_openlibs::lua_l_openlibs, lua_newuserdatatagged::lua_newuserdatatagged,
    lua_newuserdatataggedwithmetatable::lua_newuserdatataggedwithmetatable,
    lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
    lua_setuserdatametatable::lua_setuserdatametatable,
    lua_userdatadirectfield_setboolean::lua_userdatadirectfield_setboolean,
    lua_userdatadirectfield_setnumber::lua_userdatadirectfield_setnumber, luau_load::luau_load,
  },
  macros::lua_multret::LUA_MULTRET,
  records::lua_state::LuaState,
  type_aliases::lua_userdata_direct_field_get::LuaUserdataDirectFieldGet,
};

const K_TAG_VEC2: i32 = 42;
const K_TAG_OTHER: i32 = 43;

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

/// `std::unique_ptr<LuaState, void (*)(LuaState*)>` 的镜像：本文件唯一的
/// `LuaState` 裸指针持有者。
struct StateGuard(NonNull<LuaState>);

impl StateGuard {
  /// 测试环境内存充足，`luaL_newstate` 失败即环境失效，集中一处 expect
  fn new() -> Self {
    Self(NonNull::new(lua_l_newstate()).expect("lua state allocation failed"))
  }

  /// 唯一的指针→引用物化点：栈操作一律经此拿 `&mut LuaState` 后走安全方法面。
  fn vm(&mut self) -> &mut LuaState {
    // Safety: `0` 由 `NonNull::new` 判空后构造，指向 `luaL_newstate` 独占持有、
    // 存活至本守卫 drop 的状态；守卫既不 Copy 也不 Clone，故同一时刻至多一个由此
    // 方法产生的可变借用，且不越过 `&mut self` 的寿命。
    unsafe { self.0.as_mut() }
  }
}

impl Drop for StateGuard {
  fn drop(&mut self) {
    // Safety: self.0 为 NonNull（判空构造期已排除）且是本 guard 独占存活的
    // luaL_newstate 状态，落栈尾一次性关闭。
    unsafe {
      lua_close(self.0.as_ptr());
    }
  }
}

/// cpp `lua_pushcfunction`：按 C 名字把 `lua_CFunction` 压栈（callee 当场内化名字）。
fn push_c_function(
  l: &mut LuaState,
  name: &str,
  f: unsafe extern "C-unwind" fn(*mut LuaState) -> i32,
) {
  with_c_str(name.as_bytes(), |name_ptr| {
    // Safety: `l` 存活（调用方契约）；`name_ptr` 指向本次调用内有效的 NUL 结尾串，
    // 被调方立即内化；`f` 符合 `lua_CFunction` 契约。
    unsafe { l.push_c_function(Some(f), name_ptr) };
  });
}

/// cpp `lua_pushcfunction + lua_setglobal` 成对样板的收口。
fn push_global(l: &mut LuaState, name: &str, f: unsafe extern "C-unwind" fn(*mut LuaState) -> i32) {
  push_c_function(l, name, f);
  l.set_global_bytes(name.as_bytes());
}

/// cpp `lua_registeruserdatadirectfieldget` 的指针形参收口：注册表项按 C ABI 取
/// `*mut LuaState` 与 NUL 结尾字段名，其余调用点一律用引用。
fn register_direct_field_get(
  l: &mut LuaState,
  tag: i32,
  field: &str,
  handler: LuaUserdataDirectFieldGet,
) {
  with_c_str(field.as_bytes(), |field_ptr| {
    // Safety: `l` 存活且由本帧借用证明；`tag` 为测试常量的界内值；字段名当场被内化；
    // `handler` 是本文件内的 C ABI 回调，注册表项存活期内一直有效。
    unsafe { lua_registeruserdatadirectfieldget(from_mut(l), tag, field_ptr, handler) };
  });
}

/// cpp `runCode`：编译 + 载入 + `lua_pcall(0, LUA_MULTRET, 0)`。
fn run_code(l: &mut LuaState, source: &str) -> i32 {
  let bytecode = compile(
    source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );

  // Safety: `l` 借自存活状态（`from_mut` 只透传地址、不转移所有权）；chunkname 是
  // 字面量、`bytecode` 为本帧拥有的缓冲，载入即消化，无悬垂。
  let load_status = unsafe { luau_load(from_mut(l), "test", &bytecode, 0) };

  if load_status != 0 {
    return -1; // load failed
  }

  l.pcall(0, LUA_MULTRET, 0)
}

/// 直接字段派发 handler 的载荷读取：把 VM 交出的 `*mut c_void` 一次性物化为
/// `&Vec2`（载荷只读，故共享借用），此后全部走引用。
///
/// # Safety
/// `ud` 指向派发器保证存活且对齐的 `Vec2` 载荷，读借用不越过本次回调帧。
#[inline]
unsafe fn vec2_payload<'a>(ud: *mut c_void) -> &'a Vec2 {
  // Safety: 前置条件即 `ud` 非空、指向 `Vec2` 大小的可读载荷块且只读使用。
  unsafe { &*ud.cast::<Vec2>() }
}

/// 新建 tag 为 `K_TAG_VEC2` 的 userdata 并把载荷物化为 `&mut Vec2`：此后写字段
/// 全走引用，裸指针不出本函数。
///
/// # Safety
/// `l` 为 VM 传入的存活状态；返回借用的寿命由调用方（本次回调帧）覆盖。
unsafe fn vec2_userdata<'a>(l: *mut LuaState) -> &'a mut Vec2 {
  // Safety: `lua_newuserdatatagged` 依 C API 契约返回 `size_of::<Vec2>()` sized、
  // 对齐、本帧可写的载荷块（分配失败即长跳转/环境失效，与 cpp 同形）。
  unsafe { &mut *lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_VEC2).cast::<Vec2>() }
}

/// cpp `lua_createVec2`
unsafe extern "C-unwind" fn create_vec_2(l: *mut LuaState) -> i32 {
  // Safety: l 为 VM 传入的存活状态（`lua_CFunction` 回调契约），只在本帧内借用。
  let state = unsafe { &mut *l };
  let x = state.check_number(1);
  let y = state.check_number(2);

  // Safety: 同上，`l` 存活；返回的载荷块本帧可写。
  let vec = unsafe { vec2_userdata(l) };
  vec.x = x;
  vec.y = y;
  1
}

/// cpp `lua_createOtherWithMt`
unsafe extern "C-unwind" fn create_other_with_mt(l: *mut LuaState) -> i32 {
  // Safety: l 为 VM 传入的存活状态（回调契约），仅 C-API 压栈。
  unsafe {
    lua_newuserdatataggedwithmetatable(l, size_of::<Vec2>(), K_TAG_OTHER);
  }
  1
}

/// cpp `lua_createOtherWithoutMt`
unsafe extern "C-unwind" fn create_other_without_mt(l: *mut LuaState) -> i32 {
  // Safety: l 为 VM 传入的存活状态（回调契约），仅 C-API 压栈。
  unsafe {
    lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_OTHER);
  }
  1
}

/// cpp 用例 1 的 handler：`setnumber(res, ud->x)`
unsafe extern "C-unwind" fn get_x_number(ud: *mut c_void, result: *mut c_void) {
  // 被测对象就是「VM 交出的 userdata 载荷裸指针」，故裸形参保留、内部一次物化。
  let vec = unsafe { vec2_payload(ud) };
  // Safety: 直接字段派发契约——result 为本次调用的结果槽。
  unsafe { lua_userdatadirectfield_setnumber(result, vec.x) };
}

/// cpp 用例 5 的 handler：`setnumber(res, ud->y)`
unsafe extern "C-unwind" fn get_y_number(ud: *mut c_void, result: *mut c_void) {
  let vec = unsafe { vec2_payload(ud) };
  // Safety: 同 get_x_number 的派发契约。
  unsafe { lua_userdatadirectfield_setnumber(result, vec.y) };
}

/// cpp 用例 2 的 handler：`setboolean(res, ud->x != 0 || ud->y != 0)`
unsafe extern "C-unwind" fn get_non_zero_boolean(ud: *mut c_void, result: *mut c_void) {
  let vec = unsafe { vec2_payload(ud) };
  let non_zero = i32::from(vec.x != 0.0 || vec.y != 0.0);
  // Safety: 同 get_x_number 的派发契约。
  unsafe { lua_userdatadirectfield_setboolean(result, non_zero) };
}

/// cpp 用例 3 的 handler：计数 + `setnumber(res, ud->x)`
unsafe extern "C-unwind" fn counted_get_x_number(ud: *mut c_void, result: *mut c_void) {
  bump_handler_hit_count();
  let vec = unsafe { vec2_payload(ud) };
  // Safety: 同 get_x_number 的派发契约；计数走原子，与解引用无耦合。
  unsafe { lua_userdatadirectfield_setnumber(result, vec.x) };
}

/// cpp 用例 6 的 kTagOther handler：`setnumber(res, 999)` + 计数
unsafe extern "C-unwind" fn counted_get_999_number(_ud: *mut c_void, result: *mut c_void) {
  // Safety: 派发契约下 result 为帧内有效结果槽；不触碰 ud。
  unsafe { lua_userdatadirectfield_setnumber(result, 999.0) };
  bump_handler_hit_count();
}

/// cpp 用例 4 的 `__index`：恒返回 -1
unsafe extern "C-unwind" fn push_minus_one(l: *mut LuaState) -> i32 {
  // Safety: l 为 VM 传入的存活状态（回调契约），本帧只压入一个 number。
  unsafe { &mut *l }.push_number(-1.0);
  1
}

/// 栈顶值必须是 number 且等于 `expected`（cpp `REQUIRE(lua_isnumber(...)); CHECK_EQ(...)`）。
fn assert_top_number(l: &LuaState, expected: f64) {
  assert_ne!(lua_isnumber(l, -1), 0);
  assert_eq!(l.to_number(-1).unwrap_or(0.0), expected);
}

// Source: `tests/DirectFieldAccess.test.cpp:63-91`
#[test]
fn handler_setnumber_result() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let mut state = StateGuard::new();
  let l = state.vm();

  register_direct_field_get(l, K_TAG_VEC2, "X", Some(get_x_number));
  push_global(l, "createVec2", create_vec_2);

  let status = run_code(
    l,
    r#"
        local v = createVec2(3.5, 0)
        return v.X
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  assert_top_number(l, 3.5);
}

// Source: `tests/DirectFieldAccess.test.cpp:93-131`
#[test]
fn handler_setboolean_result() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let mut state = StateGuard::new();
  let l = state.vm();

  register_direct_field_get(l, K_TAG_VEC2, "NonZero", Some(get_non_zero_boolean));
  push_global(l, "createVec2", create_vec_2);

  let status = run_code(
    l,
    r#"
        local v = createVec2(1, 0)
        return v.NonZero
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  assert!(l.is_boolean(-1));
  assert!(l.to_boolean(-1));

  let status = run_code(
    l,
    r#"
        local v = createVec2(0, 0)
        return v.NonZero
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  assert!(l.is_boolean(-1));
  assert!(!l.to_boolean(-1));
}

// Source: `tests/DirectFieldAccess.test.cpp:133-168`
#[test]
fn repeated_access_handler_called_every_iteration() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _lock = lock_hit_count();
  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let mut state = StateGuard::new();
  let l = state.vm();

  reset_handler_hit_count();

  register_direct_field_get(l, K_TAG_VEC2, "X", Some(counted_get_x_number));
  push_global(l, "createVec2", create_vec_2);

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
  assert_top_number(l, 35.0);

  assert_eq!(handler_hit_count(), 5);
}

// Source: `tests/DirectFieldAccess.test.cpp:170-224`
#[test]
fn unregistered_tag_falls_through_to_index_metamethod() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _lock = lock_hit_count();
  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let mut state = StateGuard::new();
  let l = state.vm();

  lua_l_openlibs(l);
  reset_handler_hit_count();

  register_direct_field_get(l, K_TAG_VEC2, "X", Some(counted_get_x_number));

  // 给 kTagOther 挂元表，__index 对任意字段返回 -1
  l.new_metatable_by_str("metaOther");
  push_c_function(l, "__index", push_minus_one);
  l.set_field_str(-2, "__index");
  // Safety: `l` 存活（本帧借用）；tag 为界内常量；元表已在栈顶 -2 由上面建立。
  unsafe { lua_setuserdatametatable(from_mut(l), K_TAG_OTHER) };

  push_global(l, "createVec2", create_vec_2);
  push_global(l, "createOther", create_other_with_mt);

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
  assert_eq!(l.get_top(), 2);
  assert_eq!(l.to_number(-2).unwrap_or(0.0), 1.0); // 直接派发生效
  assert_eq!(l.to_number(-1).unwrap_or(0.0), -1.0); // kTagOther 无派发表，回落 __index

  assert_eq!(handler_hit_count(), 1); // handler 只命中 Vec2
}

// Source: `tests/DirectFieldAccess.test.cpp:226-264`
#[test]
fn multiple_fields_same_type_dispatch_independently() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let mut state = StateGuard::new();
  let l = state.vm();

  register_direct_field_get(l, K_TAG_VEC2, "X", Some(get_x_number));
  register_direct_field_get(l, K_TAG_VEC2, "Y", Some(get_y_number));
  push_global(l, "createVec2", create_vec_2);

  let status = run_code(
    l,
    r#"
        local v = createVec2(1.5, 2.5)
        return v.X, v.Y
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  assert_eq!(l.get_top(), 2);
  assert_eq!(l.to_number(-2).unwrap_or(0.0), 1.5);
  assert_eq!(l.to_number(-1).unwrap_or(0.0), 2.5);
}

mod same_field_name_different_tags_dispatch_independently {
  //! Source: `tests/DirectFieldAccess.test.cpp:266-311`

  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  use super::*;

  /// cpp 用例 6 的 kTagVec2 handler：`setnumber(res, ud->x)` + 计数
  unsafe extern "C-unwind" fn counted_get_x(ud: *mut c_void, result: *mut c_void) {
    let vec = unsafe { vec2_payload(ud) };
    // Safety: 同 get_x_number 的派发契约。
    unsafe { lua_userdatadirectfield_setnumber(result, vec.x) };
    bump_handler_hit_count();
  }

  #[test]
  fn same_field_name_different_tags_dispatch_independently() {
    let _lock = lock_hit_count();
    let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
    let mut state = StateGuard::new();
    let l = state.vm();

    lua_l_openlibs(l);
    reset_handler_hit_count();

    // 同名字段 "X" 分别挂到 kTagVec2 / kTagOther 两张派发表
    register_direct_field_get(l, K_TAG_VEC2, "X", Some(counted_get_x));
    register_direct_field_get(l, K_TAG_OTHER, "X", Some(counted_get_999_number));
    push_global(l, "createVec2", create_vec_2);
    push_global(l, "createOther", create_other_without_mt);

    let status = run_code(
      l,
      r#"
        return createVec2(3, 0).X, createOther().X
    "#,
    );
    assert_eq!(status, LuaStatus::Ok as i32);
    assert_eq!(l.get_top(), 2);
    assert_eq!(l.to_number(-2).unwrap_or(0.0), 3.0);
    assert_eq!(l.to_number(-1).unwrap_or(0.0), 999.0);

    assert_eq!(handler_hit_count(), 2);
  }
}
