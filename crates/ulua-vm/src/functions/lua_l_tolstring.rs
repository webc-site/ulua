//! Source: `VM/src/laux.cpp:616-679` (hand-ported)

use core::ffi::c_char;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    c_slice, cstr_cow,
    lua_encodepointer::lua_encodepointer,
    lua_l_callmeta::lua_l_callmeta,
    lua_l_typename::lua_l_typename,
    lua_pushfstring_l::lua_pushfstring_l,
    lua_pushlstring::lua_pushlstring,
    lua_pushstring::lua_pushstring,
    lua_pushvalue::lua_pushvalue,
    lua_toboolean::lua_toboolean,
    lua_tointeger_64::lua_tointeger_64,
    lua_tolstring::{c_str_out_param, lua_tolstring_ref},
    lua_tonumberx::lua_tonumberx,
    lua_topointer::lua_topointer,
    lua_tovector::lua_tovector,
    lua_type::lua_type,
    luai_int_2_str::luai_int2str,
    luai_num_2_str::luai_num2str_buf,
  },
  macros::{
    lua_l_error::luaL_error, lua_vector_size::LUA_VECTOR_SIZE, luai_maxint_2_str::LUAI_MAXINT2STR,
    luai_maxnum_2_str::LUAI_MAXNUM2STR,
  },
  records::lua_state::LuaState,
};

/// Rust 内部核心（§2/§3 出参收口）：cpp `luaL_tolstring`（`laux.cpp:612`）——按
/// `__tostring` 元方法/内建规则把 `idx` 槽值串化并**压栈**，返回结果串的全部字节
/// （含内嵌 `\0`，长度即切片长度）。
///
/// C 形 `size_t* len` 出参收口为 `Option<&'a [u8]>`：垫片 [`lua_l_tolstring`] 独家承接
/// 出参写入。栈语义与 cpp 逐字保持：结果串留在栈顶（由调用方弹出），`__tostring` 返回
/// 非串时经 `luaL_error` 抛错发散。
///
/// # Safety
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`，`idx` 为其合法栈索引；`__tostring`
/// 元方法回跑可改栈、可抛错，返回后不得继续持旧栈槽指针；返回切片指向压入栈顶的转换
/// 结果串内部，再次操作该栈前有效（寿命 `'a` 与 [`lua_tolstring_ref`] 同形）。
/// cpp laux.cpp:612。
pub unsafe fn lua_l_tolstring_ref<'a>(l: *mut LuaState, idx: i32) -> Option<&'a [u8]> {
  unsafe {
    if lua_l_callmeta(l, idx, c"__tostring".as_ptr()) != 0 {
      let s = lua_tolstring_ref(l, -1);
      if s.is_none() {
        luaL_error!(l, "'__tostring' must return a string");
      }
      return s;
    }

    match lua_type(l, idx) {
      x if x == LuaType::Nil as i32 => {
        lua_pushlstring(l, c"nil".as_ptr(), 3);
      }
      x if x == LuaType::Boolean as i32 => {
        lua_pushstring(
          l,
          if lua_toboolean(l, idx) != 0 {
            c"true".as_ptr()
          } else {
            c"false".as_ptr()
          },
        );
      }
      x if x == LuaType::Number as i32 => {
        // Number 类型槽必可转换；旧形忽略 isnum、失败时格式化 0.0，unwrap_or(0.0) 等价
        let n = lua_tonumberx(l, idx).unwrap_or(0.0);
        let mut s = [0 as c_char; LUAI_MAXNUM2STR as usize];
        let len = luai_num2str_buf(&mut s, n);
        lua_pushlstring(l, s.as_ptr(), len);
      }
      x if x == LuaType::Vector as i32 => {
        let v = lua_tovector(l, idx);
        let mut s = [0 as c_char; (LUAI_MAXNUM2STR as usize) * (LUA_VECTOR_SIZE as usize)];
        let mut pos = 0;
        for (i, &comp) in c_slice(v, LUA_VECTOR_SIZE as usize).iter().enumerate() {
          if i != 0 {
            s[pos] = b',' as c_char;
            s[pos + 1] = b' ' as c_char;
            pos += 2;
          }
          pos += luai_num2str_buf(&mut s[pos..], comp as f64);
        }
        lua_pushlstring(l, s.as_ptr(), pos);
      }
      x if x == LuaType::String as i32 => {
        lua_pushvalue(l, idx);
      }
      x if x == LuaType::Integer as i32 => {
        let val = lua_tointeger_64(l, idx);
        let mut s = [0 as c_char; LUAI_MAXINT2STR as usize];
        let len = luai_int2str(&mut s, val);
        lua_pushlstring(l, s.as_ptr(), len);
      }
      _ => {
        let ptr = lua_topointer(l, idx);
        let enc = lua_encodepointer(l, ptr as usize);
        let name = cstr_cow(lua_l_typename(l, idx));
        lua_pushfstring_l(l, format_args!("{}: 0x{:016x}", name, enc));
      }
    }

    lua_tolstring_ref(l, -1)
  }
}

/// C-ABI 适配垫片：把 [`lua_l_tolstring_ref`] 的切片折算回 lua.h 约定的
/// `(const char*, size_t* len)` 形态——`len` 非空时写出串长，`None` 折算回 NULL。
/// 仅供真实 C ABI 边界使用；Rust 调用方一律直接消费 [`lua_l_tolstring_ref`]。
///
/// # Safety
///
/// 同 [`lua_l_tolstring_ref`]；此外 `len` 必须为可写 `usize` 槽或 null（透传给
/// 出参写入）。
pub unsafe fn lua_l_tolstring(l: *mut LuaState, idx: i32, len: *mut usize) -> *const c_char {
  // Safety: 转发同契约的 `lua_l_tolstring_ref`；出参折算收口到共享的
  // `c_str_out_param`（返回值指向栈顶结果串的字节区，恒有终止 NUL，C 串语义不变）。
  unsafe { c_str_out_param(lua_l_tolstring_ref(l, idx), len) }
}
