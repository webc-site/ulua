//! 寄存器解释器的栈槽视图门面（review.md §2「把 unsafe 关进有契约的最小边界」）。
//!
//! `luau_execute` 主循环以 `(*l).base` 为帧基址、以字节码 A/B/C 字段为寄存器号寻址
//! 栈槽（cpp `VM_REG!` 族宏的 Rust 移植）。本类型把「栈窗口切片、字节码字窗口、
//! vector 分量视图、字符串字节视图」等指针算术收敛为带安全契约的方法，解释器臂内
//! 不再手写 `from_raw_parts` / `*vb.add(n)`。
//! 与 `ulua-code-gen` 侧 `VmFrame`（JIT 慢路径门面）同一 idiom：指针只存在于本结构
//! 与其方法返回值中，unsafe 只出现在构造与类型化读写的最小实现处。
//!
//! 不变量（构造时成立、存活期间维持）：`l` 为执行中的存活 `LuaState`。各方法
//! 显式接收槽地址/表指针参数（由解释器臂上的 `VM_REG!` 等既有边界算得），因此
//! 跨栈重定位重入点（`vm_protect!`、`luaD_growstack`、C 调用等）后，调用方必须
//! 先按 VM 约定刷新局部 `base` 再重新取槽位，不得缓存旧的切片视图或槽地址。

