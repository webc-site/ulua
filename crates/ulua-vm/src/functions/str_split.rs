use core::slice::from_ref;

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

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：`l` 须处于可
/// 抛错受保护帧；栈槽 #1 为源串实参（非串经 `lua_l_checklstring_ref` 抛 "string expected"
/// 发散）、#2 为可选分隔符（none/nil 走缺省逗号，否则同为必检串实参）；结果表由
/// `lua_createtable` 建立、逐段经 `lua_pushlstring_bytes` + `lua_rawseti` 落表，压栈需栈顶
/// 余量，分配可触发 GC。
///
/// 源串/分隔符两个窗口须跨整段落表循环存活（cpp 的 `haystack`/`needle` 游标同形），循环内
/// 又须反复经 `l` 建表/压栈，p28 锚定形与 `&mut` 接收者不可共存 ⇒ 按 r16-v29 桥接判例在
/// 入口一次就地转手裸句柄（借用窗止于本次调用）。
///
/// # Safety
/// `unsafe fn` 屏障按 r16-v21 判例保留：本函数体承载一枚跨语句存活的裸句柄 `lp`（由 `&mut`
/// 接收者重取，存活与独占前提已由类型承载），块内一切取参/建表/压栈经它转手。
pub unsafe fn str_split(l: &mut LuaState) -> i32 {
  // SAFETY: `l` 由 `&mut` 保证有效且独占，转手后的 `lp` 即同一存活帧；两个切片均借用自栈槽
  // 串体（不可变、不搬移），本次调用内有效
  unsafe {
    let lp = l.as_mut_ptr();

    // 借用切片形态取源串/分隔符：出参 len 由切片长度承接，游走全程按下标，免指针算术
    let hay = lua_l_checklstring_ref(&mut *lp, 1);
    let nee = if (*lp).is_none_or_nil(2) {
      SEP_COMMA
    } else {
      lua_l_checklstring_ref(&mut *lp, 2)
    };
    let haystack_len = hay.len();
    let needle_len = nee.len();

    let mut span_start: usize = 0;
    let mut num_matches = 0;

    // cpp lstrlib.cpp:1089 LuauOptimizeStringSplit 优化路径
    if needle_len == 0 {
      // 空分隔符按单字符切分源串，结果表容量确定，一次性预分配
      lua_createtable(lp, haystack_len as i32, 0);

      for (idx, ch) in hay.iter().enumerate() {
        lua_pushlstring_bytes(&mut *lp, from_ref(ch));
        lua_rawseti(&mut *lp, -2, idx as i32 + 1);
      }

      1
    } else if needle_len == 1 {
      // 单字符分隔符预统计匹配次数，精准预分配结果表容量
      let sep = nee[0];
      let count = 1 + memchr::memchr_iter(sep, hay).count();
      lua_createtable(lp, count as i32, 0);

      for found in memchr::memchr_iter(sep, hay) {
        lua_pushlstring_bytes(&mut *lp, &hay[span_start..found]);
        num_matches += 1;
        lua_rawseti(&mut *lp, -2, num_matches);
        span_start = found + 1;
      }

      lua_pushlstring_bytes(&mut *lp, &hay[span_start..]);
      num_matches += 1;
      lua_rawseti(&mut *lp, -2, num_matches);

      1
    } else {
      lua_createtable(lp, 0, 0);

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
              lua_pushlstring_bytes(&mut *lp, &hay[span_start..iter]);
              num_matches += 1;
              lua_rawseti(&mut *lp, -2, num_matches);

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

      lua_pushlstring_bytes(&mut *lp, &hay[span_start..]);
      num_matches += 1;
      lua_rawseti(&mut *lp, -2, num_matches);

      1
    }
  }
}

lua_lib_fn!(pub fn str_split @ref, str_split_arm);
