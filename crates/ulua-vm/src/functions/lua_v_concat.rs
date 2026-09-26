//! Source: `VM/src/lvmutils.cpp:399-461` (hand-ported)

use core::{
  ffi::c_char,
  ptr::{copy_nonoverlapping, null_mut},
  slice::from_raw_parts,
};

use crate::{
  enums::{tms::TMS, value_view::ValueView},
  functions::{
    call_bin_tm::call_bin_tm, lua_g_concaterror::lua_g_concaterror,
    lua_s_buffinish::lua_s_buffinish, lua_s_bufstart::lua_s_bufstart, lua_s_newlstr::lua_s_newlstr,
  },
  macros::{
    getstr::getstr, lua_g_runerror::lua_g_runerror, maxssize::MAXSSIZE, setsvalue::setsvalue,
    tostring::tostring,
  },
  records::{lua_l_strbuf::LUA_BUFFERSIZE, lua_state::LuaState, t_string::tstring},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// 串轴的槽位读取（cpp `tsvalue(o)->len` / `svalue(o)` 对的视图形态）：`String` 变体
/// 直接带出 `tstring` 借用，长度与数据指针一并取出。
///
/// `None` 表示该槽非串——即原 `tsvalue!` 宏 `check_exp(ttisstring)` 的前提被破坏。本
/// 文件所有调用点都在 cpp `tostring!` 真值链之后，该态按 `tostring!` 契约（tag 判串，
/// 或 `lua_v_tostring` 把数值就地转串写回）不可达，落 `None` 只让它沿 cpp 的转换失败
/// 分支退出，不新增行为。借用随返回值活在消费点，不跨分配/TM 调用。
#[inline]
fn string_slot(t: &TValue) -> Option<&tstring> {
  match ValueView::from_tvalue(t) {
    ValueView::String(ts) => Some(ts),
    _ => None,
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_v_concat(l: *mut LuaState, mut total: i32, mut last: i32) {
  unsafe {
    loop {
      let top: StkId = (*l).base.add((last + 1) as usize);
      let first = top.sub(2);
      let second = top.sub(1);
      let mut n = 2; // number of elements handled in this pass (at least 2)

      // cpp `!(ttisstring(top-2) || ttisnumber(top-2)) || !tostring(top-1)`：首操作数
      // 的 tag 判据经 B1 视图（String/Number 之外的变体即元方法路径），`tostring!` 的
      // 「tag 判串或就地转串」语义保持宏形态
      let first_is_operand = matches!(
        ValueView::from_tvalue(&*first),
        ValueView::String(_) | ValueView::Number(_)
      );
      if !first_is_operand || !tostring!(l, second) {
        if call_bin_tm(l, first, second, first, TMS::TmConcat) == 0 {
          lua_g_concaterror(l, first, second);
        }
      } else if let Some(second_str) = string_slot(&*second) {
        // 第二操作数是空串？结果即首操作数（as string）
        if second_str.len == 0 {
          let _ = tostring!(l, first);
        } else {
          // at least two string values; get as many as possible
          let mut tl = second_str.len as usize;
          // collect total length
          n = 1;
          while n < total {
            let cur = top.sub((n + 1) as usize);
            if !tostring!(l, cur) {
              break;
            }
            let Some(slen) = string_slot(&*cur).map(|ts| ts.len as usize) else {
              break;
            };
            if slen > MAXSSIZE as usize - tl {
              lua_g_runerror!(l, "string length overflow");
            }
            tl += slen;
            n += 1;
          }

          let mut buf = [0 as c_char; LUA_BUFFERSIZE];
          let mut ts: *mut tstring = null_mut();

          let buffer: *mut c_char = if tl < LUA_BUFFERSIZE {
            buf.as_mut_ptr()
          } else {
            ts = lua_s_bufstart(l, tl);
            (*ts).data.as_mut_ptr()
          };

          // concat all strings
          tl = 0;
          for i in (1..=n).rev() {
            // 本区间槽位全部由上面的 `tostring!` 真值链保证为串槽
            let cur = top.sub(i as usize);
            let Some(src) = string_slot(&*cur) else {
              continue;
            };
            let slen = src.len as usize;
            copy_nonoverlapping(getstr(src as *const tstring), buffer.add(tl), slen);
            tl += slen;
          }

          if tl < LUA_BUFFERSIZE {
            // Safety: buffer 指向本地栈数组 buf 或刚分配的 TString data 区，
            // tl 为已拷入的总长（本分支 tl < LUA_BUFFERSIZE 且各源串 len 之和），界内可读
            setsvalue!(
              l,
              top.sub(n as usize),
              lua_s_newlstr(l, from_raw_parts(buffer.cast::<u8>(), tl))
            );
          } else {
            setsvalue!(l, top.sub(n as usize), lua_s_buffinish(l, ts));
          }
        }
      }
      total -= n - 1; // got `n` strings to create 1 new
      last -= n - 1;
      if total <= 1 {
        break; // repeat until only 1 result left
      }
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn lua_v_concat_export(l: *mut LuaState, total: i32, last: i32) {
  unsafe {
    lua_v_concat(l, total, last);
  }
}
