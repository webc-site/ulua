use core::slice::from_raw_parts;

use crate::{
  functions::{
    lua_createtable::lua_createtable, lua_l_checklstring::lua_l_checklstring,
    lua_pushinteger::lua_pushinteger, lua_pushlstring::lua_pushlstring, lua_settable::lua_settable,
  },
  macros::{lua_isnoneornil::lua_isnoneornil, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// 缺省分隔符字节切片（逗号，纯 Rust 切片不再使用 NUL 结尾）。
const SEP_COMMA: &[u8] = b",";

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn str_split(l: *mut LuaState) -> i32 {
  unsafe {
    let mut haystack_len: usize = 0;
    let haystack = lua_l_checklstring(l, 1, &mut haystack_len);
    let mut needle_len: usize = 0;
    let nee = if lua_isnoneornil!(l, 2) {
      needle_len = SEP_COMMA.len();
      SEP_COMMA
    } else {
      let needle = lua_l_checklstring(l, 2, &mut needle_len);
      from_raw_parts(needle as *const u8, needle_len)
    };

    // Safety: 契约保证两串 payload 字节区可读（needle 为空时得 0 长切片）
    let hay = from_raw_parts(haystack as *const u8, haystack_len);

    let mut span_start: usize = 0;
    let mut num_matches = 0;

    lua_createtable(l, 0, 0);

    // C++ `iter = begin; if (needleLen == 0) iter++`；游标改源串相对下标
    let mut iter = usize::from(needle_len == 0);
    // Don't iterate the last needleLen - 1 bytes of the string - they are
    // impossible to be splits and would let us compare past the end of the
    // buffer.（C++ `iter <= end - needleLen` 的同界判定）
    while iter + needle_len <= haystack_len {
      // u8 切片比较与 C memcmp 语义一致，且允许嵌入 null 字节
      if &hay[iter..iter + needle_len] == nee {
        num_matches += 1;
        lua_pushinteger(l, num_matches);
        // Safety: span_start ≤ iter ≤ haystack_len，add 落点为源串界内或串尾（pushlstring 按 len 消费）
        lua_pushlstring(l, haystack.add(span_start), iter - span_start);
        lua_settable(l, -3);

        span_start = iter + needle_len;
        if needle_len > 0 {
          iter += needle_len - 1;
        }
      }
      iter += 1;
    }

    if needle_len > 0 {
      num_matches += 1;
      lua_pushinteger(l, num_matches);
      // Safety: span_start ≤ haystack_len（仅 needle_len>0 时推进过界内命中位）
      lua_pushlstring(l, haystack.add(span_start), haystack_len - span_start);
      lua_settable(l, -3);
    }

    1
  }
}

lua_lib_fn!(pub fn str_split, str_split_arm);
