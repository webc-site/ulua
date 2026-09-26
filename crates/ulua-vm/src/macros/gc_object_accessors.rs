//! `GcObject` 10 个 GC 类型 × 5 种同构访问器壳（`as_X`/`as_X_mut`/`try_as_X`/
//! `try_as_X_ref`/`try_as_X_ptr`）与 `gclist` 读写对的单源工厂：每个类型只在调用
//! 点占一条目（方法名、类型名文案、union 字段、tag 判定、`LuaType` 路径、视图变
//! 体），函数体逐字同形收进宏体，签名与可变性语义与手写版逐点等价
//! （`try_as_*<'a>` 的 `'a` 仍由调用方指定，无界寿命语义不变）。union 读取一律
//! 先匹配 tag（`header().is_X()` 或 `lua_type()`）再触碰对应分支。类型条目列表
//! 与 `gclist` 链字段条目以 `=>` 分隔。仿
//! `lua_t_value.rs` 的 `gc_value_setter!` 先例：首行 doc 以 `#[doc = concat!(...)]`
//! 由类型名文案拼装（`拉丁类型名`条目自带前后空格，与手写版逐字符一致），共享
//! `# Safety` 段落收在宏体。本宏仅在 `records/gc_object.rs` 单点消费，宏体短名
//! 标识符（`GcObject`/`GcView`/`addr_of_mut` 等）按调用点作用域解析。
#[macro_export]
macro_rules! gc_object_accessors {
  ( $(
    $name:ident, $name_mut:ident, $try:ident, $try_ref:ident, $try_ptr:ident,
    $tn:expr, $ty:ty, $field:ident, $check:ident, $lt:path, $var:ident;
  )* => $( $lt_link:path => $field_link:ident, )* ) => {
    impl GcObject {
      $(
      #[doc = concat!("安全检查并转换为只读", $tn, "引用")]
      #[inline]
      pub fn $name(&self) -> Option<&$ty> {
        if self.header().$check() {
          Some(unsafe { &self.$field })
        } else {
          None
        }
      }

      #[doc = concat!("安全检查并转换为可变", $tn, "引用")]
      #[inline]
      pub fn $name_mut(&mut self) -> Option<&mut $ty> {
        if self.header().$check() {
          Some(unsafe { &mut self.$field })
        } else {
          None
        }
      }
      )*

      /// 获取只读类型解构视图
      #[inline]
      pub fn as_view(&self) -> Option<GcView<'_>> {
        match self.lua_type() {
          $( Some($lt) => Some(GcView::$var(unsafe { &self.$field })), )*
          _ => None,
        }
      }

      /// 获取可变类型解构视图
      #[inline]
      pub fn as_view_mut(&mut self) -> Option<GcViewMut<'_>> {
        match self.lua_type() {
          $( Some($lt) => Some(GcViewMut::$var(unsafe { &mut self.$field })), )*
          _ => None,
        }
      }
    }

    $(
    #[doc = concat!("若裸指针非空且类型匹配，解构为可变", $tn, "引用")]
    ///
    /// # Safety
    /// 若 `gco` 非空，必须指向有效存活的 `GcObject` 内存，且生命周期 `'a` 内不发生别名冲突。
    #[inline]
    pub unsafe fn $try<'a>(gco: *mut GcObject) -> Option<&'a mut $ty> {
      if gco.is_null() {
        None
      } else {
        unsafe { (*gco).$name_mut() }
      }
    }

    #[doc = concat!("若裸指针非空且类型匹配，解构为只读", $tn, "引用")]
    ///
    /// # Safety
    /// 若 `gco` 非空，必须指向有效存活的 `GcObject` 内存。
    #[inline]
    pub unsafe fn $try_ref<'a>(gco: *const GcObject) -> Option<&'a $ty> {
      if gco.is_null() {
        None
      } else {
        unsafe { (*gco).$name() }
      }
    }

    #[doc = concat!("若裸指针非空且类型匹配，返回", $tn, "对象的原始指针")]
    ///
    /// # Safety
    /// 若 `gco` 非空，必须指向有效存活且头字段可读的 `GcObject` 内存。
    #[inline]
    pub unsafe fn $try_ptr(gco: *mut GcObject) -> Option<*mut $ty> {
      if gco.is_null() {
        None
      } else if unsafe { (*gco).header().$check() } {
        Some(unsafe { addr_of_mut!((*gco).$field) as *mut $ty })
      } else {
        None
      }
    }
    )*

    impl GcObject {
      /// 读取具有 `gclist` 侵入式链字段的对象的下一个 GC 节点
      ///
      /// # Safety
      /// 调用方须保证 `self` 为存活且其类型标签与 union 变体相符的 `GcObject`。
      #[inline]
      pub unsafe fn gclist(&self) -> Option<*mut GcObject> {
        match self.lua_type() {
          $( Some($lt_link) => Some(unsafe { (*self.$field_link).gclist }), )*
          _ => None,
        }
      }

      /// 设置具有 `gclist` 侵入式链字段的对象的下一个 GC 节点
      ///
      /// # Safety
      /// 调用方须保证 `self` 为存活且其类型标签与 union 变体相符的 `GcObject`，且 `gclist` 字段可写。
      #[inline]
      pub unsafe fn set_gclist(&mut self, next: *mut GcObject) -> bool {
        match self.lua_type() {
          $(
            Some($lt_link) => {
              unsafe { (*self.$field_link).gclist = next };
              true
            }
          )*
          _ => false,
        }
      }
    }
  };
}

pub use gc_object_accessors;
