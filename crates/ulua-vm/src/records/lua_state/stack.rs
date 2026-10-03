use core::ffi::{c_char, c_void};

use super::LuaState;
use crate::{
  functions::{
    c_slice, c_slice_mut, ensure_stack::ensure_stack, lua_call::lua_call,
    lua_checkstack::lua_checkstack, lua_concat::lua_concat, lua_insert::lua_insert,
    lua_newuserdatatagged::lua_newuserdatatagged, lua_pcall::lua_pcall,
    lua_pushcclosurek::lua_pushcclosurek, lua_pushinteger_64::lua_pushinteger_64,
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
  pub(crate) fn slot_distance(from: StkId, to: StkId) -> i32 {
    // SAFETY: `from`/`to` 为同数组内合法槽指针（见本方法契约），offset_from 仅作
    // 槽距读数；裸字段算术为 StkId 栈布局固有边界。
    unsafe { to.offset_from(from) as i32 }
  }

  /// 「保留 top 槽 + 初始化 + 抬栈顶」的单一收口原语：push 族基元
  /// （nil/number/integer/boolean）与按值拷贝压栈（`lua_a_pushvalue`）对裸槽指针的
  /// 写入全部收敛到 [`Self::reserved_top_slot`] 与 [`Self::incr_top`] 两个最小 unsafe
  /// 边界，方法体自身安全，各调用点只提供安全的槽初始化闭包。
  ///
  /// 扩容先行（`ensure_stack_space` 可能移动栈，故其后再派生本槽独占借用），与旧
  /// 逐方法「ensure → `(*self.top).set_X` → `api_incr_top`」的求值次序逐指令一致。
  #[inline(always)]
  pub(crate) fn push_slot_with(&mut self, init: impl FnOnce(&mut TValue)) {
    self.ensure_stack_space(1);
    init(self.reserved_top_slot().as_mut());
    self.incr_top();
  }

  // ---- r12-w7a1：API 侧搬运/交换族槽算术收口的槽窗/栈顶重锚边界原语 ----
  //
  // 收编口径：functions/ 下搬运/交换族不再就地做 `top` 裸偏移/裸重锚；读面经
  // `top_slot`/`slots_below_top`/`reserved_slots_mut` 派生，写面（栈顶提交）经
  // `reanchor_top`/`advance_top`/`rewind_top` 落笔。各原语为原逐点位表达式的
  // 指针平移镜像——扩容检查、写屏障与 api_check 断言位点全留在调用方，时序不动。
  //
  // r12-w9a 塌缩：w7a2 曾为调用/错误/元表族另建同义对 `raise_top`/`lower_top`
  // （本体与本对逐字同形，仅 unsafe 签名之差），经亲验为纯同义，已灭形收编——
  // 该族全部点位统一走本对 `advance_top`/`rewind_top`。

  /// 边界原语：取相对栈顶 `off` 格的槽地址读数（`off < 0` = 顶下既存槽、`off == 0`
  /// = 保留顶槽、`off > 0` = 顶外预留槽），镜像 cpp `L->top + off` 的读数形；只派生
  /// 地址，不解引用、不写 `top`。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// `top + off` 须落在当前栈数组分配界内——负向界由调用点 `api_checknelems` 类前置
  /// 断言保证，正向界由 `ensure_stack`/`ensure_stack_space` 类扩容先行保证；所得槽
  /// 地址在栈重分配后失效，跨扩容须重读（「扩容先行、借用后派生」纪律同
  /// [`Self::reserved_top_slot`] 契约）。
  #[inline(always)]
  pub(crate) fn top_slot(&self, off: isize) -> StkId {
    // SAFETY: 界内由本方法契约保证；`offset` 仅指针平移、不解引用，与原逐点位
    // `top` 相对偏移读数逐指令等价。
    unsafe { self.top.offset(off) }
  }

  /// 边界原语：栈顶之下 `count` 格的只读槽窗 `[top-count, top)`（搬运族源侧）。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// `count` 须不超过 `top - base`（调用点 `api_checknelems` 类断言先行）；窗口在
  /// 返回借用期内可读，期间栈不得重分配（扩容先行纪律）。
  #[inline(always)]
  pub(crate) fn slots_below_top(&self, count: usize) -> &[TValue] {
    // SAFETY: 界内与存活见本方法契约；窗基址经 `top_slot` 派生，与原逐点位
    // `c_slice(top - count, count)` 形等价。
    unsafe { c_slice(self.top_slot(-(count as isize)), count) }
  }

  /// 边界原语：栈顶之上 `count` 格的预留可写槽窗 `[top, top+count)`（搬运族目标侧/
  /// 补空填充）。不自带断言、不抬栈顶——顶提交仍由调用点按原时序经
  /// [`Self::advance_top`]/[`Self::reanchor_top`] 或 `api_incr_top` 类宏落笔。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// 调用前须已 `ensure_stack`/`ensure_stack_space` 类扩容先行，使顶后 `count` 槽
  /// 可独占写入；窗内写序与写屏障/GC 时序由调用点按 VM 纪律自持（与原裸窗逐格
  /// `c_slice_mut + setobj` 形态一致）；窗口存活期内栈不再重分配。
  #[inline(always)]
  pub(crate) fn reserved_slots_mut(&mut self, count: usize) -> &mut [TValue] {
    // SAFETY: 界内与可写见本方法契约；与原逐点位 `c_slice_mut(top, count)` 等价。
    unsafe { c_slice_mut(self.top, count) }
  }

  /// 边界原语：把栈顶重锚到给定槽地址（镜像 cpp `L->top = <已缓存槽>` 写形）；
  /// 纯字段落笔，无断言、无算术——断言位点保留在调用方。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// `slot` 须为当前栈数组内的槽地址，且按调用点语义满足弹收形 `base <= slot <= top`
  /// 或置顶形（`lua_settop` 类：`ensure_stack` 先行后 `slot <= stack_last`）。
  #[inline(always)]
  pub(crate) fn reanchor_top(&mut self, slot: StkId) {
    self.top = slot;
  }

  /// 边界原语：栈顶升 `count` 格，提交已写好的预留槽（镜像 cpp `L->top += n` 提交
  /// 形）；不自带 `api_incr_top` 断言（与原裸赋值点位逐形一致，需要断言的点位仍走
  /// 调用方原宏）。r12-w9a 塌缩收编：原 w7a2 同义对 `raise_top` 的全部消费点
  /// （调用/错误/元表族裸场域抬顶点位）迁移至本方法，语义与被替代点位逐位同形。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// 顶后 `count` 槽须已由扩容先行覆盖且已按序写完；`top + count` 落在分配界内。
  #[inline(always)]
  pub(crate) fn advance_top(&mut self, count: usize) {
    // SAFETY: 界内见本方法契约；`add` 仅指针平移。
    unsafe { self.top = self.top.add(count) };
  }

  /// 边界原语：栈顶降 `count` 格收掉栈顶 `count` 槽（镜像 cpp `L->top -= n` 弹栈
  /// 提交形）。r12-w9a 塌缩收编：原 w7a2 同义对 `lower_top` 的全部消费点迁移至本
  /// 方法，语义与被替代点位逐位同形。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// `count` 须不超过 `top - base`（调用点消费断言先行）；所得新顶落在分配界内。
  #[inline(always)]
  pub(crate) fn rewind_top(&mut self, count: usize) {
    // SAFETY: 界内见本方法契约；`sub` 仅指针平移。
    unsafe { self.top = self.top.sub(count) };
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
    lua_pushinteger_64(self, n)
  }

  #[inline(always)]
  pub fn push_bytes(&mut self, s: &[u8]) {
    // SAFETY: `self` 为存活 `&mut LuaState`（接收者类型承载）；`s` 为借用切片，
    // 核心界内拷入堆串不留存，契约见 `lua_pushlstring_bytes`。
    unsafe { lua_pushlstring_bytes(self, s) }
  }

  #[inline(always)]
  pub fn push_str(&mut self, s: &str) {
    self.push_bytes(s.as_bytes())
  }

  #[inline(always)]
  pub fn push_value(&mut self, idx: i32) {
    lua_pushvalue(self, idx)
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
    lua_settop(self, idx)
  }

  #[inline(always)]
  pub fn remove(&mut self, idx: i32) {
    lua_remove(self, idx)
  }

  #[inline(always)]
  pub fn insert(&mut self, idx: i32) {
    lua_insert(self, idx)
  }

  #[inline(always)]
  pub fn replace(&mut self, idx: i32) {
    lua_replace(self, idx)
  }

  #[inline(always)]
  pub fn concat(&mut self, n: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_concat(self.as_mut_ptr(), n) }
  }

  #[inline(always)]
  pub fn check_stack(&mut self, sz: i32) -> bool {
    // r16-v3：callee 已前移 `&mut LuaState` 引用形，本门面直传独占借用，无 unsafe 残留。
    lua_checkstack(self, sz) != 0
  }

  #[inline(always)]
  pub(crate) fn ensure_stack_space(&mut self, extra: usize) {
    // SAFETY: `self` 是存活已初始化 `LuaState` 的独占借用，借用期内无其它别名；
    // 其余前提与被转发的 `unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { ensure_stack(self, extra as i32) }
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
    lua_pcall(self, nargs, nresults, errfunc)
  }

  #[inline(always)]
  pub fn call(&mut self, nargs: i32, nresults: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 的有效指针；其余前提（合法索引/栈界）
    // 与被转发的 `pub unsafe fn` 的 `# Safety` 文档一致，由本方法调用方按文档保证。
    unsafe { lua_call(self.as_mut_ptr(), nargs, nresults) }
  }
}