use core::{
  ptr::from_ref,
  slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::{
  enums::tms::TMS,
  functions::lua_t_gettmbyobj::lua_t_gettmbyobj,
  macros::{fasttm::fasttm, getstr::getstr, lua_vector_size::LUA_VECTOR_SIZE},
  records::{lua_state::LuaState, lua_table::LuaTable, t_string::tstring},
  type_aliases::{instruction::Instruction, stk_id::StkId, t_value::TValue},
};

/// 一帧解释器执行上下文：活 `LuaState` 句柄（帧基址仍由循环局部量 `base` 持有，
/// 与本结构的 `(*l).base` 读取时点一致）。
#[derive(Clone, Copy)]
pub(crate) struct VmFrame {
  l: *mut LuaState,
}

impl VmFrame {
  /// 收下解释器循环的 `l`。构造仅存指针值、不解引用；`l` 为存活 `LuaState`
  /// （`ci`/`top`/`base`/`stack`/`stacksize` 一致有效）的前提由各解引用方法
  /// 在自己的调用边界按类型不变式论证，与本构造无关。
  #[inline]
  pub(crate) fn new(l: *mut LuaState) -> Self {
    Self { l }
  }

  /// `L->top` 写（调用点论证新值为分配栈内合法新栈顶，且不越 `stack_last`）。
  #[inline]
  pub(crate) fn set_top(&self, top: StkId) {
    let l = self.l;
    // Safety: 类型不变量——l 为活 state；top 由调用点按帧预留区论证（原语句即直接字段写）。
    unsafe { (*l).top = top };
  }

  /// 槽的元表（cpp `ttistable(ra) ? hvalue(ra)->metatable :
  /// ttisuserdata(ra) ? uvalue(ra)->metatable : nullptr` 展开）：
  /// 仅 table/userdata 可能携带，未携带（含槽类型不符与 `metatable` 字段为空）
  /// 一律 `None`。返回引用寿命与栈窗口解耦为独立 `'a`（GC 对象驻堆，寿命随
  /// GC 不变式而非门面值）。
  #[inline]
  pub(crate) fn slot_metatable<'a>(&self, slot: *const TValue) -> Option<&'a LuaTable> {
    // Safety: 类型不变量——slot 为存活 TValue；谓词分支内 as_table/as_userdata 为同址类型化读数，
    // metatable 字段的 null→None 归一由 Option::as_ref 完成。
    unsafe {
      if (*slot).is_table() {
        (*slot).as_table().metatable.as_ref()
      } else if (*slot).is_userdata() {
        (*slot).as_userdata().metatable.as_ref()
      } else {
        None
      }
    }
  }

  /// `fasttm(l, et, ev)` 的 Option 化：元表缺失（输入 `None`）或元方法缺失
  /// （gfasttm 空表短路 / tmcache 判定）均返回 `None`，与 cpp
  /// `fn = fasttm(...); fn && ttisfunction(fn)` 的判空链逐格同形。
  #[inline]
  pub(crate) fn fast_tm(&self, et: Option<&LuaTable>, ev: TMS) -> Option<*const TValue> {
    let l = self.l;
    // Safety: 类型不变量——l 活 state；et 由 Option 保证非空且指向存活元表，fasttm 只读其
    // tmcache/哈希区（升级 *mut 仅因 gfasttm 沿用 C 签名，不解写）。
    unsafe {
      et.map(|et| fasttm(l, from_ref(et).cast_mut(), ev))
        .filter(|tm| !tm.is_null())
    }
  }

  /// 「userdata 携带 C 元方法」快速路径三连判（算术族臂的
  /// `ttisuserdata(rb) && luaT_gettmbyobj(...) != nullptr && ttisfunction(tm)
  /// && clvalue(tm)->is_c`）：条件全满足时返回该元方法槽，否则 `None`。
  #[inline]
  pub(crate) fn c_tm_by_obj(&self, slot: *const TValue, ev: TMS) -> Option<*const TValue> {
    let l = self.l;
    // Safety: 类型不变量——slot 为存活栈槽（臂内 VM_REG! 算得）；gettmbyobj 永不返回空
    // （无元表时回落 nilobject），as_closure 先经 is_function() 谓词证明。
    unsafe {
      if !(*slot).is_userdata() {
        return None;
      }
      let tm = lua_t_gettmbyobj(l, slot, ev);
      ((*tm).is_function() && (*tm).as_closure().is_c != 0).then_some(tm)
    }
  }

  /// 「userdata 且其元表带 C 元方法」快速路径判（GETTABLEKS/SETTABLEKS 的
  /// `fasttm(uvalue(rb)->metatable, ev)` + 函数 + is_c 三连），对应 cpp
  /// `lvmexecute.cpp:602/740` 的 `fn = fasttm(...); fn && ttisfunction(fn) &&
  /// clvalue(fn)->isC`：fasttm 对无元表/未缓存命中的 userdata 返回空，判空后
  /// 才做 tag 读（缺省即 `None`，不解引用空指针）。
  #[inline]
  pub(crate) fn c_udata_tm(&self, slot: *const TValue, ev: TMS) -> Option<*const TValue> {
    let l = self.l;
    // Safety: 类型不变量——slot 为存活栈槽；as_userdata/fasttm/as_closure 各按其谓词前置。
    unsafe {
      if !(*slot).is_userdata() {
        return None;
      }
      let tm = fasttm(l, (*slot).as_userdata().metatable, ev);
      (!tm.is_null() && (*tm).is_function() && (*tm).as_closure().is_c != 0).then_some(tm)
    }
  }

  /// `(*L->global).mt[tt]`：类型注册元表，未注册为 `None`（cpp `nullptr` 的
  /// Option 归一，调用点无须哨兵比较）。
  #[inline]
  pub(crate) fn type_metatable<'a>(&self, tt: u32) -> Option<&'a LuaTable> {
    let l = self.l;
    // Safety: 类型不变量——l 活 state；`tt` 为 ttype!/LuaType 读得的合法下标
    // （< LUA_TCOUNT，global.mt 数组长度 14）；mt[tt] 的 null→None 归一由 as_ref 完成。
    unsafe { (*(*l).global).mt[tt as usize].as_ref() }
  }

  /// `(*L->global).mt[tt]` 类型元表上的 `fasttm` C 元方法三连判（GETTABLEKS
  /// vector 分支的 `fasttm(L, globalmt(LUA_TVECTOR), TM_INDEX)` + 函数 + is_c，
  /// cpp `lvmexecute.cpp:637-639`）。
  #[inline]
  pub(crate) fn type_metatable_c_tm(&self, tt: u32, ev: TMS) -> Option<*const TValue> {
    let l = self.l;
    // Safety: 类型不变量——l 活 state；global.mt[tt] 为全局类型元表数组合法下标
    // （调用点以 LuaType 取值），as_closure 先经判空与 is_function() 谓词证明。
    unsafe {
      let tm = fasttm(l, (*(*l).global).mt[tt as usize], ev);
      (!tm.is_null() && (*tm).is_function() && (*tm).as_closure().is_c != 0).then_some(tm)
    }
  }

  /// 栈上连续 `n` 槽的只读切片视图 `[slot, slot+n)`。
  ///
  /// # 使用契约
  /// 调用点论证 `slot..slot+n` 落在活栈数组界内（帧布局或显式 checkstack），
  /// 且切片存续期内不跨可能重分配栈的重入点。
  #[inline]
  pub(crate) fn slots(&self, slot: StkId, n: usize) -> &[TValue] {
    // Safety: 上述契约（同 ulua-code-gen VmFrame::slots）。
    unsafe { from_raw_parts(slot, n) }
  }

  /// 栈上连续 `n` 槽的可写切片视图 `[slot, slot+n)`（契约同 [`VmFrame::slots`]，
  /// 单线程 VM 串行执行下写权限独占成立）。返回寿命与 `&self` 门面解耦为独立
  /// `'s`（同 ulua-code-gen `VmFrame::slots_mut`）：切片实为栈区视图，寿命随栈而非门面值。
  #[inline]
  pub(crate) fn slots_mut<'s>(&self, slot: StkId, n: usize) -> &'s mut [TValue] {
    // Safety: 同 `slots`，且写权限独占由 VM 单线程调度保证。
    unsafe { from_raw_parts_mut(slot, n) }
  }

  /// 可读 `n` 个指令字的切片视图（起点由调用点给出）。
  ///
  /// # 使用契约
  /// `pc` 指向存活字节码数组内的合法位置且 `n` 个后续字均在该数组内
  /// （指令长度/捕获字数 <= sizecode，编译器保证）。
  #[inline]
  pub(crate) fn insns(&self, pc: *const Instruction, n: usize) -> &[Instruction] {
    // Safety: 上述契约（同 ulua-code-gen VmFrame::insns）。
    unsafe { from_raw_parts(pc, n) }
  }

  /// vector 槽的分量视图（调用点已以 `ttisvector!` 谓词证明；引用寿命随栈，
  /// 不得跨可能重分配栈的重入点使用）。
  #[inline]
  pub(crate) fn lanes(&self, slot: *const TValue) -> &[f32; LUA_VECTOR_SIZE as usize] {
    // Safety: 调用点已证 slot 内存有活 vector，as_vector_ref 为类型化读数。
    unsafe { (*slot).as_vector_ref() }
  }

  /// vector 槽第 `i` 分量读数（`i` 可为 3：`LUA_VECTOR_SIZE == 3` 构建下该分量
  /// 不存在，调用点只会在编译期死分支里引用它，永不执行；`== 4` 构建下语义
  /// 与 [`VmFrame::lanes`] 索引一致）。
  #[inline]
  pub(crate) fn lane_at(&self, slot: *const TValue, i: usize) -> f32 {
    // Safety: 调用点已证 slot 为活 vector；切片视图覆盖 LUA_VECTOR_SIZE 个分量。
    unsafe { (*slot).as_vector_ref() }
      .as_slice()
      .get(i)
      .copied()
      .unwrap_or(0.0)
  }

  /// 表序列部分 `[start, start+len)` 的可写切片视图（调用点已证区间落在
  /// `sizearray` 界内；区间存于堆上表内，寿命随 GC 对象，不得跨 rehash/resize 使用）。
  #[inline]
  pub(crate) fn array_window<'s>(
    &self,
    h: *mut LuaTable,
    start: usize,
    len: usize,
  ) -> &'s mut [TValue] {
    // Safety: 调用点以 sizearray 定界论证 array+start..+len 可写（SETLIST 的
    // luaH_resizearray 前置保证）。
    unsafe { from_raw_parts_mut((*h).array.add(start), len) }
  }

  /// `getstr` + `len`：字符串值字节视图（含 NUL 终止符，共 `len + 1` 字节；
  /// 调用点已以 `ttisstring!` 谓词证明槽内存有活 string）。
  #[inline]
  pub(crate) fn string_bytes(&self, ts: *const tstring) -> &[u8] {
    // Safety: getstr 契约保证可读 len+1 字节（同 ulua-code-gen VmFrame::string_bytes）。
    unsafe { from_raw_parts(getstr(ts).cast::<u8>(), (*ts).len as usize + 1) }
  }
}
