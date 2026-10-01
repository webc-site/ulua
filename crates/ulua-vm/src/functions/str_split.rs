use crate::{
  functions::{
    lua_createtable::lua_createtable, lua_l_checklstring::lua_l_checklstring_ref,
    lua_pushlstring::lua_pushlstring_bytes, lua_rawseti::lua_rawseti,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 缺省分隔符字节切片（逗号，纯 Rust 切片不再使用 NUL 结尾）。
const SEP_COMMA: &[u8] = b",";

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn str_split(l: *mut LuaState) -> i32 {
  unsafe {
    // 借用切片形态取源串/分隔符：出参 len 由切片长度承接，游走全程按下标，免指针算术
    let hay = lua_l_checklstring_ref(&mut *l, 1);
    let nee = if (*l).is_none_or_nil(2) {
      SEP_COMMA
    } else {
      lua_l_checklstring_ref(&mut *l, 2)
    };
    let haystack_len = hay.len();
    let needle_len = nee.len();

    let mut span_start: usize = 0;
    let mut num_matches = 0;

    // cpp lstrlib.cpp:1089 LuauOptimizeStringSplit 优化路径
    if needle_len == 0 {
      // 空分隔符按单字符切分源串，结果表容量确定，一次性预分配
      lua_createtable(l, haystack_len as i32, 0);

      for (idx, ch) in hay.iter().enumerate() {
        lua_pushlstring_bytes(l, core::slice::from_ref(ch));
        lua_rawseti(&mut *l, -2, idx as i32 + 1);
      }

      1
    } else if needle_len == 1 {
      // 单字符分隔符预统计匹配次数，精准预分配结果表容量
      let sep = nee[0];
      let count = 1 + memchr::memchr_iter(sep, hay).count();
      lua_createtable(l, count as i32, 0);

      for found in memchr::memchr_iter(sep, hay) {
        lua_pushlstring_bytes(l, &hay[span_start..found]);
        num_matches += 1;
        lua_rawseti(&mut *l, -2, num_matches);
        span_start = found + 1;
      }

      lua_pushlstring_bytes(l, &hay[span_start..]);
      num_matches += 1;
      lua_rawseti(&mut *l, -2, num_matches);

      1
    } else {
      lua_createtable(l, 0, 0);

      if needle_len <= haystack_len {
        let last = haystack_len - needle_len;
        let mut iter = 0;
        let first = nee[0];
        let last_ch = nee[needle_len - 1];

        while iter <= last {
          // 利用 memchr 向量化跳跃首字符，并内联检查末字符与完整切片
          if let Some(offset) = memchr::memchr(first, &hay[iter..=last]) {
            iter += offset;
            let cand = &hay[iter..iter + needle_len];
            if cand[needle_len - 1] == last_ch && cand == nee {
              lua_pushlstring_bytes(l, &hay[span_start..iter]);
              num_matches += 1;
              lua_rawseti(&mut *l, -2, num_matches);

              span_start = iter + needle_len;
              iter = span_start;
            } else {
              iter += 1;
            }
          } else {
            break;
          }
        }
      }

      lua_pushlstring_bytes(l, &hay[span_start..]);
      num_matches += 1;
      lua_rawseti(&mut *l, -2, num_matches);

      1
    }
  }
}

lua_lib_fn!(pub fn str_split, str_split_arm);
