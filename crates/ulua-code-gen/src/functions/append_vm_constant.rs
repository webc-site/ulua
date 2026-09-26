use ulua_vm::enums::lua_type::LuaType;

extern crate alloc;

use alloc::string::String;
use core::slice::from_raw_parts;

use ulua_common::functions::format_g::{format_g, format_g_append_vector};
use ulua_vm::{macros::lua_vector_size::LUA_VECTOR_SIZE, records::proto::Proto};

use crate::functions::{
  append::append, is_printable_string_constant::is_printable_string_constant, proto_views,
};

// Lua 值类型 tag（lobject.h）
const LUA_TNIL: i32 = LuaType::Nil as i32;
const LUA_TBOOLEAN: i32 = LuaType::Boolean as i32;
const LUA_TNUMBER: i32 = LuaType::Number as i32;
const LUA_TINTEGER: i32 = LuaType::Integer as i32;
const LUA_TVECTOR: i32 = LuaType::Vector as i32;
const LUA_TSTRING: i32 = LuaType::String as i32;

const K_MAX_STRING_CONSTANT_PRINT_LENGTH: usize = 16;

/// dump 常量表第 `index` 项。
///
/// `index` 为负或越界时不输出任何内容（cpp 此处是越界读，取安全方向，与
/// [`proto_views::string_constant`] 的折叠一致）。
pub fn append_vm_constant(result: &mut String, proto: &Proto, index: i32) {
  // 常量表经 `proto_views::constants` 的安全切片视图读取（review.md §2）。
  let Some(position) = usize::try_from(index).ok() else {
    return;
  };
  let Some(constant) = proto_views::constants(proto).get(position) else {
    return;
  };

  if constant.tt == LUA_TNIL {
    append(result, format_args!("nil"));
  } else if constant.tt == LUA_TBOOLEAN {
    // Safety: tt 判据一致下读 union 布尔位，同址标量副本拷贝。
    let is_true = unsafe { constant.value.b != 0 };
    append(
      result,
      format_args!("{}", if is_true { "true" } else { "false" }),
    );
  } else if constant.tt == LUA_TNUMBER {
    // Safety: tt 判据一致下读 union 浮点位，同址标量副本拷贝。
    let n = unsafe { constant.value.n };
    if n.is_nan() {
      append(result, format_args!("nan"));
    } else {
      // C++ 用 "%.17g"；Rust 的 `{}` 打印最短 round-trip 形式。
      result.push_str(&format_g(n, 17));
    }
  } else if constant.tt == LUA_TINTEGER {
    // Safety: tt 判据一致下读 union 整数位，同址标量副本拷贝。
    let int_val = unsafe { constant.value.l };
    append(result, format_args!("{}i", int_val));
  } else if constant.tt == LUA_TSTRING {
    // 字符串载荷经 `proto_views::string_constant` 读取（内部即 cpp 的
    // `getstr(tsvalue(&proto->k[index]))` + `len`），越界折叠为空切片。
    let bytes = proto_views::string_constant(proto, position);

    if is_printable_string_constant(bytes) {
      let n = bytes.len().min(K_MAX_STRING_CONSTANT_PRINT_LENGTH);
      let text = String::from_utf8_lossy(&bytes[..n]);

      if bytes.len() < K_MAX_STRING_CONSTANT_PRINT_LENGTH {
        append(result, format_args!("'{}'", text));
      } else {
        append(result, format_args!("'{}'...", text));
      }
    }
  } else if constant.tt == LUA_TVECTOR {
    // value.v 是 union 中的 float[2]；[2]/[3] 索引尾部的
    // TValue 存储，对应 C++ 的 `const float* v = constant.value.v`。
    // Safety: 把本地 constant 的 TValue 存储视作 f32 数组，读取长度受 LUA_VECTOR_SIZE
    // 夹住（==4 读到 4 个，否则 3 个），均为同址只读 reinterpret。
    let lanes = if LUA_VECTOR_SIZE == 4 { 4 } else { 3 };
    let v = unsafe {
      let base = &constant.value as *const _ as *const f32;
      from_raw_parts(base, lanes)
    };

    // 3/4 分量出口统一交 `format_g_append_vector`（`%.9g`、`v[3] != 0` 判四
    // 分量，3-lane 构建切片长 3 恒三分量）——与原 `if LUA_VECTOR_SIZE == 4`
    // 双分支逐字节一致。
    let comps = [
      f64::from(v[0]),
      f64::from(v[1]),
      f64::from(v[2]),
      v.get(3).copied().map_or(0.0, f64::from),
    ];
    format_g_append_vector(result, &comps[..lanes], |x| format_g(x, 9));
  }
}
