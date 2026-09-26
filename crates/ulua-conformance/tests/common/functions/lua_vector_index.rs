use core::{ffi::c_int, slice};

use ulua_vm::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    lua_pushcclosurek::lua_pushcclosurek,
    lua_pushnumber::lua_pushnumber,
    lua_pushvector_lapi::{
      lua_pushvector_lua_state_f32_f32_f32, lua_pushvector_lua_state_f32_f32_f32_f32,
    },
  },
  macros::{
    lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error, lua_vector_size::LUA_VECTOR_SIZE,
  },
  records::lua_state::LuaState,
};

use crate::common::functions::{cstr::cstr, cstr_text::cstr_text, lua_vector_dot::lua_vector_dot};

/// 分量平方和——保持 cpp 的固定累加次序（v0²+v1²+v2²，四维再 +v3²），
/// 浮点结合序与 oracle 逐位一致。
fn magnitude_sq_sum(v: &[f32]) -> f32 {
  let mut sum = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
  if LUA_VECTOR_SIZE == 4 {
    sum += v[3] * v[3];
  }
  sum
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vector_index(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造且活跃，
  // `lua_l_checkvector` 保证返回指针指向可读 `LUA_VECTOR_SIZE` 个 f32 的向量。
  let v = unsafe {
    let v = lua_l_checkvector(l, 1);
    slice::from_raw_parts(v, LUA_VECTOR_SIZE as usize)
  };

  // Safety: `l` 活跃（同上），checkstring 返回本帧内可读的 NUL 结尾串指针。
  let name_ptr = unsafe { luaL_checkstring!(l, 2) };
  // Safety: 同上，`name_ptr` 为 NUL 结尾 C 字符串。
  let name = unsafe { cstr_text(name_ptr) };

  if name == "Magnitude" {
    let sum = magnitude_sq_sum(v);
    // Safety: `l` 活跃，pushnumber 无额外前置条件。
    unsafe { lua_pushnumber(l, sum.sqrt() as f64) };
    return 1;
  }

  if name == "Unit" {
    let inv_sqrt = 1.0 / magnitude_sq_sum(v).sqrt();

    // Safety: `l` 活跃；向量压入变体由 LUA_VECTOR_SIZE 编译期配置决定，
    // 下方按维数选取的重载与 cpp 分支一致。
    if LUA_VECTOR_SIZE == 4 {
      unsafe {
        lua_pushvector_lua_state_f32_f32_f32_f32(
          l,
          v[0] * inv_sqrt,
          v[1] * inv_sqrt,
          v[2] * inv_sqrt,
          v[3] * inv_sqrt,
        )
      };
    } else {
      unsafe {
        lua_pushvector_lua_state_f32_f32_f32(l, v[0] * inv_sqrt, v[1] * inv_sqrt, v[2] * inv_sqrt)
      };
    }
    return 1;
  }

  if name == "Dot" {
    // Safety: `l` 活跃；`b"Dot\0"` 为静态 NUL 结尾字节串，闭包/续体签名匹配 C ABI。
    unsafe { lua_pushcclosurek(l, Some(lua_vector_dot), cstr(b"Dot\0"), 0, None) };
    return 1;
  }

  // Safety: `l` 活跃；`luaL_error` 以 long-jump 终止本回调，`name` 为安全引用。
  unsafe { luaL_error!(l, "{name} is not a valid member of vector") }
}
