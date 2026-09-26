//! JIT 慢路径的 VM 帧门面（review.md §2「把 unsafe 关进有契约的最小边界」）。
//!
//! 生成码经 `extern "C-unwind"` 回调进入 `execute*`/`call*` 系列慢路径时，参数是
//! `(*mut LuaState, *const Instruction, StkId base, *mut TValue k)` 这组裸地址——它们与
//! ulua-vm 的指针式 ABI 一起构成本 crate 的 VM 边界。本类型在边界处一次性收下这些地址，
//! 之后业务逻辑（取指令字、寻址栈槽、读写 state 字段、调用 VM 例程）全部经方法以
//! safe API 进行：
//!
//! - 字节码/栈槽/表 node/array 的读取一律收敛为切片视图（[`VmFrame::insns`]/
//!   [`VmFrame::slots`]·[`VmFrame::slots_mut`]/[`VmFrame::table_nodes`]·
//!   [`VmFrame::table_array`]），下标界内检查替代裸指针算术；`reg`/`slot_at` 等
//!   「仅算不解引用」的地址平移用 wrapping 算术安全化，机器语义与原 `ptr::add` 一致；
//! - 可空指针（metatable、元方法、direct-field 表）以 `Option` 判别返回，
//!   null 哨兵不再渗透到业务层；
//! - 单行「活对象字段读/写壳」经 `define_vm_frame_accessor!` 宏表同形收口（含
//!   `opt` 空哨兵折叠臂），逐处 `// Safety` 论证保留在条目上方；
//! - 余下的 `unsafe` 全部是 GC 堆解引用与 ulua-vm `unsafe fn` 例程调用（VM ABI
//!   契约，(c) 类），每处附 `# Safety`/`// Safety` 论证；要再消需 ulua-vm 侧改
//!   出借安全引用的 API（跨 crate，见改写报告待办）。
//!
//! 不变量：构造后、本值存活期间 `(*l).ci` 为活 CallInfo、`base` 落在分配栈内
//! （与 cpp 慢路径函数同一 VM 调用约定）。凡可能触发栈重分配的调用（vm_protect 等）
//! 之后必须经方法重取槽位，不得缓存旧地址。

use core::{
  ffi::c_int,
  mem::size_of,
  ptr::{from_mut, from_ref, null_mut},
  slice::{from_raw_parts, from_raw_parts_mut},
};

