use core::ptr::{null, null_mut};

use crate::type_aliases::stk_id::StkId;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CallInfo {
  // DELIBERATE DEVIATION（§2 判定=规则 2「null 只是指针算式基址」，故保留 `*mut`，逐字段理由见下）：
  // base/func/top 是 cpp `lstate.h:59-61` 的 `StkId`（即 `TValue*`），指向 `LuaState.stack` arena 内的
  // 栈切片。cpp 里 CI 复用/新建一律先写死这三者再入栈，使用期从不为 null，null 只落在下面 `Default`
  // 的「未接线占位形态」。改成 `Option<NonNull<TValue>>` 会把取槽、算帧宽、边界比较里多处
  // `ptr::offset_from`/地址相减的热路径逼成 unwrap 分支——违反 §9.4「关键路径不得变慢」，且 `Option`
  // 无法参与 `func + 1`、`top.offset_from(base)` 这类裸地址算术。
  /// `base` 是本帧寄存器堆栈切片基址。热路径以它为算式起点：由 `ci->base = ci->func + 1`
  /// （cpp `lstate.cpp:179`、`lvmexecute.cpp:3875`）派生，`base[i]` 取操作数槽，
  /// `v.offset_from((*ci).base)`（`dumpthread.rs:134`）求寄存器号，`L->base = ci->base` 整体搬运地址、
  /// api 边界以 `(*l).base` 做范围比较。`Option` 会让这些差值/加一算式套 unwrap 且不可表达（§9.4）。
  pub base: StkId,
  /// `func` 指向栈上「本帧被调函数」槽，是 `base` 的算式锚点（`base == func + 1`，见 [`Self::base`]）。
  /// cpp 建帧即赋值、从不为空，null 只在 `Default` 占位。改 `Option` 会给 `func + 1`、按 `func` 做指针差
  /// 的热点加 unwrap 且让加一不可表达（§9.4）。
  pub func: StkId,
  /// `top` 是本帧栈顶，纯算式操作数：`(*l.ci).top.offset_from(l.top)`（`ensure_stack.rs` 的
  /// `try_reserve_stack`）求增栈量、`p_val <= (*(*l).ci).top`（`api_update_top!`）做越界比较、`top - base` 定帧宽。cpp 建帧时
  /// `ci->top` 即写为合法栈内地址，null 只在 `Default` 占位。改 `Option` 会给每次压栈/扩容判定加 unwrap
  /// 分支（§9.4），且相减无法对 `Option` 表达。
  pub top: StkId,
  pub savedpc: *const u32,
  pub errfunc: i32,
  pub nresults: i32,
  pub flags: u32,
}

impl Default for CallInfo {
  // 既有约定（review.md §2）：堆对象 POD 字段初值——base/func/top 的 null 仅作 Default 占位，帧入栈时即写合法栈内地址；契约见各字段 doc（§9.4 改 Option 会加 unwrap 分支）
  fn default() -> Self {
    Self {
      base: null_mut(),
      func: null_mut(),
      top: null_mut(),
      savedpc: null(),
      errfunc: 0,
      nresults: 0,
      flags: 0,
    }
  }
}
