//! Source: `VM/src/lapi.cpp:99-118` (hand-ported)

use crate::{
  functions::pseudo_2_addr::pseudo_2_addr,
  macros::{
    api_check::api_check, lua_o_nilobject::LUA_O_NILOBJECT, lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// api 索引 → 栈槽地址换算（cpp `index2addr`）。`l` 以引用传入（存活由类型保证）；
/// 本体仅做指针距离读数与偏移，不解引用栈元素、不写 `l`。任意 `idx`（含越界/0）均
/// 收敛为 `LUA_O_NILOBJECT` 哨兵或合法槽地址，无栈外指针算术（见各分支注释）；
/// 返回的 StkId 仅在栈未重分配前有效。cpp/VM/src/lapi.cpp:115 index2addr。
pub fn index_2_addr(l: &LuaState, idx: i32) -> StkId {
  if idx > 0 {
    // SAFETY: `(*l.ci).top` 与 `base` 指向同一栈数组，`l.ci` 有效存活，`offset_from` 仅作距离读数不解引用。
    api_check!(
      l,
      idx as isize <= unsafe { (*l.ci).top.offset_from(l.base) }
    );
    // 先比较再偏移：C++ 里 `base + (idx - 1)` 只是个悬垂指针，随后与 top 比
    // 较返回 nilobject（lua_type(L, 1000) 是合法调用）；Rust 里对越界 off 做
    // `add` 本身就是 UB，所以用偏移量比较替代指针比较。
    let off = (idx - 1) as usize;
    let used = unsafe { l.top.offset_from(l.base) };
    if off >= used as usize {
      LUA_O_NILOBJECT as *mut TValue
    } else {
      // SAFETY:`off < used`，`base.add(off)` 落在栈数组 `[base, top)` 内。
      unsafe { l.base.add(off) }
    }
  } else if idx > LUA_REGISTRYINDEX {
    // 负索引（或 0）：`top + idx` 仅当 `idx != 0` 且结果落在 `[base, top)` 内
    // 才是合法指针算术。cpp 仅以 debug 断言表达该契约，release 下对越界负索
    // 引直接做出栈数组下界的指针算术（UB，无 oracle 输出）。
    // DELIBERATE DEVIATION（cpp lapi.cpp:126-129）：越界（含 idx==0）返回
    // `LUA_O_NILOBJECT` 哨兵——与 cpp 正索引越界同款降级（消费方如 lua_type
    // 得到确定的 LUA_TNONE），不再产生栈外指针。断言条件改写为非移项形式
    // （cpp 的 `-idx` 对 i32::MIN 是符号溢出），定义域内与 cpp 逐位一致；
    // 快路径仅多一次 isize 加法比较，正常路径零额外间接。
    let rel = (idx as isize) + unsafe { l.top.offset_from(l.base) };
    api_check!(l, idx != 0 && rel >= 0);
    if idx != 0 && rel >= 0 {
      // SAFETY:`rel >= 0` 且 `|idx| <= used`，`top.offset(idx)` 落在栈数组内。
      unsafe { l.top.offset(idx as isize) }
    } else {
      LUA_O_NILOBJECT as *mut TValue
    }
  } else {
    pseudo_2_addr(l, idx)
  }
}
