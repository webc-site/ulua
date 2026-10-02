//! Source: `VM/src/ldo.cpp` (ldo.cpp:718-727, hand-ported)

use core::ffi::c_void;

use crate::{
  functions::lua_d_callny::lua_d_callny,
  macros::{incr_top::incr_top, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧（错误处理器本身，允许再抛）：`ud` 转成的 `errfunc` 须为落在
/// `(*l).stack` 内的有效 `StkId`（错误函数槽位）；`(*l).top` 之后须已预留 ≥2 空槽，因本函数经两次
/// `setobj_2_s!` 写 `(*l).top`、`top-1` 并 `incr_top!` 抬栈，再由 `lua_d_callny` 以 `(top-2,1)` 调用；可触发 GC。
/// cpp VM/src/ldo.cpp:718
pub(crate) unsafe extern "C-unwind" fn callerrfunc(l: *mut LuaState, ud: *mut c_void) {
  unsafe {
    let errfunc = ud as StkId;

    // r12-w7a2 收编：两连写位的三处 `(*l).top` 场域重读收为同一函数体内的
    // 等价窗预绑定（写槽序不变；setobj2s 的屏障不动栈，绑定全程有效）
    let top: StkId = (*l).top;
    setobj_2_s!(l, top, top.sub(1));
    setobj_2_s!(l, top.sub(1), errfunc);
    incr_top!(l);

    // 保留（恢复点后现读）：incr_top! 内含 checkstack，可搬栈后必须现读场域——
    // cpp 同形最小读面 `luaD_callnoyield(L, L->top - 2, 1)`
    lua_d_callny(l, (*l).top.offset(-2), 1);
  }
}
