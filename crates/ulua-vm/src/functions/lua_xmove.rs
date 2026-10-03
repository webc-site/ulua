use crate::{
  functions::{ensure_stack::ensure_stack_impl, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{api_check::api_check, api_checknelems::api_checknelems, setobj_2_s::setobj_2_s},
  records::{lua_state::LuaState, lua_t_value::TValue},
};

/// `lua_xmove`（cpp/VM/src/lapi.cpp:203）：把 `from` 栈顶 `n` 个值搬到 `to` 栈顶。
/// 调用序契约（正确性，非内存安全；r16-v3 引用形前移，两侧存活由 `&mut` 类型承载）：
/// `from`、`to` 须为同 `global_State` 下两棵 `LuaState`（`api_check` 断言 `(*from).global == (*to).global`）；
/// `n >= 0` 且 `from` 栈顶有 `n` 个可消费项（`api_checknelems`）；`ensure_stack_impl(to, from, n)` 保证 `to` 栈
/// 顶之后容 `n` 新槽（失败在 `from` 抛错）。`(*from).top - n..top` 与 `(*to).top..top + n` 互不重叠（异线程；
/// 同线程由 `from == to` 指针相等短路），`setobj_2_s` 逐项搬移并同步 `top`。
pub fn lua_xmove(from: &mut LuaState, to: &mut LuaState, n: i32) {
  // r16-v3：引用形参入体即重建裸指针别名（来源为两侧独占借用引用，指针不出本帧），
  // 体内自 `api_check!` 起逐字保持原裸指针实现——窗写 `dst_win` 与 `setobj_2_s!` 的
  // `(*to).global` 读数同帧共存需裸形，引用直传会触发 E0503 借用冲突。
  let from: *mut LuaState = from;
  let to: *mut LuaState = to;
  // SAFETY: 契约保证两侧同 VM 存活、`n` 在 from 顶可消费；块内栈槽裸指针读写、
  // ci 界扩容与顶指针落笔均属实现本体。
  unsafe {
    api_check!(from, n >= 0);

    if from == to {
      return;
    }

    api_checknelems!(from, n);
    api_check!(from, (*from).global == (*to).global);

    lua_c_threadbarrier_lapi(to);

    // cpp `ensure_stack_impl(to, from, n)`：目标帧不够时扩容，失败在 from 上抛错
    ensure_stack_impl(&mut *to, &mut *from, n);

    // 槽窗门面：源侧取 `from` 顶下 n 格只读窗、目标侧取 `to` 顶后 n 格预留可写窗
    //（扩容先行已由 ensure_stack_impl 覆盖，窗基址/界内契约见原语文档）
    let src_win = (&*from).slots_below_top(n as usize);
    let dst_win = (&mut *to).reserved_slots_mut(n as usize);

    // SAFETY:from 栈顶下 n 格值有效（api_checknelems 已过）；to 栈保证容纳 n 个新值
    //（checkstack 已过）。
    for (dst, src) in dst_win.iter_mut().zip(src_win) {
      setobj_2_s!(to, dst as *mut TValue, src as *const TValue as *mut TValue);
    }

    // 顶提交：写窗未触 `top` 字段，`advance_top(n)`/`rewind_top(n)` 与原
    // `to->top = ttop + n` / `from->top = ftop` 落值逐位一致，提交次序不动
    (&mut *from).rewind_top(n as usize);
    (&mut *to).advance_top(n as usize);
  }
}
