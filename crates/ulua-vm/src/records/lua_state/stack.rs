use core::ffi::{c_char, c_void};

use super::LuaState;
use crate::{
  functions::{
    ensure_stack::ensure_stack, lua_call::lua_call, lua_checkstack::lua_checkstack,
    lua_concat::lua_concat, lua_insert::lua_insert, lua_newuserdatatagged::lua_newuserdatatagged,
    lua_pcall::lua_pcall, lua_pushcclosurek::lua_pushcclosurek,
    lua_pushinteger_64::lua_pushinteger_64,
    lua_pushlightuserdatatagged::lua_pushlightuserdatatagged,
    lua_pushlstring::lua_pushlstring_bytes, lua_pushvalue::lua_pushvalue, lua_remove::lua_remove,
    lua_replace::lua_replace, lua_settop::lua_settop,
  },
  macros::api_incr_top::api_incr_top,
  records::{lua_t_value::TValue, slot::Slot},
  type_aliases::{lua_c_function::LuaCFunction, stk_id::StkId},
};

impl LuaState {
  /// 边界原语：把 `self.top`（指向独立分配栈数组中的 StkId）转成保留 top 槽的
  /// [`Slot`] 句柄（B2-0 「reserved_top_slot 家族对接」）。全模块唯一把 `self.top`
  /// 收口为类型化槽句柄之处——push 族的裸槽写入不再散落，只经这一个有契约的最小
  /// 边界，写面经 [`Slot::as_mut`] 派生独占借用（与原 `&mut *self.top` 同窗口）。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// 调用前须已 [`Self::ensure_stack_space`]`(1)`，从而 `self.top` 落在分配栈界
  /// 内且指向一个可独占写入的空槽；栈数组在 `self` 的借用期内存活。
  #[inline(always)]
  fn reserved_top_slot(&mut self) -> Slot<'_> {
    // SAFETY: 见本方法契约——ensure_stack_space 已保证 top 槽在分配栈界内且可独占写入。
    unsafe { Slot::from_raw(self.top) }
  }

  /// 边界原语：写完保留 top 槽后抬一格栈顶，封装 `api_incr_top!` 的裸指针算术，
  /// 使栈顶自增不再以 inline 指针运算渗透到方法体。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// 紧接 [`Self::reserved_top_slot`] 的写入之后调用；`top < ci->top` 断言按 VM
  /// 约定成立。
  #[inline(always)]
  fn incr_top(&mut self) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；`api_incr_top!` 仅做
    // debug 断言与 `top = top.add(1)`，见本方法契约。
    unsafe { api_incr_top!(self.as_mut_ptr()) }
  }

  /// 边界原语：同一次栈分配内两个 StkId 之间的槽距读数，收口裸字段 `offset_from`
  /// 算术，替代散落的 `self.top.offset_from(self.base)`。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// `from`/`to` 须指向同一次栈分配内的合法槽（lua_State 不变量）。
  #[inline(always)]
  fn slot_distance(from: StkId, to: StkId) -> i32 {
    // SAFETY: `from`/`to` 为同数组内合法槽指针（见本方法契约），offset_from 仅作
    // 槽距读数；裸字段算术为 StkId 栈布局固有边界。
    unsafe { to.offset_from(from) as i32 }
  }

  /// 「保留 top 槽 + 初始化 + 抬栈顶」的单一收口原语：push 族基元
  /// （nil/number/integer/boolean）对裸槽指针的写入全部收敛到
  /// [`Self::reserved_top_slot`] 与 [`Self::incr_top`] 两个最小 unsafe 边界，方法体
  /// 自身安全，各调用点只提供安全的槽初始化闭包。
  ///
  /// 扩容先行（`ensure_stack_space` 可能移动栈，故其后再派生本槽独占借用），与旧
  /// 逐方法「ensure → `(*self.top).set_X` → `api_incr_top`」的求值次序逐指令一致。
  #[inline(always)]
  fn push_slot_with(&mut self, init: impl FnOnce(&mut TValue)) {
    self.ensure_stack_space(1);
    init(self.reserved_top_slot().as_mut());
    self.incr_top();
  }

  #[inline(always)]
  pub fn push_nil(&mut self) {
    self.push_slot_with(TValue::set_nil)
  }

  #[inline(always)]
  pub fn push_number(&mut self, n: f64) {
    self.push_slot_with(move |slot| slot.set_nvalue(n))
  }

  #[inline(always)]
  pub fn push_integer(&mut self, n: i32) {
    self.push_slot_with(move |slot| slot.set_nvalue(n as f64))
  }

  #[inline(always)]
  pub fn push_boolean(&mut self, b: bool) {
    self.push_slot_with(move |slot| slot.set_bvalue(b as i32))
  }

  /// 压入 64 位整数值（`LUA_TINTEGER` 精确槽，与 [`Self::push_integer`] 的 f64 槽相对）。
  /// 扩容由 [`lua_pushinteger_64`] 内部完成。
  #[inline(always)]
  pub fn push_integer_64(&mut self, n: i64) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；契约见
    // `lua_pushinteger_64`（内部自管扩容）。
    unsafe { lua_pushinteger_64(self.as_mut_ptr(), n) }
  }

  #[inline(always)]
  pub fn push_bytes(&mut self, s: &[u8]) {
    // SAFETY: 存活 LuaState 指针 + `s` 为借用切片（长度随传），契约见
    // `lua_pushlstring_bytes`。
    unsafe { lua_pushlstring_bytes(self.as_mut_ptr(), s) }
  }

  #[inline(always)]
  pub fn push_str(&mut self, s: &str) {
    self.push_bytes(s.as_bytes())
  }

  #[inline(always)]
  pub fn push_value(&mut self, idx: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_pushvalue(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn pop(&mut self, n: i32) {
    self.set_top(-n - 1)
  }

  #[inline(always)]
  pub fn get_top(&self) -> i32 {
    // 栈顶槽距经边界原语 `slot_distance` 收口（见其契约）：base/top 为同次栈分配
    // 内的合法槽指针（lua_State 不变量）。
    Self::slot_distance(self.base, self.top)
  }

  #[inline(always)]
  pub fn set_top(&mut self, idx: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_settop(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn remove(&mut self, idx: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_remove(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn insert(&mut self, idx: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_insert(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn replace(&mut self, idx: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_replace(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn concat(&mut self, n: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_concat(self.as_mut_ptr(), n) }
  }

  #[inline(always)]
  pub fn check_stack(&mut self, sz: i32) -> bool {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_checkstack(self.as_mut_ptr(), sz) != 0 }
  }

  #[inline(always)]
  pub(crate) fn ensure_stack_space(&mut self, extra: usize) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { ensure_stack(self.as_mut_ptr(), extra as i32) }
  }

  /// 压入一个 `tag = 0` 的 full userdata，返回其 payload 指针。
  /// 扩容由 [`lua_newuserdatatagged`] 内部 `ensure_stack` 完成。
  ///
  /// 返回指针的存活期由 GC 维持（userdata 被回收后不得再解引用），仅作不透明
  /// payload 使用。
  #[inline(always)]
  pub fn new_userdata(&mut self, s: usize) -> *mut c_void {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_newuserdatatagged(self.as_mut_ptr(), s, 0) }
  }

  /// 压入一个 lightuserdata 槽。
  ///
  /// # Safety
  /// `p` 仅作不透明指针值存入（VM 不解引用），但签名保留 unsafe：裸指针入参
  /// 的合法性（指向存活内存或为整数编码值）由调用方域约定。
  #[inline(always)]
  pub unsafe fn push_lightuserdata(&mut self, p: *mut c_void) {
    // SAFETY: 入参 `p` 按本方法契约作不透明值透传，被调方不解引用。
    unsafe { lua_pushlightuserdatatagged(self.as_mut_ptr(), p, 0) }
  }

  /// # Safety
  /// `f` 须遵循 Lua C 函数约定；`debugname` 须为空或在闭包存活期内有效的 NUL 字符串指针。
  #[inline(always)]
  pub unsafe fn push_c_function(&mut self, f: LuaCFunction, debugname: *const c_char) {
    // SAFETY: `f`/`debugname` 按本方法契约原样透传给 `lua_pushcclosurek`。
    unsafe { lua_pushcclosurek(self.as_mut_ptr(), f, debugname, 0, None) }
  }

  /// # Safety
  /// `f` 须遵循 Lua C 函数约定；`debugname` 须为空或在闭包存活期内有效的 NUL 字符串指针；栈顶须有 `nup` 个待捕获上值。
  #[inline(always)]
  pub unsafe fn push_c_closure(&mut self, f: LuaCFunction, debugname: *const c_char, nup: i32) {
    // SAFETY: `f`/`debugname`/`nup` 按本方法契约原样透传给 `lua_pushcclosurek`。
    unsafe { lua_pushcclosurek(self.as_mut_ptr(), f, debugname, nup, None) }
  }

  #[inline(always)]
  pub fn pcall(&mut self, nargs: i32, nresults: i32, errfunc: i32) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_pcall(self.as_mut_ptr(), nargs, nresults, errfunc) }
  }

  #[inline(always)]
  pub fn call(&mut self, nargs: i32, nresults: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_call(self.as_mut_ptr(), nargs, nresults) }
  }
}
