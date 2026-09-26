use core::ffi::c_char;

use crate::{
  functions::{
    lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_addvalue::lua_l_addvalue, lua_l_prepbuffsize::lua_l_prepbuffsize,
    lua_tolstring::lua_tolstring_ref, push_onecapture::push_onecapture,
  },
  macros::{l_esc::L_ESC, lua_l_error::luaL_error},
  records::{lua_l_strbuf::LuaLStrbuf, match_state::MatchState},
};

/// cpp `lstrlib.cpp add_s`（lstrlib.cpp:765）：按替换串模板把 `%m` 捕获展开进累加器。
/// `s`/`e` 为整窗匹配的源偏移对（`%0` 用），偏移==cpp 指针同界。
///
/// # Safety
/// `ms.l` 为存活调用帧（槽 3 为替换串、`lua_l_addvalue` 读栈顶）、`b` 为其上登记的
/// 累加器；`s <= e <= ms.src.len()`（`%0` 分支的源窗口由 `MatchState` 门面取切片）。
pub(crate) unsafe fn add_s(ms: &mut MatchState, b: &mut LuaLStrbuf, s: usize, e: usize) {
  // Safety: 契约保证 `l` 存活、拼接区由 `lua_l_prepbuffsize` 先行扩展，块内写入不越界
  unsafe {
    // 槽 3 恒为替换串（gsub 契约）；`None` 即 cpp 解引用 NULL 的不可达路径，收敛为
    // 空串（不追加任何内容），不改变正常路径行为
    let repl = lua_tolstring_ref(ms.l, 3).unwrap_or(&[]);
    lua_l_prepbuffsize(b, repl.len());

    // cpp `for (i = 0; i < l; i++)`：`%` 分支吞掉其后的序号/转义字节，正对应迭代器
    // 多取一次 `next`；`%` 为末字节时 cpp 读串尾终止 NUL（值为 0，既非数字也非
    // `L_ESC` → 报错），`next` 取 None 折算成 0 即同点位
    let mut bytes = repl.iter().copied();
    while let Some(c) = bytes.next() {
      if c != L_ESC as u8 {
        lua_l_addchar(b, c as c_char);
        continue;
      }

      let next = bytes.next().unwrap_or(0); // skip ESC
      if !next.is_ascii_digit() {
        if next != L_ESC as u8 {
          luaL_error!(
            ms.l,
            "invalid use of '{}' in replacement string",
            L_ESC as u8 as char
          );
        }
        lua_l_addchar(b, next as c_char);
      } else if next == b'0' {
        // cpp: lua_addlstring(b, s, e - s); —— %0 整窗，按源偏移切片直接追加
        lua_l_addlstring(b, ms.src_slice(s, e - s));
      } else {
        push_onecapture(ms, (next - b'1') as i32, Some(s), Some(e));
        lua_l_addvalue(b); // add capture to accumulated result
      }
    }
  }
}
