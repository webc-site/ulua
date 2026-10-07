//! Source: `VM/src/lapi.cpp:65-97` (hand-ported; includes the file-static
//! `getcurrenv` helper inlined here since it was never a graph node)

use crate::{
  macros::{
    api_check::api_check, curr_func::curr_func, lua_environindex::LUA_ENVIRONINDEX,
    lua_globalsindex::LUA_GLOBALSINDEX, lua_ispseudo::lua_ispseudo,
    lua_o_nilobject::LUA_O_NILOBJECT, lua_registryindex::LUA_REGISTRYINDEX, registry::registry,
    sethvalue::sethvalue,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 `LuaState`，`(*l).gt` 有效；当 `(*l).ci != (*l).base_ci` 时当前调用帧
/// `func` 须为闭包（`curr_func` 取 `(*cl).env`），否则回退到 `gt`。cpp `lapi.cpp:81`。
unsafe fn getcurrenv(l: *mut LuaState) -> *mut LuaTable {
  unsafe {
    if (*l).ci == (*l).base_ci {
      // no enclosing function? use global table as environment
      (*l).gt
    } else {
      let func = curr_func!(l);
      (*func).env
    }
  }
}

/// 伪索引 → 槽地址换算（cpp `pseudo2addr`）。`l` 以引用传入（存活由类型保证）；
/// `idx` 须满足 `lua_ispseudo(idx)`（`api_check` 保证），LUA_ENVIRONINDEX/GLOBALSINDEX
/// 分支会经 `sethvalue` 写 `global.pseudotemp` 并可能触发写屏障（写路径只触
/// `global`/GC 对象头，不写穿 `l` 本体）；越界伪索引返回 `luaO_nilobject`。
/// cpp `lapi.cpp:89`。
pub(crate) fn pseudo_2_addr(l: &LuaState, idx: i32) -> StkId {
  api_check!(l, lua_ispseudo(idx));
  // SAFETY:`l` 存活（引用形保证）；写路径仅触 `(*l).global.pseudotemp`、GC 对象
  // 头与 upvalue 槽位，均不写穿 `l` 本体，`read_ptr` 只读转发契约成立（见其文档）。
  unsafe {
    let lp = l.read_ptr();
    match idx {
      // pseudo-indices
      LUA_REGISTRYINDEX => registry!(lp) as *const TValue as *mut TValue,
      LUA_ENVIRONINDEX => {
        let tmp = &mut (*(*lp).global).pseudotemp as *mut TValue;
        sethvalue!(lp, tmp, getcurrenv(lp));
        tmp
      }
      LUA_GLOBALSINDEX => {
        let tmp = &mut (*(*lp).global).pseudotemp as *mut TValue;
        sethvalue!(lp, tmp, (*lp).gt);
        tmp
      }
      _ => {
        let func = curr_func!(lp);
        // cpp `LUA_GLOBALSINDEX - idx`：合法 upvalue 伪索引（`lua_upvalueindex(i)`，
        // i≥1）折叠出 1..=255 的 i，行为不变；对垃圾超负 idx 该减法是符号溢出 UB
        // （溢出后 `i <= nupvalues` 判真、`upvals[i-1]` 出负下标）。
        // DELIBERATE DEVIATION（cpp lapi.cpp:103-105）：改 wrapping 折叠 +
        // `1..=nupvalues` 闭区间判定，溢出折叠出的非正值/越界值一律返回
        // `LUA_O_NILOBJECT` 哨兵；合法 upvalue 索引行为与 cpp 逐位一致。
        let i = LUA_GLOBALSINDEX.wrapping_sub(idx);
        if (1..=(*func).nupvalues as i32).contains(&i) {
          let c = &mut (*func).inner.c;
          c.upvals.as_mut_ptr().add((i - 1) as usize)
        } else {
          LUA_O_NILOBJECT as *mut TValue
        }
      }
    }
  }
}
