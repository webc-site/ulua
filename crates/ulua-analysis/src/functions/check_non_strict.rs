//! C++ free function `Luau::checkNonStrict(...)`
//! (`Analysis/src/NonStrictTypeChecker.cpp:1287-1320`).
use core::ptr::NonNull;

use crate::{
  functions::{copy_errors::copy_errors, freeze::freeze, unfreeze::unfreeze},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, data_flow_graph::DataFlowGraph,
    internal_error_reporter::InternalErrorReporter, module::Module,
    non_strict_type_checker::NonStrictTypeChecker, source_module::SourceModule,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::type_error_data::TypeErrorData,
};

/// 对应 C++ `checkNonStrict`（NonStrictTypeChecker.cpp:1287-1320）。基础设施形参
/// 已收窄为受检引用（C++ NotNull/const T* 语义由类型编码），剩余裸指针仅
/// `module`（C++ `Module&` 直译）：须为非空、地址稳定的 Arc 写穿句柄且比本次
/// 检查长寿，经 checked `NonNull` 物化——null 输入由 UB 收敛为 panic；
/// `source_module.root` 的非空 arena 块指针前提由其构造点（parser arena）保证。
pub fn check_non_strict(
  builtin_types: Handle<BuiltinTypes>,
  type_function_runtime: &mut TypeFunctionRuntime,
  ice: &mut InternalErrorReporter,
  unifier_state: &mut UnifierSharedState,
  dfg: &DataFlowGraph,
  limits: &TypeCheckLimits,
  source_module: &SourceModule,
  module: *mut Module,
) {
  let mut module_nn = NonNull::new(module).expect("module 写穿句柄非空（cpp Module& 直译）");

  // SAFETY: module_nn 由上方判空取得，指向 Arc<Module> 堆对象（地址固定、比本次检查
  // 长寿）；取 &mut 仅为一句话派生 internal_types 的 arena 句柄，构造后即转成裸指针
  // 字段暂存，不再经该借用解引用。单线程独占期无别名可变借用。
  let arena = unsafe { Handle::from_mut(&mut module_nn.as_mut().internal_types) };
  // 「裸构造 → 自指针回填 → 未布线中间态外露」的悬置窗口收进 `new_boxed`
  // 封闭门面，调用点拿到即布线完成的 `Box` 安全句柄。
  let mut type_checker = NonStrictTypeChecker::new_boxed(
    arena,
    builtin_types,
    type_function_runtime,
    ice,
    unifier_state,
    dfg,
    limits,
    module,
  );

  type_checker.visit_ast_stat_block(source_module.root);

  // SAFETY: AST 遍历已在上一行结束，type_checker 经构造存下的 internal_types
  // 裸指针此后不再被解引用，故此处对 module_nn 重建 &mut 独占借用无在册别名；
  // 块内 unfreeze/copy_errors/retain/freeze 单线程串行、先后不重叠。
  // builtin_types 为会话级非空句柄（目标为常驻单例），copy_errors 只共享读取其
  // 错误类型常量。
  unsafe {
    let module = module_nn.as_mut();
    unfreeze(&mut module.interface_types);
    copy_errors(
      &mut module.errors,
      &mut module.interface_types,
      builtin_types.get(),
    );

    module
      .errors
      .retain(|err| !matches!(err.data, TypeErrorData::UnknownRequire(_)));

    freeze(&mut module.interface_types);
  }
}
