use crate::{
  functions::{
    lmemfind::lmemfind, lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_optinteger::lua_l_optinteger, lua_pushinteger::lua_pushinteger, lua_pushnil::lua_pushnil,
    lua_toboolean::lua_toboolean, r#match::match_item, nospecials::nospecials, posrelat::posrelat,
    prepstate::prepstate, push_captures::push_captures, reprepstate::reprepstate,
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
      lua_pushnil(l);
      return 1;
    }
    // 上方钳位保证 init >= 1，故 start 为合法源偏移（== src.len() 即 cpp 串尾哨兵）
    let start = init as usize - 1;

    // cpp: `if (find && (lua_toboolean(L, 4) || nospecials(p, lp)))` 走 plain 分支
    if find != 0 && (lua_toboolean(l, 4) != 0 || nospecials(pat) != 0) {
      // cpp: lmemfind(s + init - 1, ls - init + 1, p, lp) —— 窗口 [init-1, ls) 即 src[start..]
      if let Some(hit) = lmemfind(&src[start..], pat) {
        let found = start as i32 + hit as i32;
        lua_pushinteger(l, found + 1);
        lua_pushinteger(l, found + pat.len() as i32);
        return 2;
      }
    } else {
      let mut ms = MatchState::default();
      // cpp: `int anchor = (*p == '^'); if (anchor) { p++; lp--; }` —— 空 pattern 时
      // cpp 读终止 NUL，必不为 '^'，与切片 `first()` 无值同点位
      let anchor = pat.first() == Some(&b'^');
      let pat = if anchor { &pat[1..] } else { pat }; // skip anchor character
      prepstate(&mut ms, l, src, pat);

      // cpp `str_find_aux`：`s1 = s + (init - 1)`，`while (s1++ < ms.src_end && !anchor)`
      // —— s1 游标为相对 ms.src 的偏移游走
      let mut s1_off = start;
      loop {
        reprepstate(&mut ms);
        // Safety: 钳位保证 s1_off ∈ [init-1, src.len()]（== src_end 即原串尾哨兵）
        let res = match_item(&mut ms, s1_off, 0);
        if let Some(res_off) = res {
          if find != 0 {
            // cpp: push integer(s1 - s + 1), push integer(res - s)
            lua_pushinteger(l, s1_off as i32 + 1);
            lua_pushinteger(l, res_off as i32);
            // cpp: push_captures(ms, NULL, NULL) + 2 —— None 即原 NULL 哨兵
            return push_captures(&mut ms, None, None) + 2;
          } else {
            return push_captures(&mut ms, Some(s1_off), Some(res_off));
          }
        }

        // C++ `s1++ < ms.src_end`：先比较旧游标（< src_end 即 s1_off < src.len()）再前进
        if anchor || s1_off >= src.len() {
          break;
        }
        s1_off += 1;
      }
    }

    lua_pushnil(l);
    1
  }
}
