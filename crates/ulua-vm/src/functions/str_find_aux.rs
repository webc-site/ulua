use crate::{
  functions::{
    lmemfind::lmemfind, lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_optinteger::lua_l_optinteger, r#match::match_item, nospecials::nospecials,
    posrelat::posrelat, prepstate::prepstate, push_captures::push_captures,
    reprepstate::reprepstate,
  },
  records::{lua_state::LuaState, match_state::MatchState},
};

/// cpp `lstrlib.cpp str_find_aux`（lstrlib.cpp:663）：`string.find`/`string.match` 共用体。
///
/// # Safety
/// `l` 必须是正在执行的 string 库 C 函数帧的存活 `LuaState`：栈槽 #1/#2 为源串/
/// pattern（`lua_l_checklstring_ref` 的借用切片在本次调用全程存活，偏移游走由
/// `src.len()`/`pat.len()` 钳位），#3 为起始索引、#4 为 plain 布尔；结果全部经
/// `lua_push*`/`push_captures` 压栈，匹配递归深度由 `MatchState.matchdepth` 兜底。
pub unsafe fn str_find_aux(l: *mut LuaState, find: i32) -> i32 {
  unsafe {
    let src = lua_l_checklstring_ref(l, 1);
    let pat = lua_l_checklstring_ref(l, 2);

    let mut init = posrelat(lua_l_optinteger(l, 3, 1), src.len());
    if init < 1 {
      init = 1;
    } else if init > src.len() as i32 + 1 {
      // cpp `(int)ls + 1`：源串长度按同一 `i32` 域比较，截断语义逐点位保持
      (*l).push_nil();
      return 1;
    }
    // 上方钳位保证 init >= 1，故 start 为合法源偏移（== src.len() 即 cpp 串尾哨兵）
    let start = init as usize - 1;

    // cpp: `if (find && (lua_toboolean(L, 4) || nospecials(p, lp)))` 走 plain 分支
    if find != 0 && ((*l).to_boolean(4) || nospecials(pat) != 0) {
      // cpp: lmemfind(s + init - 1, ls - init + 1, p, lp) —— 窗口 [init-1, ls) 即 src[start..]
      if let Some(hit) = lmemfind(&src[start..], pat) {
        let found = start as i32 + hit as i32;
        (*l).push_integer(found + 1);
        (*l).push_integer(found + pat.len() as i32);
        return 2;
      }
    } else {
      let mut ms = MatchState::default();
      // cpp: `int anchor = (*p == '^'); if (anchor) { p++; lp--; }` —— 空 pattern 时
      // cpp 读终止 NUL，必不为 '^'，与切片 `first()` 无值同点位
      let anchor = pat.first() == Some(&b'^');
      let pat = if anchor { &pat[1..] } else { pat }; // skip anchor character
      prepstate(&mut ms, l, src, pat);

      // cpp do-while `s1++ < ms.src_end && !anchor`：候选起点遍历 [start, src.len()]
      // （== src.len() 的串尾哨兵点位也试一次，do-while 体至少执行一轮），
      // anchor 时失败即止——只试首点位
      for s1_off in start..=src.len() {
        reprepstate(&mut ms);
        // SAFETY: 钳位保证 s1_off ∈ [init-1, src.len()]（== src_end 即原串尾哨兵）
        let res = match_item(&mut ms, s1_off, 0);
        if let Some(res_off) = res {
          if find != 0 {
            // cpp: push integer(s1 - s + 1), push integer(res - s)
            (*l).push_integer(s1_off as i32 + 1);
            (*l).push_integer(res_off as i32);
            // cpp: push_captures(ms, NULL, NULL) + 2 —— None 即原 NULL 哨兵
            return push_captures(&mut ms, None, None) + 2;
          } else {
            return push_captures(&mut ms, Some(s1_off), Some(res_off));
          }
        }

        if anchor {
          break;
        }
      }
    }

    (*l).push_nil();
    1
  }
}
