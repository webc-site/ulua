//! Source: `VM/src/lapi.cpp:302-333` (hand-ported)

use core::ptr::{eq, null_mut};

use crate::{
  functions::{index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_c_barrier::lua_c_barrier,
    lua_environindex::LUA_ENVIRONINDEX, lua_globalsindex::LUA_GLOBALSINDEX,
    lua_o_nilobject::LUA_O_NILOBJECT, setobj::setobj,
  },
  records::{closure::Closure, gc_object::try_as_closure_ptr, lua_state::LuaState},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且当前调用帧 `(*l).ci` 的 `func` 槽内确为一个闭包
/// TValue（`(*(*ci).func).value.gc` 指向存活 GCObject），否则解引用 `.cl` 未定义。
/// 对应 cpp `lapi.cpp:302` 中 `lua_replace` 使用的 `clsL(L)` 宏。
unsafe fn current_closure(l: *mut LuaState) -> *mut Closure {
  // 既有约定（review.md §2）：VM c-API 边界签名折返——`current_closure` 返回 `*mut Closure`，C 闭包/非闭包时 null 为合法折返，边界体内保留
  unsafe {
    let func = (*(*l).ci).func;
    try_as_closure_ptr((*func).value.gc).unwrap_or(null_mut())
  }
}

/// `lua_replace` 核心（cpp `VM/src/lapi.cpp:302`）。调用序契约（正确性，非内存安
/// 全）：栈顶至少留 1 个替换源（`api_checknelems 1`）；`idx` 经硬化的
/// `index_2_addr` 须解析为非哨兵的可写槽；`idx==LUA_ENVIRONINDEX` 时须处于活动
/// 调用帧（`ci != base_ci`）且当前帧 func 为 C 闭包、顶元素为 table，
/// `idx==LUA_GLOBALSINDEX` 时顶元素亦须为 table；写 `func.env`/`gt` 及屏障可
/// GC，须在受保护帧内调用。
pub(crate) fn lua_replace(l: &mut LuaState, idx: i32) {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已硬化；setobj/屏障/current_closure
  // 的指针前提由引用形、调用序契约与 VM 栈不变式成立。
  unsafe {
    let lp = l.as_mut_ptr();
    api_checknelems!(lp, 1);
    lua_c_threadbarrier_lapi(lp);
    let o: StkId = index_2_addr(&*lp, idx);
    api_check!(lp, !eq(o, LUA_O_NILOBJECT));
    // 栈顶单槽窗口：`(*lp).top.offset(-1)` 的六连裸重读收为一次预绑定
    // （index_2_addr/屏障/GC 均不改写 `(*lp).top`，读取时机与逐指令等价）
    let src: StkId = (*lp).top.offset(-1);
    if idx == LUA_ENVIRONINDEX {
      api_check!(lp, (*lp).ci != (*lp).base_ci);
      let func: *mut Closure = current_closure(lp);
      api_check!(lp, (*src).is_table());
      (*func).env = (*src).as_table_ptr();
      lua_c_barrier!(lp, func, src);
    } else if idx == LUA_GLOBALSINDEX {
      api_check!(lp, (*src).is_table());
      (*lp).gt = (*src).as_table_ptr();
    } else {
      setobj!(lp, o, src);
      if idx < LUA_GLOBALSINDEX {
        lua_c_barrier!(lp, current_closure(lp), src);
      }
    }
    (*lp).top = src;
  }
}