use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b};
use ulua_vm::{
  enums::tms::TMS,
  functions::{
    lua_d_call::lua_d_call, lua_d_growstack::lua_d_growstack,
    lua_d_performcally::lua_d_performcally, lua_f_new_lclosure::lua_f_new_lclosure,
    lua_g_methoderror::lua_g_methoderror, lua_g_typeerror_l::lua_g_typeerror_l,
    lua_h_getstr::lua_h_getstr, lua_h_resizearray::lua_h_resizearray, lua_h_setstr::lua_h_setstr,
    lua_o_rawequal_obj::lua_o_rawequal_obj, lua_v_call_tm::lua_v_call_tm,
    lua_v_gettable::lua_v_gettable, lua_v_settable::lua_v_settable,
    set_iterator_done::set_iterator_done, set_iterator_index::set_iterator_index,
  },
  macros::{
    checkliveness::checkliveness, fastnotm::fastnotm, fasttm::fasttm, getstr::getstr,
    gval_2_slot::gval2slot, lua_c_barrier::lua_c_barrier, lua_c_barrierfast::lua_c_barrierfast,
    lua_c_barriert::luaC_barriert, lua_c_check_gc::lua_c_check_gc, setclvalue::setclvalue,
    sethvalue::sethvalue, setnilvalue::setnilvalue, setnvalue::setnvalue, setobj::setobj,
    setobj_2_s::setobj_2_s, setobj_2_t::setobj2t, stacklimitreached::stacklimitreached,
    ttype::ttype,
  },
  records::{
    closure::Closure, lua_node::LuaNode, lua_table::LuaTable, proto::Proto, t_string::tstring,
    udata::Udata,
  },
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  functions::vm_kv::vm_kv,
  macros::{
    define_vm_frame_accessor::define_vm_frame_accessor, vm_patch_c::vm_patch_c,
    vm_protect_pc::vm_protect_pc,
  },
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

/// 一条慢路径指令的执行上下文：`l` + 帧基址 `base` 的成对句柄。
#[derive(Clone, Copy)]
pub(crate) struct VmFrame {
  l: *mut LuaState,
  base: StkId,
}

impl VmFrame {
  /// 收编一次 VM 回调的 `l`/`base` 参数。
  ///
  /// # Safety
  /// `l` 为存活 `LuaState`（`ci`/`top`/`stack`/`stacksize`/`cachedslot` 一致有效）；
  /// `base` 为活动栈帧基址，使 `VM_REG!` 语义下的寄存器号可解析到栈内。
  #[inline]
  pub(crate) unsafe fn new(l: *mut LuaState, base: StkId) -> Self {
    Self { l, base }
  }

  /// 仅凭活 `l` 收帧：帧基址取当前 `L->base`（FORGLOOP 等非 `execute*` 回调入口）。
  ///
  /// # Safety
  /// `l` 为存活 `LuaState` 且其 `base` 即活动栈帧基址（VM 调度不变量）。
  #[inline]
  pub(crate) unsafe fn current(l: *mut LuaState) -> Self {
    // Safety: 依类型不变量——L->base 为活动帧基址，转交 `new` 的同一契约。
    let base = unsafe { (*l).base };
    unsafe { Self::new(l, base) }
  }

  /// 本指令之后的字流起点为 `pc`、可读 `n` 个指令字的切片视图。
  ///
  /// 界内性由字节码结构（指令长度 / 捕获字数 ≤ code 数组长度，VM ABI 契约）保证。
  #[inline]
  pub(crate) fn insns(&self, pc: *const Instruction, n: usize) -> &[Instruction] {
    // Safety: `pc` 指向存活字节码数组内的合法位置且 `n` 个后续字均在该数组内
    // （各调用点按其 `# Safety` 契约论证，与本视图约定一致）。
    unsafe { from_raw_parts(pc, n) }
  }

  /// 栈寄存器寻址（cpp `VM_REG`，含界内断言）。
  #[inline]
  pub(crate) fn reg(&self, index: i32) -> StkId {
    // 界内复核保持 VM_REG! 语义：index（u32 视角）< top − base。地址算术用
    // wrapping_add，与既往 `base.add` 的机器语义完全一致（有效输入同地址；
    // 无效输入在原实现本就是 UB，此处仅多一道 debug 断言护栏）。
    ulua_common::LUAU_ASSERT!((index as u32) < (self.slots_diff(self.top(), self.base) as u32));
    self.base.wrapping_add(index as usize)
  }

  /// 同 `reg`，但以指令 A 字段寻址。
  #[inline]
  pub(crate) fn reg_a(&self, insn: Instruction) -> StkId {
    self.reg(luau_insn_a(insn) as i32)
  }

  /// 同 `reg`，但以指令 B 字段寻址。
  #[inline]
  pub(crate) fn reg_b(&self, insn: Instruction) -> StkId {
    self.reg(luau_insn_b(insn) as i32)
  }

  /// `pc.offset(n)`：第 n 个后续指令字的起点（可为 code 数组 one-past-end，仅算不读）。
  #[inline]
  pub(crate) fn insn_offset(&self, pc: *const Instruction, n: usize) -> *const Instruction {
    // 纯地址算术、从不解引用；读字一律经 `insns` 切片视图。机器语义与原 `pc.add(n)` 一致。
    pc.wrapping_add(n)
  }

  /// `pc.offset(delta)`（带符号）：条件跳转的目标字（cpp `pc + LUAU_INSN_D(insn)`，
  /// 可为 one-past-end，仅算不读）。
  #[inline]
  pub(crate) fn insn_jump(&self, pc: *const Instruction, delta: isize) -> *const Instruction {
    // 纯地址算术、从不解引用（跳转目标落在活字节码数组内由编译器保证，调用点契约）。
    pc.wrapping_offset(delta)
  }

  /// 常量表第 `i` 槽地址（调用点已断言 `i < proto->sizek`）。
  #[inline]
  pub(crate) fn const_slot(&self, k: *mut TValue, i: u32) -> *mut TValue {
    // 纯地址算术、从不解引用；界内性由调用点 AUX 下标契约保证（同原 `k.add`）。
    k.wrapping_add(i as usize)
  }

  define_vm_frame_accessor! {
    /// `Closure::env`（Lua 闭包构造时接线的活表）。
    // Safety: 类型不变量——cl 为活 L 闭包，env 恒为存活表指针。
    closure_env(cl: *mut Closure) -> *mut LuaTable = (*cl).env;
    /// `Closure::nupvalues`。
    // Safety: 类型不变量——cl 为活闭包。
    closure_nupvalues(cl: *const Closure) -> u8 = (*cl).nupvalues;
    /// `Closure::preload` 旗标。
    // Safety: 同上。
    closure_preload(cl: *const Closure) -> u8 = (*cl).preload;
    /// `Closure::inner.l.p`：Lua 闭包的非空活 Proto（调用点已证 `is_c == 0`）。
    // Safety: 类型不变量——L 闭包 inner.l.p 为非空活 Proto（union 域读取，闭包构造时接线）。
    closure_proto(cl: *const Closure) -> *mut Proto = (*cl).inner.l.p;
    /// `Proto::code`：字节码数组基址（回退路径回推 savedpc 用）。
    // Safety: 类型不变量——p 为活 Proto（调用点取自 L 闭包）。
    proto_code(p: *const Proto) -> *const Instruction = (*p).code;
  }

  define_vm_frame_accessor! {
    /// `Closure::is_c != 0`（C 闭包判定）。
    // Safety: 类型不变量——cl 为活闭包（调用点取自 clvalue/活栈槽）。
    closure_is_c(cl: *const Closure) -> bool = (*cl).is_c != 0;
    /// 清 `Closure::preload`（dupclosure 收尾）。
    // Safety: 同上。
    clear_closure_preload(cl: *mut Closure) -> () = (*cl).preload = 0;
    /// `Closure::inner.l.uprefs` 基址（柔性尾部定长数组，元素数 = nupvalues）。
    // Safety: 类型不变量——cl 为活 L 闭包，uprefs 随闭包分配、构造时接线。
    closure_uprefs(cl: *mut Closure) -> *mut TValue = (*cl).inner.l.uprefs.as_mut_ptr();
  }

  /// `Closure::inner.l.uprefs[i]`：upvalue 槽地址（调用点保证 `i < nupvalues`）。
  #[inline]
  pub(crate) fn closure_upvalue(&self, cl: *mut Closure, i: usize) -> *mut TValue {
    // 纯地址算术、从不解引用（同 `slot_at` 约定）；界内由调用点 nupvalues 契约保证。
    self.closure_uprefs(cl).wrapping_add(i)
  }

  /// `lua_f_new_lclosure`：按 `nelems` 个 upvalue 分配 L 闭包。
  #[inline]
  pub(crate) fn new_lclosure(
    &self,
    nelems: c_int,
    env: *mut LuaTable,
    p: *mut Proto,
  ) -> *mut Closure {
    let l = self.l;
    // Safety: 类型不变量——l 活 state、env 活表、p 活 Proto（调用点自闭包字段取得）。
    unsafe { lua_f_new_lclosure(l, nelems, env, p) }
  }

  /// `setclvalue!`：向栈槽写闭包值。
  #[inline]
  pub(crate) fn set_closure_value(&self, slot: StkId, cl: *mut Closure) {
    let l = self.l;
    // Safety: 类型不变量——slot 为界内可写栈槽、cl 为活闭包。
    unsafe { setclvalue!(l, slot, cl) };
  }

  /// `setobj!`：存活检查下的 TValue 原样拷贝。
  #[inline]
  pub(crate) fn copy_value(&self, dst: *mut TValue, src: *const TValue) {
    let l = self.l;
    // Safety: 类型不变量——dst 可写、src 存活（调用点按各自契约论证）。
    unsafe { setobj!(l, dst, src) };
  }

  /// `lua_o_rawequal_obj(...) != 0`：两槽原始相等判定。
  #[inline]
  pub(crate) fn raw_equal(&self, a: *const TValue, b: *const TValue) -> bool {
    // Safety: 调用点已证两槽均为存活 TValue（纯读数）。
    unsafe { lua_o_rawequal_obj(a, b) != 0 }
  }

  /// `lua_c_barrier!`：闭包写屏障。
  #[inline]
  pub(crate) fn barrier_closure(&self, cl: *mut Closure, v: *mut TValue) {
    let l = self.l;
    // Safety: 类型不变量——cl 为活闭包、v 为存活槽（刚写入 cl 的被写对象）。
    unsafe { lua_c_barrier!(l, cl, v) };
  }

  /// `lua_c_check_gc!`：增量 GC 步进检查（可能重入 VM；调用点须在 protect 内）。
  #[inline]
  pub(crate) fn check_gc(&self) {
    let l = self.l;
    // Safety: 类型不变量——l 为活 state。
    unsafe { lua_c_check_gc!(l) };
  }

  define_vm_frame_accessor! {
    /// `LuaTable::nodemask8`（哈希部分掩码，槽号按其取模即界内）。
    // Safety: 调用点已证 h 为活表。
    table_nodemask8(h: *mut LuaTable) -> i32 = (*h).nodemask8 as i32;
    /// `Proto::numparams`（定参个数）。
    // Safety: 类型不变量——p 为活 Proto（调用点取自 L 闭包）。
    proto_numparams(p: *const Proto) -> i32 = (*p).numparams as i32;
  }

  define_vm_frame_accessor! {
    /// `LuaTable::readonly` 旗标。
    // Safety: 调用点已证 h 为活表。
    table_readonly(h: *mut LuaTable) -> u8 = (*h).readonly;
  }

  define_vm_frame_accessor! {
    /// `LuaTable::metatable`：表 `h` 的元表；无元表为 `None`（cpp nullptr 哨兵收进
    /// Option 判别，业务层不再判裸空指针）。
    // Safety: 调用点已证 h 为活表；metatable 槽为空或活表。
    opt table_metatable(h: *mut LuaTable) -> *mut LuaTable = (*h).metatable;
    /// `Udata::metatable`：userdata `u` 的元表；无元表为 `None`。
    // Safety: 调用点已证 u 为活 userdata；metatable 槽为空或活表。
    opt udata_metatable(u: *const Udata) -> *mut LuaTable = (*u).metatable;
  }

  /// `global.udatadirectfields[udata.tag]`；未注册 direct-field 表为 `None`。
  #[inline]
  pub(crate) fn udata_direct_field_dispatch(&self, u: *const Udata) -> Option<*mut LuaTable> {
    // Safety: 类型不变量——global 存活、注册表为定长数组，tag 索引由 userdata 注册侧约束。
    let dispatch = unsafe { (*(*self.l).global).udatadirectfields[(*u).tag as usize] };
    (!dispatch.is_null()).then_some(dispatch)
  }

  /// `global.mt[index]`（可为空；`index` 取 `LuaType` 判别值或 `value_type` 读数）。
  #[inline]
  pub(crate) fn global_metatable(&self, index: u32) -> Option<*mut LuaTable> {
    // Safety: 类型不变量——global 存活，mt 各槽为空或活表；index 由标签低位置域算得。
    let mt = unsafe { (*(*self.l).global).mt[index as usize] };
    (!mt.is_null()).then_some(mt)
  }

  /// `sethvalue!`：向栈槽挂表值。
  #[inline]
  pub(crate) fn set_table(&self, slot: &mut TValue, h: *mut LuaTable) {
    let l = self.l;
    // Safety: 类型不变量——slot 为可写 TValue；h 为活表（调用点取自活对象字段）。
    unsafe { sethvalue!(l, slot, h) };
  }

  /// 槽地址相对帧基址的寄存器号（`reg` 的逆运算；供已算好的槽地址回推编号）。
  #[inline]
  pub(crate) fn reg_index(&self, slot: StkId) -> i32 {
    self.slots_diff(slot, self.base)
  }

  /// `slot.offset(n)`：同段栈内存地址平移（VM ABI 契约保证不越分配栈界）。
  #[inline]
  pub(crate) fn slot_at(&self, slot: StkId, n: usize) -> StkId {
    // 纯地址算术、从不解引用；读写一律经 `slots*`/`set_*` 等有界方法。
    slot.wrapping_add(n)
  }

  /// `slot.sub(n)`：同段栈内存地址回移（VM ABI 契约保证不越分配栈界）。
  #[inline]
  pub(crate) fn slot_back(&self, slot: StkId, n: usize) -> StkId {
    // 纯地址算术、从不解引用（同上）。
    slot.wrapping_sub(n)
  }

  /// 栈上连续 `n` 槽的只读切片视图 `[slot, slot+n)`。
  #[inline]
  pub(crate) fn slots(&self, slot: StkId, n: usize) -> &[TValue] {
    // Safety: 调用点论证 slot..slot+n 落在活栈数组界内（VM ABI），且切片存续期内
    // 不跨可能重分配栈的重入点。
    unsafe { from_raw_parts(slot, n) }
  }

  /// 栈上连续 `n` 槽的可写切片视图 `[slot, slot+n)`。
  ///
  /// 返回切片的生命周期与 `&self` 解耦（`'s` 独立）：数据实际活在 `LuaState`
  /// 栈数组而非本帧句柄内，借用检查层面无需连带 `self`，写权限独占由调用点
  /// Safety 注解决定（与改造前 `&self` 省略式签名的运行时行为一致）。
  #[inline]
  pub(crate) fn slots_mut<'s>(&self, slot: StkId, n: usize) -> &'s mut [TValue] {
    // Safety: 同 `slots`，且单线程 VM 串行执行下写权限独占成立（调用点论证）。
    unsafe { from_raw_parts_mut(slot, n) }
  }

  /// 变参个数 `n = base - ci->func - proto.numparams - 1`（GETVARARGS 系慢路径）。
  #[inline]
  pub(crate) fn varargs_count(&self) -> i32 {
    // 字段读全部经带契约访问器壳；槽距算术安全（slots_diff）。
    let p = self.closure_proto(self.current_closure());
    self.slots_diff(self.base, self.ci_func()) - self.proto_numparams(p) - 1
  }

  /// 当前被调闭包（`clvalue(L->ci->func)`）。
  #[inline]
  pub(crate) fn current_closure(&self) -> *mut Closure {
    // Safety: 类型不变量——(*l).ci 为活 CallInfo，其 func 为存活闭包 TValue。
    unsafe { (*(*(*self.l).ci).func).as_closure_ptr() }
  }

  /// 以 `l` 为宿主把 `src` 拷入栈槽 `dst`（`setobj_2_s!`）。
  #[inline]
  pub(crate) fn set_stack_value(&self, dst: StkId, src: *const TValue) {
    let l = self.l;
    // Safety: 类型不变量——dst 为界内可写栈槽；src 为存活 TValue（调用点按各自契约论证）。
    unsafe { setobj_2_s!(l, dst, src) };
  }

  /// 向栈槽写入 nil（`setnilvalue!`）。
  #[inline]
  pub(crate) fn set_nil(&self, slot: StkId) {
    // Safety: 类型不变量——slot 为界内可写栈槽。
    unsafe {
      setnilvalue!(slot);
    }
  }

  define_vm_frame_accessor! {
    // Safety: 类型不变量——slot 为存活 TValue，纯读标签。
    is_table(slot: *const TValue) -> bool = (*slot).is_table();
    // Safety: 同上。
    is_userdata(slot: *const TValue) -> bool = (*slot).is_userdata();
    // Safety: 同上。
    is_vector(slot: *const TValue) -> bool = (*slot).is_vector();
    // Safety: 同上。
    is_function(slot: *const TValue) -> bool = (*slot).is_function();
    // Safety: 同上。
    is_string(slot: *const TValue) -> bool = (*slot).is_string();
    // Safety: 同上。
    is_nil(slot: *const TValue) -> bool = (*slot).is_nil();
  }

  define_vm_frame_accessor! {
    /// `L->cachedslot` 读。
    // Safety: 类型不变量——l 为活 state，cachedslot 为普通 i32 字段。
    lget cachedslot as cachedslot -> i32;
    /// `L->top` 读。
    // Safety: 类型不变量——l 为活 state。
    lget top as top -> StkId;
    /// `L->base` 读（`sync_base` 的取回源）。
    // Safety: 同上。
    lget base as lbase -> StkId;
    /// `L->stack`：分配栈数组基址。
    // Safety: 同上。
    lget stack as stack_start -> StkId;
    /// `L->stack_last`：分配栈保留区末尾。
    // Safety: 同上。
    lget stack_last as stack_last -> StkId;
    /// `L->stacksize`：分配栈容量（槽数）。
    // Safety: 同上。
    lget stacksize as stack_capacity -> c_int;
  }

  define_vm_frame_accessor! {
    /// `L->cachedslot` 写。
    // Safety: 同上。
    lset cachedslot as set_cachedslot(slot: i32);
    /// `L->top` 写。
    // Safety: 类型不变量——top 为分配栈内合法新栈顶（调用点论证）。
    lset top as set_top(top: StkId);
    /// `L->base` 写（rebind_frame 重接线）。
    // Safety: 类型不变量——new_base 为分配栈内合法新帧基（调用点论证）。
    lset base as set_lbase(new_base: StkId);
  }

  define_vm_frame_accessor! {
    /// `L->ci->top`（当前帧界）。
    // Safety: 类型不变量——ci 为活 CallInfo。
    ciget top as ci_top -> StkId;
    /// `L->ci->func`（当前被调槽）。
    // Safety: 同上。
    ciget func as ci_func -> StkId;
  }

  define_vm_frame_accessor! {
    /// `L->ci->base` 写（rebind_frame 重接线）。
    // Safety: 类型不变量——ci 为活 CallInfo，new_base 为分配栈内合法新帧基。
    ciset base as set_ci_base(new_base: StkId);
    /// `L->ci->top` 写（rebind_frame 重接线）。
    // Safety: 同上。
    ciset top as set_ci_top(top: StkId);
  }

  /// 断言 `top + n` 不越过 `stack + stacksize`（cpp 慢路径入口的栈界复核）。
  #[inline]
  pub(crate) fn assert_top_fits(&self, n: usize) {
    // 纯字段读数 + 地址比较（不解引用 top），经安全访问器与地址算术壳复核。
    ulua_common::LUAU_ASSERT!(
      self.slot_at(self.top(), n)
        < self.slot_at(self.stack_start(), self.stack_capacity() as usize)
    );
  }

  /// `lua_h_getstr`：按字符串键查表（miss 返回 `LUA_O_NILOBJECT` 哨兵槽）。
  #[inline]
  pub(crate) fn get_str(&self, h: *mut LuaTable, key: *mut tstring) -> *const TValue {
    // Safety: 调用点已证 h 为活表、key 为活字符串；本例程为纯查表。
    unsafe { lua_h_getstr(h, key) }
  }

  /// 表 `h` 哈希部分节点切片的只读视图 `[node, node + (1 << lsizenode))`（cpp `sizenode`）。
  #[inline]
  pub(crate) fn table_nodes(&self, h: *mut LuaTable) -> &[LuaNode] {
    // Safety: 调用点已证 h 为活表；node 数组随表一体分配、长 `1 << lsizenode`
    // （空表亦含 1 个 dummynode），视图存续期不跨表重排重入点。
    unsafe { from_raw_parts((*h).node, 1usize << (*h).lsizenode) }
  }

  /// 表 `h` 哈希部分第 `slot` 节点的 (key 视图, 值槽)。key 域是 `TKey`（tt 与 next 打包
  /// 于一字），与 `TValue` 共享偏移 0 的标签/值域（vm 侧 LuaNode 布局约定），故统一以
  /// `TValue` 视图返回；调用点以 `is_string` 等谓词先行判定后再取载荷。
  #[inline]
  pub(crate) fn table_node(&self, h: *mut LuaTable, slot: usize) -> (*const TValue, *const TValue) {
    // 槽号由调用点以 `nodemask8`（= 节点数 − 1）掩码，界内性由切片下标复核。
    let n = &self.table_nodes(h)[slot];
    (from_ref(&n.key).cast(), from_ref(&n.val).cast())
  }

  define_vm_frame_accessor! {
    /// `LuaTable::sizearray`：序列部分长度。
    // Safety: 调用点已证 h 为活表。
    array_len(h: *mut LuaTable) -> i32 = (*h).sizearray;
  }

  /// 表 `h` 序列部分数组切片的可写视图 `[array, array + sizearray)`。
  ///
  /// 视图约定同 [`VmFrame::slots_mut`]：返回切片生命周期与 `&self` 解耦（`'s`
  /// 独立）——数据活在 GC 堆上的表内而非本帧句柄；写权限独占由 vm 单线程串行
  /// 执行保证，不得跨可能重排该数组的重入点（如 `resize_array`）持有。
  #[inline]
  pub(crate) fn table_array<'s>(&self, h: *mut LuaTable) -> &'s mut [TValue] {
    // Safety: 调用点已证 h 为活表；array 随表分配、长 sizearray（非负）。
    unsafe { from_raw_parts_mut((*h).array, (*h).sizearray as usize) }
  }

  /// `array[index]`：序列部分槽地址（调用点已证 `index < sizearray`）。
  #[inline]
  pub(crate) fn array_slot(&self, h: *mut LuaTable, index: usize) -> *mut TValue {
    // 经切片视图界内下标取槽；越界由切片检查兜底（原实现为 UB）。
    from_mut(&mut self.table_array(h)[index])
  }

  /// 把节点 key 的 `value/extra/tt` 逐字段拷入栈槽 `dst` 并复核 liveness
  /// （cpp `getNodeKey`）。
  #[inline]
  pub(crate) fn node_key_into(&self, n: &LuaNode, dst: StkId) {
    // Safety: 类型不变量——n 为活 LuaNode 的引用视图（调用点取自 `table_nodes`，
    // TKey 与 TValue 的 value/extra 同型逐字对齐，tt 经 TKey::tt 位域折叠读出）；
    // dst 为界内可写栈槽；checkliveness! 仅只读断言 GC 颜色。
    unsafe {
      let key = &n.key;
      (*dst).value = key.value;
      (*dst).extra = key.extra;
      (*dst).tt = key.tt();
      checkliveness!((*self.l).global, dst);
    }
  }

  define_vm_frame_accessor! {
    /// `ttype!(slot)`：值的基本类型标签。
    // Safety: 调用点已证 slot 为存活 TValue（纯读标签）。
    value_type(slot: *const TValue) -> u32 = ttype!(slot);
  }

  define_vm_frame_accessor! {
    /// `L->namecall` 写（NAMECALL 慢路径记录被调方法名，寿命随活 string 常量）。
    // Safety: 类型不变量——l 为活 state；ts 为活 string（调用点取自断言过的常量）。
    lset namecall as set_namecall(ts: *mut tstring);
  }

  /// `lua_g_methoderror`：方法缺失报错（必然抛出，不返回）。
  #[inline]
  pub(crate) fn method_error(&self, p1: *const TValue, p2: *const tstring) -> ! {
    // Safety: 类型不变量——p1 为存活槽、p2 活 string；错误路径经 unwind 出栈（C-unwind ABI）。
    unsafe { lua_g_methoderror(self.l, p1, p2) }
  }

  /// `luaG_typeerrorl`：类型不符报错（必然抛出，不返回）。
  #[inline]
  pub(crate) fn type_error(&self, o: *const TValue, op: &str) -> ! {
    // Safety: 类型不变量——o 为存活槽；错误路径经 unwind 出栈（C-unwind ABI）。
    unsafe { lua_g_typeerror_l(self.l, o, op) }
  }

  /// `luaD_call`：受保护点之外的 VM 调用例程（调用点须置于 `protect*` 内）。
  #[inline]
  pub(crate) fn call(&self, func: StkId, nresults: c_int) {
    let l = self.l;
    // Safety: 类型不变量——l 为活 state；func 处已按 Lua 协议放置可调用的栈值序列。
    unsafe { lua_d_call(l, func, nresults) };
  }

  /// `luaD_performCally`：原生执行内自 `func` 起发起 VM 调用；返回 `true` 表示
  /// yield/break，调用方须退出原生执行（FORGLOOP 非表回退专用）。
  #[inline]
  pub(crate) fn perform_call(&self, func: StkId, nresults: c_int) -> bool {
    let l = self.l;
    // Safety: 类型不变量——l 为活 state；func 处已按 Lua 协议放置可调用的栈值序列
    // （FORGLOOP 迭代器帧预留 ra..ra+6 参数槽，调用点写入后成立）。
    unsafe { lua_d_performcally(l, func, nresults) }
  }

  /// 断言 `top` 不越过 `stack_last`（回退路径把参数压到 top 之外的兜底复核）。
  #[inline]
  pub(crate) fn assert_top_reserved(&self) {
    // 纯字段读数比较（不解引用 top），经安全访问器壳复核。
    ulua_common::LUAU_ASSERT!(self.top() <= self.stack_last());
  }

  /// FORGPREP/FORGLOOP 内建迭代「无更多值」标记写（null 载荷构造收口于
  /// `ulua_vm::functions::set_iterator_done`，读侧识别为 `ValueView::IteratorDone`）。
  #[inline]
  pub(crate) fn set_iterator_done(&self, slot: StkId) {
    // Safety: 类型不变量——slot 为界内可写栈槽（内建迭代协议约定的 ra+2 槽），
    // 载荷是协议游标值 0 的指针形态，按迭代器协议永不解引用。
    unsafe { set_iterator_done(slot) };
  }

  /// 内建迭代游标写：载荷按 `(index + 1)` 地址形态装箱（收口于
  /// `ulua_vm::functions::set_iterator_index`；FORGLOOP 读回作整数游标）。
  #[inline]
  pub(crate) fn set_iterator_index(&self, slot: StkId, index: c_int) {
    // Safety: 类型不变量——slot 为界内可写栈槽（内建迭代协议 ra+2）；index 为
    // 有界数组+哈希段游标，整转指针不构造可解引用指针。
    unsafe { set_iterator_index(slot, index) };
  }

  /// 同栈数组内两槽的地址差（`a - b`，供变参计数等算术；调用点保证同数组）。
  #[inline]
  pub(crate) fn slots_diff(&self, a: StkId, b: StkId) -> i32 {
    // usize 地址差算术为纯安全代码；在类型不变量（a/b 同属分配栈数组）下与原
    // `offset_from` 逐位等价。
    ((a as usize).wrapping_sub(b as usize) / size_of::<TValue>()) as i32
  }

  /// `Closure::stacksize`。
  #[inline]
  pub(crate) fn closure_stacksize(&self, cl: *const Closure) -> usize {
    // Safety: 类型不变量——cl 为活 L 闭包。
    unsafe { (*cl).stacksize as usize }
  }

  /// `luaD_growstack` 条件扩栈（cpp `condhardstacktests` 展开；调用点置于 `protect*` 内）。
  #[inline]
  pub(crate) fn check_stack_grow(&self, n: c_int) {
    let l = self.l;
    // Safety: 类型不变量——l 为活 state；n 为帧需求规模（调用点由 stacksize+numparams 算得）。
    unsafe {
      if stacklimitreached(&*l, n) {
        lua_d_growstack(l, n);
      }
    }
  }

  /// 重接当前帧与新栈基（PREPVARARGS 收尾：`ci->base/ci->top/L->base/L->top` 四写）。
  #[inline]
  pub(crate) fn rebind_frame(&self, new_base: StkId, stacksize: usize) {
    // 四写全部经带契约的 lset/ciset 访问器壳；new_top 即原实现回读的 ci->top 值。
    let new_top = self.slot_at(new_base, stacksize);
    self.set_ci_base(new_base);
    self.set_ci_top(new_top);
    self.set_lbase(new_base);
    self.set_top(new_top);
  }

  define_vm_frame_accessor! {
    /// `vector_lanes`：vector 槽的分量视图（调用点已 `is_vector` 判定；引用寿命随栈，
    /// 不得跨可能重分配栈的重入点使用）。
    // Safety: 调用点已以 is_vector 谓词证明槽内存有活 vector。
    vector_lanes(slot: *const TValue) -> &[f32] = (*slot).as_vector_ref();
  }

  /// `getstr` + `len`：字符串值字节视图（含 NUL 终止符，共 `len + 1` 字节）。
  #[inline]
  pub(crate) fn string_bytes(&self, ts: *const tstring) -> &[u8] {
    // Safety: 调用点已证 ts 为活 string；getstr 契约保证可读 len+1 字节。
    unsafe { from_raw_parts(getstr(ts).cast::<u8>(), (*ts).len as usize + 1) }
  }

  /// `setnvalue!`：向栈槽写 number 值。
  #[inline]
  pub(crate) fn set_number(&self, slot: StkId, n: f64) {
    // Safety: 类型不变量——slot 为界内可写栈槽。
    unsafe {
      setnvalue!(slot, n);
    }
  }

  define_vm_frame_accessor! {
    /// `hvalue(slot)`：槽必须为表（调用点已 `is_table` 判定）。
    // Safety: 调用点已以 is_table() 谓词证明槽内存有活表，as_table_ptr 为同址类型化读数。
    hvalue(slot: *const TValue) -> *mut LuaTable = (*slot).as_table_ptr();
    /// `uvalue(slot)`：槽必须为 userdata（调用点已 `is_userdata` 判定）。
    // Safety: 同上，is_userdata() 已证。
    uvalue(slot: *const TValue) -> *const Udata = (*slot).as_userdata_ptr();
  }

  /// 本帧当前栈基（`protect_sync_base` 之后即最新 `L->base`）。
  #[inline]
  pub(crate) fn base_addr(&self) -> StkId {
    self.base
  }

  define_vm_frame_accessor! {
    /// `clvalue(slot)`：槽必须为函数值（调用点已 `is_function` 判定）。
    // Safety: 调用点已以 is_function() 谓词证明槽内存有活闭包。
    clvalue(slot: *const TValue) -> *mut Closure = (*slot).as_closure_ptr();
    /// `tsvalue(slot)`：槽必须为字符串（调用点已 `is_string` 判定）。
    // Safety: 调用点已以 is_string() 谓词证明槽内存有活字符串。
    tsvalue(slot: *const TValue) -> *const tstring = (*slot).as_string_ptr();
  }

  /// 读当前帧 `ci->savedpc` 并置为本指令之后的 `pc`（`vm_protect_pc`）。
  #[inline]
  pub(crate) fn save_pc(&self, pc: *const Instruction) {
    let l = self.l;
    // Safety: 类型不变量——l 为活 state，pc 为字节码数组内合法位置。
    unsafe { vm_protect_pc(l, pc) };
  }

  /// 就地改写 `pc` 处指令主字的 C 字段（cachedslot 回填加速后续查找）。
  #[inline]
  pub(crate) fn patch_c(&self, pc: *const Instruction, slot: i32) {
    // Safety: 类型不变量保证 pc 指向正在执行的字节码主字（patch 目标由调用点算好）。
    unsafe { vm_patch_c(pc, slot) };
  }

  /// 常量表第 `i` 个 kv 常量地址（cpp `vmKV`）。
  #[inline]
  pub(crate) fn kv(&self, i: u32, cl: *mut Closure, k: *mut TValue) -> *mut TValue {
    // Safety: 类型不变量 + 调用点契约——i 为合法 AUX 常量下标，cl/k 为活闭包/常量表。
    unsafe { vm_kv(i, cl, k) }
  }

  /// `luaV_gettable`（可能经元方法；调用点须在 protect 内）。
  #[inline]
  pub(crate) fn gettable(&self, t: *const TValue, key: *mut TValue, val: StkId) {
    let l = self.l;
    // Safety: 类型不变量——l 为活 state；t/key/val 为存活 TValue / 界内栈槽（调用点契约）。
    unsafe { lua_v_gettable(l, t, key, val) };
  }

  /// `luaV_settable`（可能经元方法；调用点须在 protect 内）。
  #[inline]
  pub(crate) fn settable(&self, t: *const TValue, key: *mut TValue, val: StkId) {
    let l = self.l;
    // Safety: 同上。
    unsafe { lua_v_settable(l, t, key, val) };
  }

  /// `luaV_callTM`（调用点须在 protect 内，栈上已有 nargs 个待调值）。
  #[inline]
  pub(crate) fn call_tm(&self, nargs: c_int, nresults: c_int) {
    let l = self.l;
    // Safety: 类型不变量——l 为活 state；调用点保证 [top-nargs, top) 为可调值序列。
    unsafe { lua_v_call_tm(l, nargs, nresults) };
  }

  /// `lua_h_setstr`：按字符串键取表的可写值槽（可能重排哈希部分，调用点先 save_pc）。
  #[inline]
  pub(crate) fn set_str(&self, h: *mut LuaTable, key: *mut tstring) -> *mut TValue {
    let l = self.l;
    // Safety: 类型不变量——h 为活表、key 为活字符串（调用点断言过类型）。
    unsafe { lua_h_setstr(l, h, key) }
  }

  /// `luaH_resizearray`：扩展序列部分到 `last`（可能重分配数组，调用点先 save_pc）。
  #[inline]
  pub(crate) fn resize_array(&self, h: *mut LuaTable, last: i32) {
    let l = self.l;
    // Safety: 类型不变量——h 为活表，last 为正数组界（调用点由 index+c-1 算得）。
    unsafe { lua_h_resizearray(l, h, last) };
  }

  /// `gval2slot!`：活表内值槽 → 节点槽号（调用点保证槽来自该表）。
  #[inline]
  pub(crate) fn value_to_slot(&self, h: *mut LuaTable, value: *const TValue) -> i32 {
    // Safety: 类型不变量——value 为表 h 内节点值槽（lua_h_setstr/getstr 的返回值），
    // gval2slot! 仅做同表节点算术。
    unsafe { gval2slot!(h, value) }
  }

  /// `fastnotm!`：元表缓存判定「无该元方法」；`mt` 为 `None`（无元表）恒真。
  #[inline]
  pub(crate) fn fast_not_meta(&self, mt: Option<*mut LuaTable>, event: TMS) -> bool {
    // Safety: 类型不变量——mt 为 None 或活元表（fastnotm 契约把空元表视作「无 tm」，
    // Option 哨兵只在本边界还原为 vm 侧的 nullptr 形参）。
    unsafe { fastnotm(mt.unwrap_or(null_mut()), event) }
  }

  /// `fasttm!`：按枚举取元方法槽；缺失为 `None`（cpp nullptr 哨兵收进 Option）。
  /// 命中返回裸地址而非 `&TValue`：槽位属 GC 堆上的元表节点，慢路径随后可能跨
  /// 可重入调用（理由同 execute_gettableks 中对别名史的说明）。
  #[inline]
  pub(crate) fn meta_method(&self, mt: Option<*mut LuaTable>, event: TMS) -> Option<*const TValue> {
    let l = self.l;
    // Safety: 类型不变量——mt 为 None 或活元表；空元表还原为 fasttm 的 nullptr 形参。
    let tm = unsafe { fasttm(l, mt.unwrap_or(null_mut()), event) };
    (!tm.is_null()).then_some(tm)
  }

  /// `setobj2t!`：向 GC 目标表槽写值（调用点须在 save_pc 之后、界内）。
  #[inline]
  pub(crate) fn set_table_value(&self, dst: *mut TValue, src: *const TValue) {
    let l = self.l;
    // Safety: 调用点已证 dst 为表内可写槽、src 为存活 TValue。
    unsafe {
      setobj2t!(l, dst, src);
    }
  }

  /// `luaC_barriert!`：表写屏障。
  #[inline]
  pub(crate) fn barrier_table(&self, h: *mut LuaTable, v: *const TValue) {
    let l = self.l;
    // Safety: 类型不变量——h 为活表、v 为存活值（刚写入 h 的被写对象）。
    unsafe { luaC_barriert!(l, h, v) };
  }

  /// `lua_c_barrierfast!`：表写屏障（快速形态）。
  #[inline]
  pub(crate) fn barrier_fast(&self, h: *mut LuaTable) {
    let l = self.l;
    // Safety: 类型不变量——h 为活表。
    unsafe { lua_c_barrierfast!(l, h) };
  }

  /// `vm_protect!` 三参形态：记 savedpc=pc、执行 body（可能重入 VM/GC）。
  /// body 结束即返回；期间不得复用本帧以外缓存的槽地址。
  #[inline]
  pub(crate) fn protect(&self, pc: *const Instruction, body: impl FnOnce(&VmFrame)) {
    self.save_pc(pc);
    body(self);
  }

  /// `vm_protect!` 四参形态：在 `protect` 之后把 `L->base` 同步回本帧
  /// （被调方可能重定向栈基；随后经 `reg*` 重取槽位即得到新栈地址）。
  #[inline]
  pub(crate) fn protect_sync_base(
    &mut self,
    pc: *const Instruction,
    body: impl FnOnce(&mut VmFrame),
  ) {
    self.save_pc(pc);
    body(self);
    self.sync_base();
  }

  /// 把最新 `L->base` 同步回本帧（经可能重定位栈基的重入点、但无 savedpc 语义时使用，
  /// 如 `luaD_performCally` 之后；随后经 `reg*` 重取槽位即得到新栈地址）。
  #[inline]
  pub(crate) fn sync_base(&mut self) {
    self.base = self.lbase();
  }
}
