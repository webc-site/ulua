//! `TypeChecker2` / `NonStrictTypeChecker` 上下文裸指针字段的访问器收口：
//! `builtin_types` / `module` / `limits` / `ice` 等由构造方以 NonNull 语义建立
//! （C++ `NotNull<T>` 同契约），checker 存续期内恒有效；解引用统一收进这里，
//! 调用点不再出现 `unsafe { &*self.module }` 样板。`limits` / `subtyping`
//! 字段已于 #24 字段面句柄化，对应访问器退居纯转发（subtyping 兼做
//! `Option` 判空收敛点）。
use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes, module::Module,
  non_strict_type_checker::NonStrictTypeChecker, subtyping::Subtyping,
  type_check_limits::TypeCheckLimits, type_checker_2::TypeChecker2,
};
impl TypeChecker2 {
  /// C++ `NotNull<BuiltinTypes>`：内建类型表随前端上下文存活。
  pub fn builtin_types_ref(&self) -> &BuiltinTypes {
    self.builtin_types.get()
  }

  /// C++ `NotNull<Module>`：当前检查中的模块。
  pub fn module_ref(&self) -> &Module {
    // SAFETY: 见文件注释；非空与存活性由构造契约保证。
    unsafe { &*self.module }
  }

  /// [`module_ref`](Self::module_ref) 的可变形态：checker 独占模块内部状态。
  pub fn module_mut(&mut self) -> &mut Module {
    // SAFETY: 见文件注释；独占性由 `&mut self` 保证。
    unsafe { &mut *self.module }
  }

  /// C++ `NotNull<TypeCheckLimits>`：检查限额。字段已句柄化（#24 字段面），
  /// 解引用契约集中于 [`Handle`]。
  pub fn limits_ref(&self) -> &TypeCheckLimits {
    self.limits.get()
  }

  /// C++ `NotNull<Subtyping>` 成员（指向本结构体内嵌 `_subtyping`）的句柄形态：
  /// `wire_self_pointers` 构造期回填后恒 `Some`，`None` 属构造协议违例，
  /// 确定性 panic 而非 UB（与原裸指针解引用同一窗口，判空语义由 `Option` 承载）。
  pub fn subtyping_handle(&self) -> Handle<Subtyping> {
    self
      .subtyping
      .expect("subtyping 已由 `wire_self_pointers` 在构造期回填为 Some")
  }

  /// [`subtyping_handle`](Self::subtyping_handle) 的可变消费形态：等价于原
  /// 各调用点 `(*self.subtyping)` 重建 `&mut`，借用止于语句，单线程独占驱动
  /// （[`Handle::get_mut`] 的类型级契约）。返回借用刻意不钉死 `'a`，与原
  /// `&mut *ptr` 的借用检查行为完全同构（见 `records/arena_handle.rs`）。
  pub fn subtyping_mut<'a>(&mut self) -> &'a mut Subtyping {
    self.subtyping_handle().get_mut()
  }
}
impl NonStrictTypeChecker {
  /// C++ `NotNull<BuiltinTypes>`：内建类型表随前端上下文存活。
  pub fn builtin_types_ref(&self) -> &BuiltinTypes {
    self.builtin_types.get()
  }

  /// C++ `NotNull<Module>`：当前检查中的模块。
  pub fn module_ref(&self) -> &Module {
    // SAFETY: 见文件注释；非空与存活性由构造契约保证。
    unsafe { &*self.module }
  }
}
