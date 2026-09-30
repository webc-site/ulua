use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_callbacks::lua_callbacks,
    lua_registeruserdatadirectaccess::lua_registeruserdatadirectaccess,
  },
  records::lua_state::LuaState,
};

use crate::common::{
  functions::{
    conformance_userdata_direct_access_useratom::conformance_userdata_direct_access_useratom,
    setup_userdata_helpers::setup_userdata_helpers, setup_vector_helpers::setup_vector_helpers,
    vec_2_direct_index::vec_2_direct_index, vec_2_direct_namecall::vec_2_direct_namecall,
    vec_2_direct_newindex::vec_2_direct_newindex, vertex_direct_index::vertex_direct_index,
    vertex_direct_namecall::vertex_direct_namecall, vertex_direct_newindex::vertex_direct_newindex,
  },
  records::userdata_tags::{K_TAG_VEC2, K_TAG_VERTEX},
};

/// cpp `Conformance.test.cpp:4749-4755` 两个 SUBCASE 共用的安装段：useratom 钩子 +
/// `setupVectorHelpers` + `setupUserdataHelpers`。
///
/// # Safety
///
/// `l` 为本用例存活的 LuaState。
unsafe fn udata_direct_common(l: *mut LuaState) {
  // Safety: `lua_callbacks` 返回该状态持有的回调表指针，
  // useratom 是 `extern "C-unwind"` 桩。
  unsafe { (*lua_callbacks(l)).useratom = Some(conformance_userdata_direct_access_useratom) };

  // Safety: 两个 setup 门面只接受存活 `l`，各自安装 metatable 与构造函数（并行
  // 测试下 `l` 由本用例独占，无并发访问）。
  unsafe {
    setup_vector_helpers(l);
    setup_userdata_helpers(l);
  }
}

/// cpp `Conformance.test.cpp:4762-4768` 的 SUBCASE("DirectAccess")：在共用安装段
/// 之上注册 vec2 与 vertex 两组 direct access handler。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_userdata_direct_access_setup(l: *mut LuaState) {
  // Safety: `l` 为本用例存活的 LuaState（共用段契约）。
  unsafe { udata_direct_common(l) };

  // Safety: `l` 存活；K_TAG_VEC2 是已登记的 userdata tag，三个 direct 钩子均为
  // `extern "C-unwind"` 桩，返回 1 表示注册成功。
  let vec2_ok = unsafe {
    lua_registeruserdatadirectaccess(
      l,
      K_TAG_VEC2 as c_int,
      Some(vec_2_direct_index),
      Some(vec_2_direct_newindex),
      Some(vec_2_direct_namecall),
    )
  };
  assert_eq!(vec2_ok, 1);

  // Safety: 同上——K_TAG_VERTEX 为已登记 tag，三个 vertex direct 钩子同为
  // `extern "C-unwind"` 桩。
  let vertex_ok = unsafe {
    lua_registeruserdatadirectaccess(
      l,
      K_TAG_VERTEX as c_int,
      Some(vertex_direct_index),
      Some(vertex_direct_newindex),
      Some(vertex_direct_namecall),
    )
  };
  assert_eq!(vertex_ok, 1);
}

/// cpp `Conformance.test.cpp:4770-4773` 的 SUBCASE("ValidateMetatable")：共用安装
/// 段之后**不**注册任何 direct handler——同一 fixture 的全部访问必须经 metatable
/// 元方法得到与 direct dispatch 相同的结果。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_userdata_direct_access_setup_mt(l: *mut LuaState) {
  // Safety: `l` 为本用例存活的 LuaState（共用段契约）。
  unsafe { udata_direct_common(l) };
}
