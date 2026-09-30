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
  unsafe {
    let func = (*(*l).ci).func;
    try_as_closure_ptr((*func).value.gc).unwrap_or(null_mut())
  }
}

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).top` 至少指向一个作为替换源的 TValue
/// （`api_checknelems 1`）；`idx` 经 `index_2_addr` 解析须返回非 `LUA_O_NILOBJECT` 的可写槽，
/// 且 `idx==LUA_ENVIRONINDEX` 时须处于活动调用帧（`(*l).ci!=(*l).base_ci`）且顶元素为 table，
/// `idx==LUA_GLOBALSINDEX` 时顶元素亦须为 table；写 `(*func).env`/`(*l).gt` 及屏障可 GC，
/// 需受保护帧。cpp `lapi.cpp:302`。
pub unsafe fn lua_replace(l: *mut LuaState, idx: i32) {
  unsafe {
    api_checknelems!(l, 1);
    lua_c_threadbarrier_lapi(l);
    let o: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(o, LUA_O_NILOBJECT));
    // 栈顶单槽窗口：`(*l).top.offset(-1)` 的六连裸重读收为一次预绑定
    // （index_2_addr/屏障/GC 均不改写 `(*l).top`，读取时机与逐指令等价）
    let src: StkId = (*l).top.offset(-1);
    if idx == LUA_ENVIRONINDEX {
      api_check!(l, (*l).ci != (*l).base_ci);
      let func: *mut Closure = current_closure(l);
      api_check!(l, (*src).is_table());
      (*func).env = (*src).as_table_ptr();
      lua_c_barrier!(l, func, src);
    } else if idx == LUA_GLOBALSINDEX {
      api_check!(l, (*src).is_table());
      (*l).gt = (*src).as_table_ptr();
    } else {
      setobj!(l, o, src);
      if idx < LUA_GLOBALSINDEX {
        lua_c_barrier!(l, current_closure(l), src);
      }
    }
    (*l).top = src;
  }
}
