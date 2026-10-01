//! `TValue` 与 `TKey` 双镜像 tagged-slot 访问器（`as_*` 系列）的单源工厂：
//! 两类型同为 `value + extra` 载荷 + tag 的槽位布局，GC 引用三件套
//! （只读借用 / 可变借用 / 裸指针）与标量读取族的函数体逐字同形，唯余方法名、
//! 载荷类型、union 字段、tag 判定方法与首行 doc——doc 作为属性经
//! `#[doc = $doc]` 原样保留（cpp 出处锚点留在各调用点实参），共享 `# Safety`
//! 段落收进宏体（仿 `gc_value_setter!` 先例）。条目以关键字开头选择规则臂：
//! `gc`（借用 + 可变借用 + `*mut` 裸指针）、`gc_ro`（无可变借用形态，如
//! TString）、`gc_cptr`（裸指针为 `*const` 返回，如 Udata）、`scalar`（透传
//! `value.$field`）、`scalar_bool`（`value.$field != 0` 收敛为 `bool`）。
//! `as_vector`/`as_vector_ref`（跨 value+extra 裸写形状）与 `set_*` 族不在本宏
//! 范围。宏在 `impl` 体内调用，递归经 `$crate` 路径收束。
//!
//! unsafe 收敛结构（review §2「每布局操作一个最小 helper」）：
//! - `@layout` 臂在每个宿主 impl 恰好展开一次，产出仅有的两个裸指针解引用收口
//!   `gc_obj`/`gc_obj_mut`（`value.gc` 读 + deref）；
//! - `@read`/`@read_mut`/`@read_ptr_mut`/`@read_ptr_const` 臂各自只承载一种布局（裸指针臂由 `value.gc` 直接 `addr_of` 派生，不经引用中转）
//!   操作（只读 union 字段 / 可变 union 字段 / 引用派生裸指针），`gc`/`gc_ro`/
//!   `gc_cptr` 三条目臂纯做组合，同一操作体在全宏中只出现一次；
//! - `scalar`/`scalar_bool` 臂本就是 safe fn（`Value` 全成员 Copy、构造即初始化，
//!   union 标量读恒有定义，收敛在宏体单点）。
//!
//! 安全门面评估（review §2）：本宏的消费方（解释器臂、lapi 族）一律以
//! `StkId`/裸槽指针寻址——`(*slot).as_X()` 的解引用本身即不可消除的 unsafe，
//! 引用入参门面无法减少任何调用点 unsafe 块，故不增设 Option 门面（tag 检查 +
//! `Option<&T>` 形态已由 `gc_object_accessors!` 生成的 `GcObject::as_X` 族单源
//! 承担）；复合判据中有真实业务消费方的（`ttisfunction && clvalue->isC`）下沉为
//! safe 方法 `TValue::is_c_closure`，`iscfunction!` 宏转发之。
#[macro_export]
macro_rules! tagged_slot_accessors {
  // —— 布局原语：每个宿主 impl 恰好展开一次（裸指针 deref 的唯一收口）——
  ( @layout ) => {
    /// tagged-slot GC 分支的裸指针解引用原语：本宏生成的一切 GC 引用/指针访问器
    /// 经此单点取得 `&GcObject`，`value.gc` 的读与 deref 只发生在此处。
    ///
    /// # Safety
    /// 调用方须确保 self 的类型标签与 GC 分支相符（对应 `is_*` 判定为真），且
    /// `value.gc` 指向存活、按 `GcObject` 对齐的内存（gc 指针写入方保证其有效）。
    #[inline(always)]
    unsafe fn gc_obj(&self) -> &$crate::records::gc_object::GcObject {
      // SAFETY: 契约由本方法 `# Safety` 收敛保证；union 字段 `gc` 为 Copy 裸指针读
      unsafe { &*self.value.gc }
    }

    /// [`Self::gc_obj`] 的可变形态原语。
    ///
    /// # Safety
    /// 同 [`Self::gc_obj`]，且要求 `&mut self` 独占、无别名冲突。
    #[inline(always)]
    unsafe fn gc_obj_mut(&mut self) -> &mut $crate::records::gc_object::GcObject {
      // SAFETY: 契约由本方法 `# Safety` 收敛保证
      unsafe { &mut *self.value.gc }
    }
  };

  // —— 布局操作单点：只读 GC 字段借用 ——
  ( @read $name:ident, $ty:ty, $field:ident, $check:ident, $doc:expr ) => {
    #[doc = $doc]
    ///
    /// # Safety
    /// 调用方须确保 self 的类型标签与所读 GC 分支相符（对应 `is_*` 判定为真），
    /// 且所指 `GcObject` 存活。
    #[inline(always)]
    pub unsafe fn $name(&self) -> &$ty {
      debug_assert!(self.$check());
      // SAFETY: 契约同本方法 `# Safety`；deref 收口于 `gc_obj`，此处仅按 tag
      // 相符前提读取 union 对应分支
      unsafe { &self.gc_obj().$field }
    }
  };

  // —— 布局操作单点：可变 GC 字段借用 ——
  ( @read_mut $name_mut:ident, $name_ro:ident, $ty:ty, $field:ident, $check:ident, $doc:expr ) => {
    #[doc = $doc]
    ///
    /// # Safety
    #[doc = concat!("同 [`Self::", stringify!($name_ro), "`]，且可变借用要求 `&mut` 独占、无别名冲突。")]
    #[inline(always)]
    pub unsafe fn $name_mut(&mut self) -> &mut $ty {
      debug_assert!(self.$check());
      // SAFETY: 契约同本方法 `# Safety`；deref 收口于 `gc_obj_mut`
      unsafe { &mut self.gc_obj_mut().$field }
    }
  };

  // —— 布局操作单点：由 `value.gc` 裸指针直接派生 `*mut` ——
  ( @read_ptr_mut $name_ptr:ident, $name_ro:ident, $ptr_ty:ty, $field:ident, $check:ident, $doc:expr ) => {
    #[doc = $doc]
    ///
    /// # Safety
    #[doc = concat!("同 [`Self::", stringify!($name_ro), "`]。")]
    #[inline(always)]
    pub unsafe fn $name_ptr(&self) -> $ptr_ty {
      debug_assert!(self.$check());
      // SAFETY: 契约同本方法 `# Safety`。字段地址直接由 `value.gc` 裸指针算得，
      // `addr_of_mut!` 不在堆对象上建借用，故所得指针可被调用方合法解写
      // （如 `lua_setmetatable` 写 `metatable`、字节码加载写串原子态）。
      // 旧写法先把堆对象借成 `&` 再 `cast_mut`，同一处写即经共享引用解写的 UB。
      // `ManuallyDrop` 为 `repr(transparent)`，`.cast()` 后同址同布局。
      unsafe { ::core::ptr::addr_of_mut!((*self.value.gc).$field).cast() }
    }
  };

  // —— 布局操作单点：由 `value.gc` 裸指针直接派生 `*const` ——
  ( @read_ptr_const $name_ptr:ident, $name_ro:ident, $ptr_ty:ty, $field:ident, $check:ident, $doc:expr ) => {
    #[doc = $doc]
    ///
    /// # Safety
    #[doc = concat!("同 [`Self::", stringify!($name_ro), "`]。")]
    #[inline(always)]
    pub unsafe fn $name_ptr(&self) -> $ptr_ty {
      debug_assert!(self.$check());
      // SAFETY: 同上（`addr_of!` 不建借用；本臂返回 `*const`，写方须自行
      // `cast_mut`，其前提是该堆对象本就归 VM 可变持有）
      unsafe { ::core::ptr::addr_of!((*self.value.gc).$field).cast() }
    }
  };

  // —— muncher：逐条目组合上述单点操作 ——
  ( @munch ) => {};
  ( @munch gc $name:ident, $name_mut:ident, $name_ptr:ident,
    $ty:ty, $ptr_ty:ty, $field:ident, $check:ident,
    $doc:expr, $doc_mut:expr, $doc_ptr:expr ;
    $($rest:tt)* ) => {
    $crate::tagged_slot_accessors! { @read $name, $ty, $field, $check, $doc }
    $crate::tagged_slot_accessors! { @read_mut $name_mut, $name, $ty, $field, $check, $doc_mut }
    $crate::tagged_slot_accessors! { @read_ptr_mut $name_ptr, $name, $ptr_ty, $field, $check, $doc_ptr }
    $crate::tagged_slot_accessors! { @munch $($rest)* }
  };
  ( @munch gc_ro $name:ident, $name_ptr:ident,
    $ty:ty, $ptr_ty:ty, $field:ident, $check:ident,
    $doc:expr, $doc_ptr:expr ;
    $($rest:tt)* ) => {
    $crate::tagged_slot_accessors! { @read $name, $ty, $field, $check, $doc }
    $crate::tagged_slot_accessors! { @read_ptr_mut $name_ptr, $name, $ptr_ty, $field, $check, $doc_ptr }
    $crate::tagged_slot_accessors! { @munch $($rest)* }
  };
  ( @munch gc_cptr $name:ident, $name_mut:ident, $name_ptr:ident,
    $ty:ty, $ptr_ty:ty, $field:ident, $check:ident,
    $doc:expr, $doc_mut:expr, $doc_ptr:expr ;
    $($rest:tt)* ) => {
    $crate::tagged_slot_accessors! { @read $name, $ty, $field, $check, $doc }
    $crate::tagged_slot_accessors! { @read_mut $name_mut, $name, $ty, $field, $check, $doc_mut }
    $crate::tagged_slot_accessors! { @read_ptr_const $name_ptr, $name, $ptr_ty, $field, $check, $doc_ptr }
    $crate::tagged_slot_accessors! { @munch $($rest)* }
  };
  ( @munch scalar $name:ident, $ty:ty, $field:ident, $check:ident, $doc:expr ;
    $($rest:tt)* ) => {
    #[doc = $doc]
    #[inline(always)]
    pub fn $name(&self) -> $ty {
      debug_assert!(self.$check());
      // SAFETY: `Value` 全成员为 Copy 且无 Drop，构造即全零初始化，任一标量字段
      // 读恒有定义；tag 相符性由 debug_assert 守护（tag 不符读的是残值、非 UB，
      // 与 cpp `nvalue`/`lvalue` 同形）
      unsafe { self.value.$field }
    }

    $crate::tagged_slot_accessors! { @munch $($rest)* }
  };
  ( @munch scalar_bool $name:ident, $field:ident, $check:ident, $doc:expr ;
    $($rest:tt)* ) => {
    #[doc = $doc]
    #[inline(always)]
    pub fn $name(&self) -> bool {
      debug_assert!(self.$check());
      // SAFETY: 同 `scalar` 臂（Copy 标量字段读）
      unsafe { self.value.$field != 0 }
    }

    $crate::tagged_slot_accessors! { @munch $($rest)* }
  };

  // —— 公开入口（置于最后一条臂）：先布局原语、后条目 muncher ——
  ( $($entries:tt)* ) => {
    $crate::tagged_slot_accessors! { @layout }
    $crate::tagged_slot_accessors! { @munch $($entries)* }
  };
}

pub use tagged_slot_accessors;
