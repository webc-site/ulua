use alloc::{boxed::Box, vec::Vec};
use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, instantiation::Instantiation,
    internal_error_reporter::InternalErrorReporter, module_resolver::ModuleResolverRef,
    normalizer::Normalizer, txn_log::TxnLog, type_checker::TypeChecker, type_level::TypeLevel,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};

impl TypeChecker {
  /// C++ `TypeChecker::TypeChecker(globalScope, resolver, builtinTypes, iceHandler)`
  /// (`Analysis/src/TypeInfer.cpp:205-231`)：成员初始化列表里
  /// `normalizer(nullptr, builtinTypes, NotNull{&unifierState}, SolverMode::Old)`
  /// 让 Normalizer 缓存一个指向同对象内 `unifierState` 的指针。上游之所以安全，
  /// 是因为 TypeChecker 由 `std::make_unique` 建在堆上且永不 move。
  ///
  /// Rust 侧若按值返回 `Self`，任何 move（NRVO 并非语言保证）都会让该自引用指针
  /// 指向已失效的旧槽位，故这里把对象定址在 Box 的堆槽里，再绑定指针：`Box` 不允许
  /// 把值搬出去，指针在整个生命周期内保持有效。
  ///
  /// # Safety
  /// 本构造函数把以下三个裸引用与 `ModuleResolverRef` 分派句柄原样存入返回的
  /// `Box<TypeChecker>`（对象自始
  /// 定址于 Box 堆槽、永不 move），各参数须满足：
  /// * `global_scope`：所引用的 `ScopePtr`（Arc）须比返回的 TypeChecker 长寿
  ///   ——checker 全程经 `*const ScopePtr` 解引用它（对应 C++ 按值拷贝的
  ///   `const ScopePtr&` 成员在此被裸指针化）。
  /// * `resolver`：[`ModuleResolverRef`] 分派句柄，其内部指针须由本会话存活的
  ///   `&mut FrontendModuleResolver` / `&mut NullModuleResolver` 经 `From` 构造
  ///   （C++ `ModuleResolver*`，检查期间单线程独占）。
  /// * `builtin_types`：指向存活 `BuiltinTypes` 的句柄（对应 C++
  ///   `NotNull<BuiltinTypes>`），构造期读取其内建 TypeId 字段后 checker 仍
  ///   会长期经该句柄访问，故其指向对象须比 checker 长寿且期间不被改写。
  /// * `ice_handler`：非空、对齐、指向存活 `InternalErrorReporter` 的指针
  ///   （C++ `InternalErrorReporter*`），须比 checker 及其内嵌 `unifier_state`
  ///   长寿。
  pub unsafe fn new(
    global_scope: &ScopePtr,
    resolver: ModuleResolverRef,
    builtin_types: Handle<BuiltinTypes>,
    ice_handler: *mut InternalErrorReporter,
  ) -> Box<TypeChecker> {
    let unifier_state = UnifierSharedState::new(ice_handler);

    // 依本函数 `# Safety` 契约，`builtin_types` 句柄指向存活、非空的
    // BuiltinTypes；构造阶段 checker 尚未诞生，不存在与之并存的其它借用。
    // 这里只以共享引用读取 13 个 Copy 的 TypeId/TypePackId 字段（值随拷贝
    // 脱离引用），借用止于 `Box::new` 之前；其后 checker 一律经自带的
    // `builtin_types` 句柄字段访问，不再延长此引用。
    let builtin = builtin_types.get();

    let mut result = Box::new(TypeChecker {
      global_scope: global_scope as *const ScopePtr,
      resolver,
      current_module: None,
      builtin_types,
      ice_handler,
      unifier_state,
      normalizer: Normalizer::new(
        None,          // arena：每次 check 前由 checkWithoutRecursionCheck 接线 internalTypes
        builtin_types, // 恒非空（C++ NotNull 形参）
        None,          // shared_state：待对象在 Box 里定址后接线（见函数文档）
        SolverMode::Old,
        false,
      ),
      reusable_instantiation: Instantiation::instantiation_new(
        TxnLog::empty(),
        None,
        builtin_types,
        TypeLevel::default(),
        null_mut(),
      ),
      require_cycles: Vec::new(),
      finish_time: None,
      instantiation_child_limit: None,
      unifier_iteration_limit: None,
      cancellation_token: None,
      prepare_module_scope: None,
      nil_type: builtin.nil_type,
      number_type: builtin.number_type,
      integer_type: builtin.integer_type,
      string_type: builtin.string_type,
      boolean_type: builtin.boolean_type,
      thread_type: builtin.thread_type,
      buffer_type: builtin.buffer_type,
      any_type: builtin.any_type,
      unknown_type: builtin.unknown_type,
      never_type: builtin.never_type,
      any_type_pack: builtin.any_type_pack,
      never_type_pack: builtin.never_type_pack,
      uninhabitable_type_pack: builtin.uninhabitable_type_pack,
      check_recursion_count: 0,
      recursion_count: 0,
      duplicate_type_aliases: DenseHashSet::default(),
      incorrect_extern_type_definitions: DenseHashSet::default(),
    });

    // 对象已在堆槽里定址、此后不再 move，这里绑定自引用指针才等价于上游
    // `NotNull{&unifierState}` 的原地构造。
    let this = &mut *result;
    this.normalizer.shared_state = Some(Handle::from_mut(&mut this.unifier_state));

    result
  }
}
