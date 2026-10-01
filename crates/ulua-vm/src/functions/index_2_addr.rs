//! Source: `VM/src/lapi.cpp:99-118` (hand-ported)

use crate::{
  functions::pseudo_2_addr::pseudo_2_addr,
  macros::{
    api_check::api_check, lua_o_nilobject::LUA_O_NILOBJECT, lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 LuaState 且当前帧 `(*(*l).ci).top`、`(*l).base`、`(*l).top` 指向同一栈数组并保持
/// `base <= top` 不变式。`idx` 为任意 i32：正索引越界（≥ top）与负索引越界（0 或落到 `base` 之下）
/// 都按语义返回 `LUA_O_NILOBJECT` 哨兵；伪索引走 `pseudo_2_addr`（越界 upvalue 伪索引同样返回
/// 哨兵）。返回的 StkId 仅在栈未重分配前有效。
/// cpp/VM/src/lapi.cpp:115 index2addr。
pub unsafe fn index_2_addr(l: *mut LuaState, idx: i32) -> StkId {
  unsafe {
    if idx > 0 {
      api_check!(l, idx as isize <= (*(*l).ci).top.offset_from((*l).base));
      // 先比较再偏移：C++ 里 `base + (idx - 1)` 只是个悬垂指针，随后与 top 比
      // 较返回 nilobject（lua_type(L, 1000) 是合法调用）；Rust 里对越界 off 做
      // `add` 本身就是 UB，所以用偏移量比较替代指针比较。
      let off = (idx - 1) as usize;
      if off >= (*l).top.offset_from((*l).base) as usize {
        LUA_O_NILOBJECT as *mut TValue
      } else {
        (*l).base.add(off)
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
      let rel = (idx as isize) + (*l).top.offset_from((*l).base);
      api_check!(l, idx != 0 && rel >= 0);
      if idx != 0 && rel >= 0 {
        (*l).top.offset(idx as isize)
      } else {
        LUA_O_NILOBJECT as *mut TValue
      }
    } else {
      pseudo_2_addr(l, idx)
    }
  }
}
