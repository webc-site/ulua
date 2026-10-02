use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{c_slice_mut, lua_d_call::lua_d_call},
  macros::{l_isfalse::l_isfalse, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
///
/// 比较器回调（`table.sort` 的 sort predicate 中转）：`l_state` 必须指向专用于该回调的
/// 存活 `LuaState`，其 `top == base + 2`（[表, 函数] 布局，LUAU_ASSERT 兜底）且栈深满足
/// LUA_MINSTACK（本函数额外占 3 槽）；`l`/`r` 必须指向排序数组内的存活可读 TValue。
pub(crate) unsafe extern "C-unwind" fn sort_func(
  l_state: *mut LuaState,
  l: *const TValue,
  r: *const TValue,
) -> i32 {
  // SAFETY: 契约保证 top==base+2 且有 LUA_MINSTACK 余量，top..top+3 写槽与 d_call 恢复均界内
  unsafe {
    LUAU_ASSERT!((*l_state).top == (*l_state).base.offset(2)); // table, function

    let top = (*l_state).top;
    let base = (*l_state).base;

    // 栈窗口 top..top+3：契约的 LUA_MINSTACK 余量保证界内（本函数不重分配栈），
    // 三槽写收为 c_slice_mut 视图，自上方单次窗读派生；写序与原逐槽指针一致
    let args = c_slice_mut(top, 3);
    setobj_2_s!(l_state, &raw mut args[0], base.offset(1));
    setobj_2_s!(l_state, &raw mut args[1], l);
    setobj_2_s!(l_state, &raw mut args[2], r);

    // r12-w7a2 收编：抬顶经 raise_top 原语（LUA_MINSTACK 余量保证界内；raw 绑定后
    // 窗口内仅写已界内槽、无场域写，原语现读场与窗值同）
    (*l_state).raise_top(3);
    lua_d_call(l_state, top, 1);
    // 收编：比较器调用可搬栈，回落必须现读场域——lower_top 原语与被替代式同址同宽
    (*l_state).lower_top(1); // maintain stack depth

    (!l_isfalse!((*l_state).top)) as i32
  }
}
