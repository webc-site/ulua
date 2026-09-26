// userdata 标签、对齐与元表用例
// 移植自 `cpp/tests/Conformance.test.cpp`。
//
// C API 栈操作经 [`safe_api`] 门面收口（review.md §2），用例侧零 `unsafe`。

use core::ptr::null_mut;

#[test]
fn conformance_lightuserdata_api() {
  use core::ffi::c_void;

  use crate::common::functions::{new_state::new_state, safe_api};

  let global_state = new_state();
  let l = global_state.as_ptr();

  let value = 0x12345678usize as *mut c_void;

  safe_api::pushlightuserdatatagged(l, value, 1);
  assert_eq!(safe_api::lightuserdatatag(l, -1), 1);
  assert!(safe_api::tolightuserdatatagged(l, -1, 0).is_none());
  assert_eq!(safe_api::tolightuserdatatagged(l, -1, 1), Some(value));

  // tag 名表为全局注册，`b"id\0"` 为静态 NUL 结尾字节串。
  safe_api::setlightuserdataname(l, 1, b"id\0");
  assert!(safe_api::getlightuserdataname(l, 0).is_none());
  assert_eq!(safe_api::getlightuserdataname(l, 1), Some(&b"id"[..]));
  assert_eq!(safe_api::l_typename_bytes(l, -1), b"id");
  safe_api::pop(l, 1);

  // 不同 tag 的同值 lightuserdata 不应 raw 相等。
  safe_api::pushlightuserdatatagged(l, value, 0);
  safe_api::pushlightuserdatatagged(l, value, 1);
  assert_eq!(safe_api::rawequal(l, -1, -2), 0);
  safe_api::pop(l, 2);

  // 以两种 tag 的 lightuserdata 作表键写入（tag 参与键判等）。
  safe_api::createtable(l, 0, 0);

  safe_api::pushlightuserdatatagged(l, value, 2);
  safe_api::pushinteger(l, 20);
  safe_api::settable(l, -3);
  safe_api::pushlightuserdatatagged(l, value, 3);
  safe_api::pushinteger(l, 30);
  safe_api::settable(l, -3);

  safe_api::pushlightuserdatatagged(l, value, 2);
  safe_api::gettable(l, -2);
  safe_api::pushinteger(l, 20);
  assert_eq!(safe_api::rawequal(l, -1, -2), 1);
  safe_api::pop(l, 2);

  safe_api::pushlightuserdatatagged(l, value, 3);
  safe_api::gettable(l, -2);
  safe_api::pushinteger(l, 30);
  assert_eq!(safe_api::rawequal(l, -1, -2), 1);
  safe_api::pop(l, 2);
  safe_api::pop(l, 1);

  // tag 0 回落默认类型名，随后用 __type 元方法覆写。
  safe_api::pushlightuserdatatagged(l, value, 0);
  assert_eq!(safe_api::l_typename_bytes(l, -1), b"userdata");

  safe_api::createtable(l, 0, 1);
  safe_api::pushstr(l, b"luserdata\0");
  safe_api::setfield(l, -2, b"__type\0");
  assert_eq!(safe_api::setmetatable(l, -2), 1);

  // 覆写生效后按元表 __type 报告类型名。
  assert_eq!(safe_api::l_typename_bytes(l, -1), b"luserdata");
  safe_api::pop(l, 1);
}

#[test]
fn conformance_userdata() {
  use crate::common::functions::{
    conformance_userdata_setup::conformance_userdata_setup, run_conformance::run_fixture_setup,
  };

  run_fixture_setup("userdata.luau", conformance_userdata_setup);
}

#[test]
fn conformance_userdata_alignment() {
  use crate::common::{
    functions::{
      safe_api, userdata_alignment_alloc::userdata_alignment_alloc,
      userdata_alignment_dtor::userdata_alignment_dtor,
    },
    records::state_ref::StateRef,
  };

  let global_state =
    // FFI: c-API 要求 NULL
    StateRef::new(safe_api::newstate(Some(userdata_alignment_alloc), null_mut()))
      .expect("lua state allocation failed");
  let l = global_state.as_ptr();

  // allocf 已保证 16 字节对齐，每次 push 后即刻 pop 保持栈平衡（与 cpp 相同的 size
  // 交错节奏）。
  for size in (16..=4096).step_by(4) {
    for _ in 0..10 {
      let data = safe_api::newuserdata(l, size);
      assert_eq!((data as usize) % 16, 0);
      safe_api::pop(l, 1);
    }

    for _ in 0..10 {
      let data = safe_api::newuserdatadtor(l, size, Some(userdata_alignment_dtor));
      assert_eq!((data as usize) % 16, 0);
      safe_api::pop(l, 1);
    }
  }
}

