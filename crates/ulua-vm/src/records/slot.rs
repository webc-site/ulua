//! B2-0 槽句柄原语（`StkId` 槽的类型化门面；设计票 `.qoder/audit/b2-slot-handle-design.md` 选型 c）。
//!
//! [`Slot`] 是裸 [`StkId`]（= `*mut TValue`）在 API 表层之上的收口句柄：`NonNull` newtype
//! 薄封装，不改变槽地址语义——构造与读写面全部 `inline(always)` 指针平移/解引用，无空检
//! 分支（`from_raw` 用 `new_unchecked`，契约要求非空），codegen 与既有裸解引用等价（§9.4
//! 热路径承诺）。`'a` 参数只承载「槽借用由何处的存活期派生」这一契约信息：栈可被
//! `ensure_stack` 移动，一切跨调用的槽借用必须「扩容先行、借用后派生」（唯一权威表述见
//! `records/lua_state/stack.rs` 的 `push_slot_with` 契约）。
//!
//! **别名纪律（沿用既有 lvmexecute 约定）**：同帧 `Slot` 间无 Rust 别名保证——`Slot` 为
//! `Copy`，同时持有两个 [`Slot::as_mut`]（或经 [`Slot::from_raw`] 派生的交叠可变视图）属
//! UB，与现 C++ 移植的裸指针语义一致；同帧各句柄的使用窗口由 VM 单线程串行执行纪律排序，
//! 本类型既不引入新的别名保证，也不移除既有保证。
//!
//! 构造只经有契约的原语，unsafe 恰关在本模块 impl 的最小块内：
//! - [`LuaState::slot`]：api 索引域换算（与 `index_2_addr` 同一契约与语义）；
//! - [`FrameView::reg`]：由当前帧 `base` 派生，[`FrameView::current`] 的独占借用锚定存活期；
//! - [`Slot::from_mut`]：局部独占可写 TValue（safe 构造子）；
//! - [`Slot::from_ref`]：只读侧共享 TValue——此后置句柄只允许读面；
//! - [`Slot::from_raw`]：裸指针边界重建（C ABI 导出壳、JIT 帧门面、解释器臂上的
//!   `VM_REG!`/`VM_KV!` 槽地址）；
//! - [`Slot::get`]/[`Slot::as_mut`]/[`Slot::set`]：读写出入面（只读视图取 `get`，
//!   命名对照 cpp 槽读数惯例；不取名 `as_ref` 以避免与 `std::convert::AsRef`
//!   trait 方法混淆，红线禁止新增 lint 豁免）。
//!
//! 边界不变量（设计票红线）：`CallInfo` 裸字段与帧内算术不落句柄；本类型只在 API 表层
//! 收口，字段落库仍用裸 `StkId`。
//!
//! [`StkId`]: crate::type_aliases::stk_id::StkId

use core::{marker::PhantomData, ptr::NonNull};

use crate::{
  functions::index_2_addr::index_2_addr, records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// 栈 TValue 槽的类型化句柄（寿命 `'a` 锚定「扩容先行、借用后派生」不变量）。
///
/// 别名纪律见模块文档：同帧句柄间无 Rust 别名保证。
#[derive(Clone, Copy)]
pub struct Slot<'a> {
  ptr: NonNull<TValue>,
  _life: PhantomData<&'a mut TValue>,
}

impl<'a> Slot<'a> {
  /// 最低层边界构造子：把已论证有效的槽指针收口为句柄。
  ///
  /// # Safety
  /// `ptr` 非空、对齐，指向在 `'a` 存续期内至少可读的 `TValue`；写面
  /// （[`Slot::as_mut`]/[`Slot::set`]）仅当该内存可独占写入时方可使用。跨调用
  /// 期间 `ptr` 的存活须服从「扩容先行、借用后派生」——栈重分配后旧句柄失效。
  #[inline(always)]
  pub unsafe fn from_raw(ptr: *mut TValue) -> Self {
    Self {
      // SAFETY: 本方法契约保证 `ptr` 非空且对齐可读；`new_unchecked` 不做空检，
      // 保持与裸指针等价的 codegen（§9.4）。
      ptr: unsafe { NonNull::new_unchecked(ptr) },
      _life: PhantomData,
    }
  }

  /// 只读侧构造子：由共享引用派生句柄（对应 cpp 形参 `const TValue*` 位）。
  ///
  /// # Safety
  /// 由此构造的句柄只允许读面（[`Slot::get`]/[`Slot::as_const_ptr`]）；
  /// 仅当能证明 `r` 源自独占可写内存时才允许写面，否则经 [`Slot::from_mut`] 构造。
  #[inline(always)]
  pub unsafe fn from_ref(r: &'a TValue) -> Self {
    Self {
      // SAFETY: 共享引用保证非空、对齐、`'a` 内存活；写面限制见本方法契约。
      ptr: unsafe { NonNull::new_unchecked(r as *const TValue as *mut TValue) },
      _life: PhantomData,
    }
  }

  /// 独占写侧构造子：由独占可变借用派生句柄，读写面全开，safe 构造。
  #[inline(always)]
  pub fn from_mut(r: &'a mut TValue) -> Self {
    Self {
      ptr: NonNull::from(r),
      _life: PhantomData,
    }
  }

