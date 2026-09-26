// userdata 直接字段访问 handler 用例
// 移植自 cpp/tests/DirectFieldAccess.test.cpp（Luau.Conformance 目标，见 cpp/Sources.cmake:602-614）。
// 该套件里只有 `UserdataDirectAccess`（Conformance.test.cpp:4655）引用 udata_direct.luau。
//
// C API 栈读写经 [`safe_api`] 门面；run_code / push_cfunction_global 为 crate 内既有单次边界。

use crate::common::records::direct_field_access_handler_hit_count::DIRECT_FIELD_ACCESS_TEST_MUTEX;

#[test]
fn direct_field_access_handler_setboolean_result() {
  use ulua_common::fflag;
  use ulua_vm::enums::lua_status::LuaStatus;

  use crate::common::{
    functions::{
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_get_non_zero_boolean::direct_field_access_get_non_zero_boolean,
      direct_field_access_k_tag_vec_2::K_TAG_VEC2, new_state::new_state,
      push_cfunction_global::push_cfunction_global, run_code::run_code, safe_api,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = new_state();
  let l = state.as_ptr();

  safe_api::register_direct_field_get(
    l,
    K_TAG_VEC2,
    b"NonZero\0",
    Some(direct_field_access_get_non_zero_boolean),
  );
  push_cfunction_global(l, Some(direct_field_access_create_vec_2), b"createVec2\0");

  let status = run_code(
    l,
    r#"
        local v = createVec2(1, 0)
        return v.NonZero
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  assert!(safe_api::isboolean(l, -1));
  assert_eq!(safe_api::toboolean(l, -1), 1);

  let status = run_code(
    l,
    r#"
        local v = createVec2(0, 0)
        return v.NonZero
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  assert!(safe_api::isboolean(l, -1));
  assert_eq!(safe_api::toboolean(l, -1), 0);
}

#[test]
fn direct_field_access_handler_setnumber_result() {
  use ulua_common::fflag;
  use ulua_vm::enums::lua_status::LuaStatus;

  use crate::common::{
    functions::{
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_get_x_number::direct_field_access_get_x_number,
      direct_field_access_k_tag_vec_2::K_TAG_VEC2, new_state::new_state,
      push_cfunction_global::push_cfunction_global, run_code::run_code, safe_api,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = new_state();
  let l = state.as_ptr();

  safe_api::register_direct_field_get(
    l,
    K_TAG_VEC2,
    b"X\0",
    Some(direct_field_access_get_x_number),
  );
  push_cfunction_global(l, Some(direct_field_access_create_vec_2), b"createVec2\0");

  let status = run_code(
    l,
    r#"
        local v = createVec2(3.5, 0)
        return v.X
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);

  assert_ne!(safe_api::isnumber(l, -1), 0);
  assert_eq!(safe_api::tonumber(l, -1), 3.5);
}

#[test]
fn direct_field_access_multiple_fields_same_type_dispatch_independently() {
  use ulua_common::fflag;
  use ulua_vm::enums::lua_status::LuaStatus;

  use crate::common::{
    functions::{
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_get_x_number::direct_field_access_get_x_number,
      direct_field_access_get_y_number::direct_field_access_get_y_number,
      direct_field_access_k_tag_vec_2::K_TAG_VEC2, new_state::new_state,
      push_cfunction_global::push_cfunction_global, run_code::run_code, safe_api,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = new_state();
  let l = state.as_ptr();

  safe_api::register_direct_field_get(
    l,
    K_TAG_VEC2,
    b"X\0",
    Some(direct_field_access_get_x_number),
  );
  safe_api::register_direct_field_get(
    l,
    K_TAG_VEC2,
    b"Y\0",
    Some(direct_field_access_get_y_number),
  );
  push_cfunction_global(l, Some(direct_field_access_create_vec_2), b"createVec2\0");

  let status = run_code(
    l,
    r#"
        local v = createVec2(1.5, 2.5)
        return v.X, v.Y
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  assert_eq!(safe_api::gettop(l), 2);

  assert_eq!(safe_api::tonumber(l, -2), 1.5);
  assert_eq!(safe_api::tonumber(l, -1), 2.5);
}

#[test]
fn direct_field_access_repeated_access_handler_called_every_iteration() {
  use ulua_common::fflag;
  use ulua_vm::enums::lua_status::LuaStatus;

  use crate::common::{
    functions::{
      direct_field_access_counted_get_x_number::direct_field_access_counted_get_x_number,
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_handler_hit_count::direct_field_access_handler_hit_count,
      direct_field_access_k_tag_vec_2::K_TAG_VEC2,
      direct_field_access_reset_handler_hit_count::direct_field_access_reset_handler_hit_count,
      new_state::new_state, push_cfunction_global::push_cfunction_global, run_code::run_code,
      safe_api,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _lock = DIRECT_FIELD_ACCESS_TEST_MUTEX.lock().unwrap();
  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = new_state();
  let l = state.as_ptr();

  direct_field_access_reset_handler_hit_count();

  safe_api::register_direct_field_get(
    l,
    K_TAG_VEC2,
    b"X\0",
    Some(direct_field_access_counted_get_x_number),
  );
  push_cfunction_global(l, Some(direct_field_access_create_vec_2), b"createVec2\0");

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
  assert_ne!(safe_api::isnumber(l, -1), 0);
  assert_eq!(safe_api::tonumber(l, -1), 35.0);

  assert_eq!(direct_field_access_handler_hit_count(), 5);
}

#[test]
fn direct_field_access_same_field_name_different_tags_dispatch_independently() {
  use ulua_common::fflag;
  use ulua_vm::enums::lua_status::LuaStatus;

  use crate::common::{
    functions::{
      direct_field_access_counted_get_999_number::direct_field_access_counted_get_999_number,
      direct_field_access_counted_get_x_number::direct_field_access_counted_get_x_number,
      direct_field_access_create_other_without_mt::direct_field_access_create_other_without_mt,
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_handler_hit_count::direct_field_access_handler_hit_count,
      direct_field_access_k_tag_other::K_TAG_OTHER, direct_field_access_k_tag_vec_2::K_TAG_VEC2,
      direct_field_access_reset_handler_hit_count::direct_field_access_reset_handler_hit_count,
      new_state::new_state, push_cfunction_global::push_cfunction_global, run_code::run_code,
      safe_api,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _lock = DIRECT_FIELD_ACCESS_TEST_MUTEX.lock().unwrap();
  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = new_state();
  let l = state.as_ptr();

  direct_field_access_reset_handler_hit_count();

  safe_api::openlibs(l);
  safe_api::register_direct_field_get(
    l,
    K_TAG_VEC2,
    b"X\0",
    Some(direct_field_access_counted_get_x_number),
  );
  safe_api::register_direct_field_get(
    l,
    K_TAG_OTHER,
    b"X\0",
    Some(direct_field_access_counted_get_999_number),
  );

  push_cfunction_global(l, Some(direct_field_access_create_vec_2), b"createVec2\0");
  push_cfunction_global(
    l,
    Some(direct_field_access_create_other_without_mt),
    b"createOther\0",
  );

  let status = run_code(
    l,
    r#"
        return createVec2(3, 0).X, createOther().X
    "#,
  );
  assert_eq!(status, LuaStatus::Ok as i32);
  assert_eq!(safe_api::gettop(l), 2);

  assert_eq!(safe_api::tonumber(l, -2), 3.0);
  assert_eq!(safe_api::tonumber(l, -1), 999.0);

  assert_eq!(direct_field_access_handler_hit_count(), 2);
}

#[test]
fn direct_field_access_unregistered_tag_falls_through_to_index_metamethod() {
  use ulua_common::fflag;
  use ulua_vm::enums::lua_status::LuaStatus;

  use crate::common::{
    functions::{
      direct_field_access_counted_get_x_number::direct_field_access_counted_get_x_number,
      direct_field_access_create_other_with_mt::direct_field_access_create_other_with_mt,
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_handler_hit_count::direct_field_access_handler_hit_count,
      direct_field_access_k_tag_other::K_TAG_OTHER, direct_field_access_k_tag_vec_2::K_TAG_VEC2,
      direct_field_access_push_minus_one::direct_field_access_push_minus_one,
      direct_field_access_reset_handler_hit_count::direct_field_access_reset_handler_hit_count,
      new_state::new_state, push_cfunction_global::push_cfunction_global, run_code::run_code,
      safe_api,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _lock = DIRECT_FIELD_ACCESS_TEST_MUTEX.lock().unwrap();
  let _sff = ScopedFastFlag::new(&fflag::LuauDirectFieldGet, true);
  let state = new_state();
  let l = state.as_ptr();

  direct_field_access_reset_handler_hit_count();

  safe_api::openlibs(l);
  safe_api::register_direct_field_get(
    l,
    K_TAG_VEC2,
    b"X\0",
    Some(direct_field_access_counted_get_x_number),
  );

  // __index 装载入元方法表（setfield 目标非全局），末句把元表挂到 K_TAG_OTHER。
  safe_api::newmetatable(l, b"metaOther\0");
  safe_api::pushcfunction_named(l, Some(direct_field_access_push_minus_one), b"__index\0");
  safe_api::setfield(l, -2, b"__index\0");
  safe_api::setuserdatametatable(l, K_TAG_OTHER);

  push_cfunction_global(l, Some(direct_field_access_create_vec_2), b"createVec2\0");
  push_cfunction_global(
    l,
    Some(direct_field_access_create_other_with_mt),
    b"createOther\0",
  );

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
  assert_eq!(safe_api::gettop(l), 2);

  assert_eq!(safe_api::tonumber(l, -2), 1.0);
  assert_eq!(safe_api::tonumber(l, -1), -1.0);

  assert_eq!(direct_field_access_handler_hit_count(), 1);
}
