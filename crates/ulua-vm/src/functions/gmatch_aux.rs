use crate::{
  functions::{
    lua_pushinteger::lua_pushinteger, lua_replace::lua_replace, lua_tolstring::lua_tolstring_ref,
    r#match::match_item, prepstate::prepstate, push_captures::push_captures,
    reprepstate::reprepstate,
  },
  macros::{
    lua_lib_fn::lua_lib_fn, lua_tointeger::lua_tointeger, lua_upvalueindex::lua_upvalueindex,
  },
  records::{lua_state::LuaState, match_state::MatchState},
};

/// cpp `lstrlib.cpp gmatch_aux`（lstrlib.cpp:730）：`string.gmatch` 迭代器一次推进。
///
/// # Safety
/// `l` 须为存活 LuaState 并处于 gmatch 迭代器闭包的受保护帧，其 3 个 upvalue 依次为源串/模式串/游标整数：
/// `lua_tolstring_ref` 取回 `s`/`p` 切片（`None` 即 cpp 的 NULL+0 不可达路径，折算为空串）、
/// `lua_tointeger` 读游标，`match_item`/`push_captures` 读写 `ms` 并可抛错/GC。
pub(crate) unsafe fn gmatch_aux(l: *mut LuaState) -> i32 {
  unsafe {
    let mut ms = MatchState::default();
    // upvalue 恒为串（gmatch 闭包契约）；`None` 即 cpp 解引用 NULL 的不可达路径，
    // 收敛为空串（长度 0，游标循环立即结束），不改变正常路径行为
    let src = lua_tolstring_ref(l, lua_upvalueindex(1)).unwrap_or(&[]);
    let pat = lua_tolstring_ref(l, lua_upvalueindex(2)).unwrap_or(&[]);

    prepstate(&mut ms, l, src, pat);

    // cpp `gmatch_aux`：`for (src = s + pos; src <= ms.src_end; src++)`
    // —— 游标改为相对源串的偏移（upvalue 3 本就以偏移整数存放），
    // `..= src.len()` 即原 `src <= src_end` 的串尾哨兵；起点越界时区间为空，
    // 与旧 while 条件首轮即假同一出口
    let start = lua_tointeger!(l, lua_upvalueindex(3)) as usize;
    for src_off in start..=src.len() {
      reprepstate(&mut ms);
      let e = match_item(&mut ms, src_off, 0);
      if let Some(e_off) = e {
        // cpp: newstart = e - s; if (e == src) newstart++; （空匹配下次右移一格）
        let mut newstart = e_off as i32;
        if e_off == src_off {
          newstart += 1;
        }
        lua_pushinteger(l, newstart);
        lua_replace(l, lua_upvalueindex(3));
        return push_captures(&mut ms, Some(src_off), Some(e_off));
      }
    }

    0
  }
}

lua_lib_fn!(pub(crate) fn gmatch_aux, gmatch_aux_arm);
