use crate::{
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// 可调用形态本体（[`LuauFastFunction`] 的 `Some` 分支），即表槽里真正装着的
/// 快速调用函数指针。拆出别名供 [`FastCallEntry::Fast`]
/// （crate::enums::fast_call_entry::FastCallEntry::Fast）携带 payload 用，
/// `LuauFastFunction` 逐项不变（同型 `Option`，`const` 装表与 JIT `offset_of!`
/// 消费面均不受影响）。
pub type LuauFastCallable = unsafe extern "C-unwind" fn(
  l: *mut LuaState,
  res: StkId,
  arg0: *mut TValue,
  nresults: i32,
  args: Option<&mut TValue>,
  nparams: i32,
) -> i32;

/// `luauF_table` 槽位的快速调用函数指针（cpp `luau_FastFunction`，lbuiltins.h:9）。
///
/// `args` 为第二实参起的连续槽窗口；`None` 表示「无第二参数槽」的 arity 变体
/// （`LOP_FASTCALL1` 单实参派发，cpp 侧以 `NULL` 表达），处理器据此直接回退
/// 慢路径（返回 -1），与 `nparams < 2` 的门禁语义一致。`Some` 时指针由调用方
/// （`luau_execute` 各 FASTCALL 臂）从活栈帧算得，可读写范围与 `nparams` 相符。
pub type LuauFastFunction = Option<LuauFastCallable>;
