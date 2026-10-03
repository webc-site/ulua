use crate::{
  functions::{
    lua_tolstring::lua_tolstring_ref, r#match::match_item, prepstate::prepstate,
    push_captures::push_captures, reprepstate::reprepstate,
  },
  macros::{lua_lib_fn::lua_lib_fn, lua_upvalueindex::lua_upvalueindex},
  records::{lua_state::LuaState, match_state::MatchState},
};

/// cpp `lstrlib.cpp gmatch_aux`（lstrlib.cpp:732）：`string.gmatch` 迭代器一次推进。
///
/// # Safety
/// `l` 的存活/独占前提已由 `&mut` 接收者类型承载（r16-v41 收形）；屏障仍保留是因为体内有真实
/// 裸操作：`src`/`pat` 两个 upvalue 串窗经 `lua_tolstring_ref`（收裸 `*mut`）派生，须跨过整轮
/// match 循环存活，并被 `prepstate` 存入 `ms` 后由 `match_item`/`push_captures` 经 `ms.l` 裸句柄
/// 读回（cpp 单句柄 `L` 别名），锚定串窗与后继 `&mut l` 取参/压栈不可共存 ⇒ 依 r16-v29/r16-v38
/// str_gsub 判例在入口一次就地转手裸句柄 `lp = l.as_mut_ptr()`，全程只用 `lp`，与原 `*mut l` 逐位一致。
/// 另：`l` 须处于 gmatch 迭代器闭包的受保护帧，3 个 upvalue 依次为源串/模式串/游标整数；
/// `to_integer` 读游标、`push_integer`/`replace` 写游标、`match_item`/`push_captures` 读写 `ms`
/// 并可抛错/GC，须为可抛错的受保护帧。
pub(crate) unsafe fn gmatch_aux(l: &mut LuaState) -> i32 {
  // SAFETY: `l` 由 `&mut` 保证有效且独占，转手后的 `lp` 即同一存活帧；src/pat 串窗借自 upvalue
  // 串体（Luau 字符串不可变且不被移动），循环内游标读写/压栈不使该串体悬垂
  unsafe {
    let lp = l.as_mut_ptr();
    let mut ms = MatchState::default();
    // upvalue 恒为串（gmatch 闭包契约）；`None` 即 cpp 解引用 NULL 的不可达路径，
    // 收敛为空串（长度 0，游标循环立即结束），不改变正常路径行为
    let src = lua_tolstring_ref(lp, lua_upvalueindex(1)).unwrap_or(&[]);
    let pat = lua_tolstring_ref(lp, lua_upvalueindex(2)).unwrap_or(&[]);

    prepstate(&mut ms, lp, src, pat);

    // cpp `gmatch_aux`：`for (src = s + pos; src <= ms.src_end; src++)`
    // —— 游标改为相对源串的偏移（upvalue 3 本就以偏移整数存放），
    // `..= src.len()` 即原 `src <= src_end` 的串尾哨兵；起点越界时区间为空，
    // 与旧 while 条件首轮即假同一出口
    let start = (*lp).to_integer(lua_upvalueindex(3)).unwrap_or(0) as usize;
    for src_off in start..=src.len() {
      reprepstate(&mut ms);
      let e = match_item(&mut ms, src_off, 0);
      if let Some(e_off) = e {
        // cpp: newstart = e - s; if (e == src) newstart++; （空匹配下次右移一格）
        let mut newstart = e_off as i32;
        if e_off == src_off {
          newstart += 1;
        }
        (*lp).push_integer(newstart);
        (*lp).replace(lua_upvalueindex(3));
        return push_captures(&mut ms, Some(src_off), Some(e_off));
      }
    }

    0
  }
}

lua_lib_fn!(pub(crate) fn gmatch_aux @ref, gmatch_aux_arm);