  /// 只读视图（设计票 `get`/`as_ref` 读面合一：返回 `&'a TValue`，等价原 `&*p`）。
  ///
  /// 命名说明：inherent `as_ref` 触发 clippy `should_implement_trait`（与
  /// `std::convert::AsRef::as_ref` 混淆），红线禁止新增豁免，故读面取名 `get`。
  #[inline(always)]
  pub fn get(self) -> &'a TValue {
    // SAFETY: 构造子契约保证 `ptr` 指向 `'a` 内存活的对齐 TValue，只读解引用成立。
    unsafe { self.ptr.as_ref() }
  }

  /// 独占写视图（等价原 `&mut *p`；同帧别名纪律见模块文档）。
  ///
  /// # Safety 语境注记
  /// 本方法自身 safe 但把写权限交给调用方：仅当句柄源自独占可写内存（[`Slot::from_mut`]
  /// /可写栈槽的 [`Slot::from_raw`]）时调用，否则违反 Rust 别名规则。
  #[inline(always)]
  pub fn as_mut(self) -> &'a mut TValue {
    // SAFETY: 写权限前提由本方法文档约束调用方（句柄源自独占可写内存）；
    // 构造子契约保证对齐与非空。
    unsafe { &mut *self.ptr.as_ptr() }
  }

  /// 值写入（裸值拷贝，等价原 `*p = *v`）。
  ///
  /// # Safety
  /// 句柄须源自独占可写内存（[`Slot::as_mut`] 纪律）；本方法不含 GC 屏障——屏障
  /// 时机属调用侧纪律（对照 `lua_v_settable` 的「值落槽后 `luaC_barriert!`」前提），
  /// 本方法不发明收敛。
  #[inline(always)]
  pub unsafe fn set(self, v: &TValue) {
    // 写路径经 [`Slot::as_mut`]（其写权限契约由句柄来源论证，见该方法文档）；
    // `v` 为调用方持有的存活只读引用。
    *self.as_mut() = *v;
  }

  /// 裸指针读出（宏/C ABI 边界用；等价原句柄所指地址，不引入算术）。
  #[inline(always)]
  pub fn as_ptr(self) -> *mut TValue {
    self.ptr.as_ptr()
  }

  /// 只读裸指针读出（cpp `const TValue*` 形参位边界用）。
  #[inline(always)]
  pub fn as_const_ptr(self) -> *const TValue {
    self.ptr.as_ptr().cast_const()
  }
}

/// api 索引域构造子（设计票形态）：先做 `index_2_addr` 换算、后派生句柄借用，
/// 返回寿命锚定 `self` 的独占借用——句柄存活期内不得再独占重入本 `LuaState`，
/// 这是「扩容先行、借用后派生」不变量的 Rust 侧强制。
impl LuaState {
  /// 由 api 索引 `idx` 取当前帧槽句柄（语义与 `index_2_addr` 对齐）。
  ///
  /// # Safety
  /// 与 [`index_2_addr`] 同一契约：`idx` 为合法 Lua 栈索引（正数不超帧顶、负数在
  /// `LUA_REGISTRYINDEX` 之上、伪索引走 `pseudo_2_addr`）；越界正索引按其语义落入
  /// `LUA_O_NILOBJECT` 静态 nil 槽，该槽只允许句柄读面。返回句柄仅在栈未重分配前有效。
  #[inline(always)]
  pub unsafe fn slot(&mut self, idx: i32) -> Slot<'_> {
    // SAFETY: `idx`/帧栈界按本方法契约原样供出（与既有 `index_2_addr` 调用点同前提）；
    // 换算所得槽地址在 `self` 的借用期内存活，派生句柄即锚定该借用。
    unsafe { Slot::from_raw(index_2_addr(self, idx)) }
  }
}

/// 当前帧槽视图：`Slot<'a>` 的帧存活期锚定器。
///
/// [`FrameView::current`] 收下 `&'a mut LuaState` 的独占借用后，[`FrameView::reg`]
/// 由当前帧 `base`（`(*l).base` 与 `ci->base` 为 VM 同构镜像）派生 `Slot<'a>`；
/// 句柄与视图共享同一 `'a` 锚——句柄存活期内该 `LuaState` 的独占使用被借用检查
/// 禁止，扩容类调用必须先于句柄派生完成。
pub struct FrameView<'a> {
  l: &'a mut LuaState,
}

impl<'a> FrameView<'a> {
  /// 收下当前帧视图：仅存借用、不解引用、无 unsafe。
  #[inline(always)]
  pub fn current(l: &'a mut LuaState) -> Self {
    Self { l }
  }

  /// 当前帧寄存器槽 `base[i]` 的句柄。
  ///
  /// # Safety
  /// `l` 为执行中的存活 `LuaState` 且 `ci` 已接线（活动 CallInfo 不变量），
  /// `base` 落在分配栈界内；`i` 为帧内合法寄存器索引：`base + i` 不越
  /// `top..ci->top` 的分配栈窗口。同帧重复派生的句柄间无 Rust 别名保证（模块
  /// 文档纪律），使用窗口由调用方按 lvmexecute 串行纪律排序。
  #[inline(always)]
  pub unsafe fn reg(&self, i: i32) -> Slot<'a> {
    // SAFETY: 本方法契约——`ci` 已接线、`self.l.base` 在分配栈界内，`base + i`
    // 为界内槽地址；这是句柄派生的唯一一处指针算术。
    unsafe { Slot::from_raw(self.l.base.add(i as usize)) }
  }
}
