//! Source: `VM/src/lstrlib.cpp:831`
//!
//! `string.gsub` — global substitution. Repeatedly match the pattern against the
//! source (up to `max_s` times), append each replacement via `add_value` and the
//! intervening literal text, then push the result string and the substitution
//! count. Honors a leading `^` anchor (single attempt).

use crate::{
  enums::lua_type::LuaType,
  functions::{
    add_value::add_value, lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit, lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_optinteger::lua_l_optinteger, lua_l_pushresult::lua_l_pushresult, r#match::match_item,
    prepstate::prepstate, reprepstate::reprepstate,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState, match_state::MatchState},
};

/// # Safety
/// `l` 必须是正在执行的 `string.gsub` C 函数帧的存活 `LuaState`：栈槽 #1/#2 为源串/
/// pattern（`lua_l_checklstring_ref` 的借用切片在本次调用全程存活）、#3 为替换值
/// （`arg_expected` 闸门后为 string/number/function/table）、#4 为替换上限；
/// 累加器 `b` 由本帧 `lua_l_buffinit` 登记，匹配过程可抛错与触发 GC。
/// cpp lstrlib.cpp:831 `str_gsub`。
pub(crate) unsafe fn str_gsub(l: *mut LuaState) -> i32 {
  unsafe {
    let src = lua_l_checklstring_ref(&mut *l, 1);
    let pat = lua_l_checklstring_ref(&mut *l, 2);
    let tr = (*l).type_of(3);
    let max_s = lua_l_optinteger(&mut *l, 4, src.len() as i32 + 1);
    // cpp: `int anchor = (*p == '^')` —— 空 pattern 时 cpp 读终止 NUL，必不为 '^'，
    // 与切片 `first()` 无值同点位
    let anchor = pat.first() == Some(&b'^');
    let mut n: i32 = 0;

    let mut ms = MatchState::default();
    let mut b = LuaLStrbuf::new();

    (*l).arg_expected(
      matches!(
        tr,
        LuaType::Number | LuaType::String | LuaType::Function | LuaType::Table
      ),
      3,
      "string/function/table",
    );

    lua_l_buffinit(&mut *l, &mut b);

    // cpp: `if (anchor) { p++; lp--; }` —— 借用切片右移一格跳过锚定字符（`first()`
    // 已证首字节为 '^'，故 [1..] 界内）
    let pat = if anchor { &pat[1..] } else { pat };
    prepstate(&mut ms, l, src, pat);

    // cpp `gsub`：`while (n < max_s) { e = match(ms, src, p); ...; src = e 或 src++ }`
    // —— 源游标全程为相对 ms.src 的偏移（match_item 亦为偏移协议），无指针游走
    let mut src_off: usize = 0;
    while n < max_s {
      reprepstate(&mut ms);
      let e = match_item(&mut ms, src_off, 0);
      if let Some(e_off) = e {
        n += 1;
        add_value(&mut ms, &mut b, src_off, e_off, tr);
      }

      // cpp: if (e && e > src) src = e; else if (src < ms->src_end) { addchar; src++; } else break;
      match e {
        Some(e_off) if e_off > src_off => src_off = e_off, // non empty match: skip it
        _ if src_off < ms.src.len() => {
          // 游走不变式 src_off < src.len()：界内取字节（终止 NUL 不参与）
          lua_l_addchar(&mut b, ms.src_byte(src_off));
          src_off += 1;
        }
        _ => break,
      }

      if anchor {
        break;
      }
    }

    // cpp: luaL_addlstring(b, src, ms->src_end - src) —— 尾部剩余原文按源偏移取切片追加
    let tail = ms.src_slice(src_off, ms.src.len() - src_off);
    lua_l_addlstring(&mut b, tail);
    lua_l_pushresult(&mut b);
    (*l).push_integer(n); // number of substitutions
    2
  }
}

lua_lib_fn!(pub(crate) fn str_gsub, str_gsub_arm);
