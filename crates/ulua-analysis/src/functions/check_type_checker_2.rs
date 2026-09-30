//! C++ free function `Luau::check(NotNull<BuiltinTypes>,
//! NotNull<TypeFunctionRuntime>, NotNull<UnifierSharedState>,
//! NotNull<TypeCheckLimits>, DcrLogger*, const SourceModule&, Module*)`
//! (`Analysis/src/TypeChecker2.cpp:286-305`).
use core::ptr::NonNull;

use crate::{
  functions::{copy_errors::copy_errors, freeze::freeze, unfreeze::unfreeze},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, dcr_logger::DcrLogger, module::Module,
    source_module::SourceModule, type_check_limits::TypeCheckLimits, type_checker_2::TypeChecker2,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
};

/// 对应 C++ `Luau::check`（TypeChecker2.cpp:286-305）。基础设施形参已收窄为受检
/// 引用/`Option` 引用（`builtinTypes/runtime/state/limits/logger` 的 NotNull 与
/// 可空 `DcrLogger*` 语义由类型编码），剩余裸指针仅 `module`（C++ `Module*`
/// 直译）：非空、地址稳定的 Arc 写穿句柄且比本次检查长寿，经 checked `NonNull`
/// 物化，null 输入由 UB 收敛为 panic；`source_module.root` 的非空 arena 块指针
/// 前提由 parser 构造点保证，解析 arena 在调用期间存活。
pub fn check(
  builtin_types: Handle<BuiltinTypes>,
  type_function_runtime: &mut TypeFunctionRuntime,
  unifier_state: &mut UnifierSharedState,
  limits: &mut TypeCheckLimits,
  logger: Option<&mut DcrLogger>,
  source_module: &SourceModule,
  module: *mut Module,
) {
  // TypeChecker2 typeChecker{builtinTypes, typeFunctionRuntime, unifierState, limits, logger, &sourceModule, module};
  // 「裸构造 → 自指针回填 → 裸指针外露」的悬置窗口收进 `new_boxed` 封闭
  // 门面，调用点拿到即布线完成的 `Box` 安全句柄。
  let mut module_nn = NonNull::new(module).expect("module 写穿句柄非空（cpp Module* 直译）");

  // SAFETY: module_nn 由上方判空取得，地址稳定且比本次检查长寿（本函数契约原样
  // 透传给 new_boxed），其余实参为受检引用。
  let mut type_checker = unsafe {
    TypeChecker2::new_boxed(
      builtin_types,
      type_function_runtime,
      unifier_state,
      limits,
      logger,
      source_module,
      module_nn.as_ptr(),
    )
  };

  // typeChecker.visit(sourceModule.root);
  // SAFETY: root 为 parser 在 arena 上建立的 AstStatBlock 节点，检查期内
  // 地址稳定且存活，转共享引用符合 visit_stat_block 的节点存活契约。
  unsafe { type_checker.visit_stat_block(&*source_module.root) };

  // unfreeze(module->interfaceTypes);
  // copyErrors(module->errors, module->interfaceTypes, builtinTypes);
  // freeze(module->interfaceTypes);
  // Safety: `module` 非空且比本次检查长寿（本函数契约）；checker 的 visit
  // 已在上一行结束，其经构造存下的 module 裸指针此后不再被解引用，故此处
  // 物化独占借用无在册别名，且检查器此刻已停止使用这些字段；块内
  // unfreeze/copy_errors/freeze 单线程串行、借用先后不重叠。
  unsafe {
    let m = module_nn.as_mut();
    unfreeze(&mut m.interface_types);
    copy_errors(&mut m.errors, &mut m.interface_types, builtin_types.get());
    freeze(&mut m.interface_types);
  }
}