#[test]
fn conformance_userdata_api() {
  use core::{ffi::c_void, sync::atomic::Ordering};

  use crate::common::functions::{
    new_state::new_state, safe_api, userdata_api_dtor_hits::USERDATA_API_DTOR_HITS,
    userdata_api_inline_char_dtor::userdata_api_inline_char_dtor,
    userdata_api_inline_int_dtor::userdata_api_inline_int_dtor,
    userdata_api_tag_dtor::userdata_api_tag_dtor,
  };

  USERDATA_API_DTOR_HITS.store(0, Ordering::SeqCst);

  let global_state = new_state();
  let l = global_state.as_ptr();

  // 各段是该状态机上配平的 C API 序列（cpp Conformance.test.cpp 同名用例），
  // 段间以栈内容/注册表状态的既有前后件衔接。
  let dtor_is_null = safe_api::getuserdatadtor(l, 42).is_none();
  assert!(dtor_is_null);
  safe_api::setuserdatadtor(l, 42, Some(userdata_api_tag_dtor));
  let dtor_is_set = safe_api::getuserdatadtor(l, 42).map(|dtor| dtor as *const ())
    == Some(userdata_api_tag_dtor as *const ());
  assert!(dtor_is_set);

  // `lud` 在本帧存活，lightuserdata 只被原样存取。
  let mut lud = 0i32;
  let lud_ptr = (&mut lud as *mut i32).cast::<c_void>();
  safe_api::pushlightuserdatatagged(l, lud_ptr, 0);

  assert_eq!(safe_api::tolightuserdata(l, -1), lud_ptr);
  assert_eq!(
    safe_api::touserdata(l, -1).map(|r| r as *mut c_void),
    Some(lud_ptr)
  );
  assert_eq!(safe_api::topointer(l, -1), lud_ptr.cast_const());

  // newuserdata 返回的 4 字节块按 i32 写入并即刻读取。
  let ud1 = safe_api::newuserdatatagged(l, 4, 0);
  safe_api::poke(ud1, 42i32);

  assert!(safe_api::tolightuserdata(l, -1).is_null());
  assert_eq!(
    safe_api::touserdata(l, -1).map(|r| r as *mut c_void),
    Some(ud1)
  );
  assert_eq!(safe_api::topointer(l, -1), ud1.cast_const());

  // tag 42 的 userdata 判等。
  let ud2 = safe_api::newuserdatatagged(l, 4, 42);
  safe_api::poke(ud2, -4i32);

  assert_eq!(safe_api::touserdatatagged(l, -1, 42), ud2);
  assert!(safe_api::touserdatatagged(l, -1, 41).is_null());
  assert_eq!(safe_api::userdatatag(l, -1), 42);

  // 改 tag 再复原。
  safe_api::setuserdatatag(l, -1, 43);
  assert_eq!(safe_api::userdatatag(l, -1), 43);
  safe_api::setuserdatatag(l, -1, 42);

  // int/char 两种尺寸的内联析构 userdata，指针写入紧跟分配。
  let ud3 = safe_api::newuserdatadtor(l, 4, Some(userdata_api_inline_int_dtor));
  let ud4 = safe_api::newuserdatadtor(l, 1, Some(userdata_api_inline_char_dtor));

  safe_api::poke(ud3, 43i32);
  safe_api::poke(ud4, 3i8);

  // udata1/udata2 元表经 setmetatable 绑定到两个零长 userdata。两张 metatable 各在
  // 栈底留一份（与 cpp 原序列一致），后续 -2/-1 判等依赖该布局。
  safe_api::newmetatable(l, b"udata1\0");
  safe_api::newmetatable(l, b"udata2\0");
  let ud5 = safe_api::newuserdatatagged(l, 0, 0);
  safe_api::l_getmetatable(l, b"udata1\0");
  safe_api::setmetatable(l, -2);
  let ud6 = safe_api::newuserdatatagged(l, 0, 0);
  safe_api::l_getmetatable(l, b"udata2\0");
  safe_api::setmetatable(l, -2);

  // 两个绑定各自按元表判等。
  assert_eq!(safe_api::l_checkudata(l, -2, "udata1"), ud5);
  assert_eq!(safe_api::l_checkudata(l, -1, "udata2"), ud6);

  // tag 50/51 注册全局元表。
  safe_api::newmetatable(l, b"udata3\0");
  safe_api::pushvalue(l, -1);
  safe_api::setuserdatametatable(l, 50);

  safe_api::newmetatable(l, b"udata4\0");
  safe_api::pushvalue(l, -1);
  safe_api::setuserdatametatable(l, 51);

  // udata3/udata4 判等收尾，随后 drop 触发内联析构计数。
  let ud7 = safe_api::newuserdatatagged(l, 16, 50);
  safe_api::getuserdatametatable(l, 50);
  safe_api::setmetatable(l, -2);

  let ud8 = safe_api::newuserdatataggedwithmetatable(l, 16, 51);

  assert_eq!(safe_api::l_checkudata(l, -2, "udata3"), ud7);
  assert_eq!(safe_api::l_checkudata(l, -1, "udata4"), ud8);

  drop(global_state);

  assert_eq!(USERDATA_API_DTOR_HITS.load(Ordering::SeqCst), 42);
}

#[test]
fn conformance_userdata_direct_access() {
  use ulua_common::fflag;
  use ulua_vm::records::lua_state::LuaState;

  use crate::common::{
    functions::{
      conformance_userdata_direct_access_setup::{
        conformance_userdata_direct_access_setup, conformance_userdata_direct_access_setup_mt,
      },
      get_or_create_atom::reset_direct_atom_state,
      run_conformance::run_fixture_setup,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_udata_direct_access = ScopedFastFlag::new(&fflag::LuauUdataDirectAccess6, true);

  // cpp `Conformance.test.cpp:4740-4775` 的两 SUBCASE：DirectAccess 注册 vec2/vertex
  // direct handler 直跑；ValidateMetatable 不注册 handler，靠 metatable 元方法对同一
  // fixture 得到一致结果。doctest 每 SUBCASE 从头重跑 TEST_CASE 体（含 atom 表重置），
  // 这里对应为每轮先 `reset_direct_atom_state()`。
  let setups = [
    conformance_userdata_direct_access_setup as unsafe extern "C-unwind" fn(*mut LuaState),
    conformance_userdata_direct_access_setup_mt,
  ];

  for setup in setups {
    reset_direct_atom_state();

    run_fixture_setup("udata_direct.luau", setup);
  }
}
