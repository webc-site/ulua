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
#[macro_export]
macro_rules! tagged_slot_accessors {
  () => {};
  ( gc $name:ident, $name_mut:ident, $name_ptr:ident,
    $ty:ty, $ptr_ty:ty, $field:ident, $check:ident,
    $doc:expr, $doc_mut:expr, $doc_ptr:expr ;
    $($rest:tt)* ) => {
    #[doc = $doc]
    ///
    /// # Safety
    /// 调用方须确保 self 的类型标签与所读 GC 分支相符（对应 `is_*` 判定为真），
    /// 且所指 `GcObject` 存活。
    #[inline]
    pub unsafe fn $name(&self) -> &$ty {
      debug_assert!(self.$check());
      unsafe { &(*self.value.gc).$field }
    }

    #[doc = $doc_mut]
    ///
    /// # Safety
    #[doc = concat!("同 [`Self::", stringify!($name), "`]，且可变借用要求 `&mut` 独占、无别名冲突。")]
    #[inline]
    pub unsafe fn $name_mut(&mut self) -> &mut $ty {
      debug_assert!(self.$check());
      unsafe { &mut (*self.value.gc).$field }
    }

    #[doc = $doc_ptr]
    ///
    /// # Safety
    #[doc = concat!("同 [`Self::", stringify!($name), "`]。")]
    #[inline]
    pub unsafe fn $name_ptr(&self) -> $ptr_ty {
      unsafe { ::core::ptr::from_ref(self.$name()).cast_mut() }
    }

    $crate::tagged_slot_accessors! { $($rest)* }
  };
  ( gc_ro $name:ident, $name_ptr:ident,
    $ty:ty, $ptr_ty:ty, $field:ident, $check:ident,
    $doc:expr, $doc_ptr:expr ;
    $($rest:tt)* ) => {
    #[doc = $doc]
    ///
    /// # Safety
    /// 调用方须确保 self 的类型标签与所读 GC 分支相符（对应 `is_*` 判定为真），
    /// 且所指 `GcObject` 存活。
    #[inline]
    pub unsafe fn $name(&self) -> &$ty {
      debug_assert!(self.$check());
      unsafe { &(*self.value.gc).$field }
    }

    #[doc = $doc_ptr]
    ///
    /// # Safety
    #[doc = concat!("同 [`Self::", stringify!($name), "`]。")]
    #[inline]
    pub unsafe fn $name_ptr(&self) -> $ptr_ty {
      unsafe { ::core::ptr::from_ref(self.$name()).cast_mut() }
    }

    $crate::tagged_slot_accessors! { $($rest)* }
  };
  ( gc_cptr $name:ident, $name_mut:ident, $name_ptr:ident,
    $ty:ty, $ptr_ty:ty, $field:ident, $check:ident,
    $doc:expr, $doc_mut:expr, $doc_ptr:expr ;
    $($rest:tt)* ) => {
    #[doc = $doc]
    ///
    /// # Safety
    /// 调用方须确保 self 的类型标签与所读 GC 分支相符（对应 `is_*` 判定为真），
    /// 且所指 `GcObject` 存活。
    #[inline]
    pub unsafe fn $name(&self) -> &$ty {
      debug_assert!(self.$check());
      unsafe { &(*self.value.gc).$field }
    }

    #[doc = $doc_mut]
    ///
    /// # Safety
    #[doc = concat!("同 [`Self::", stringify!($name), "`]，且可变借用要求 `&mut` 独占、无别名冲突。")]
    #[inline]
    pub unsafe fn $name_mut(&mut self) -> &mut $ty {
      debug_assert!(self.$check());
      unsafe { &mut (*self.value.gc).$field }
    }

    #[doc = $doc_ptr]
    ///
    /// # Safety
    #[doc = concat!("同 [`Self::", stringify!($name), "`]。")]
    #[inline]
    pub unsafe fn $name_ptr(&self) -> $ptr_ty {
      unsafe { ::core::ptr::from_ref(self.$name()) }
    }

    $crate::tagged_slot_accessors! { $($rest)* }
  };
  ( scalar $name:ident, $ty:ty, $field:ident, $check:ident, $doc:expr ;
    $($rest:tt)* ) => {
    #[doc = $doc]
    #[inline]
    pub fn $name(&self) -> $ty {
      debug_assert!(self.$check());
      unsafe { self.value.$field }
    }

    $crate::tagged_slot_accessors! { $($rest)* }
  };
  ( scalar_bool $name:ident, $field:ident, $check:ident, $doc:expr ;
    $($rest:tt)* ) => {
    #[doc = $doc]
    #[inline]
    pub fn $name(&self) -> bool {
      debug_assert!(self.$check());
      unsafe { self.value.$field != 0 }
    }

    $crate::tagged_slot_accessors! { $($rest)* }
  };
}

pub use tagged_slot_accessors;
