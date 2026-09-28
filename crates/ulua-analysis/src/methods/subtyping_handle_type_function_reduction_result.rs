use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::{
  functions::reduce_type_functions_type_function::reduce_type_functions,
  records::{
    function_graph_reduction_result::FunctionGraphReductionResult, scope::Scope,
    subtyping::Subtyping, type_error::TypeError, type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    uninhabited_type_function::UninhabitedTypeFunction,
  },
  type_aliases::{
    error_vec::ErrorVec, module_name_type::ModuleName, type_error_data::TypeErrorData,
    type_id::TypeId,
  },
};
impl Subtyping {
  /// C++ `Subtyping::handleTypeFunctionReductionResult`：把未求解的类型函数实例
  /// 重新驱动一遍归约，被 block 时归约为 `never` 并附 `UninhabitedTypeFunction` 错误。
  ///
  /// # Safety 说明
  /// 形参全为受检类型/引用：`function_instance` 借自 arena 存活节点（只 clone 其内容），
  /// `scope` 为非空 `&Scope`（存进 [`TypeFunctionContext`] 供类型函数实现读取）。
  /// 函数体内的 `unsafe` 仅为把构造期注入的裸成员还原成 `NonNull` 接线 context——
  /// 非空、存活、单线程无别名契约由 `Subtyping::subtyping_owned` 构造期保证，与
  /// 迁移前各 `// SAFETY:` 注释逐条同构。
  pub fn handle_type_function_reduction_result(
    &mut self,
    function_instance: &TypeFunctionInstanceType,
    scope: &Scope,
  ) -> (TypeId, ErrorVec) {
    let mut subtyping = Subtyping::subtyping_owned(
      self.builtin_types,
      self.arena,
      self.normalizer,
      self.type_function_runtime.get(),
      self.ice_reporter.get(),
    );
    // Safety: 逐项非空由 Subtyping 构造契约给出——前五个来自构造期注入的裸成员、
    // `scope` 由 `&Scope` 引用类型保证非空；本块只做地址位转换（`as_nonnull` /
    // `NonNull::from`），不解引用、不派生引用，故与随后 context 内部对这些指针的
    // 读写不构成别名冲突。
    let (arena, builtins, scope, normalizer, runtime, ice) = unsafe {
      (
        self.arena.as_nonnull(),
        self.builtin_types.as_nonnull(),
        NonNull::from(scope),
        NonNull::new_unchecked(self.normalizer_ptr()),
        self.type_function_runtime.as_nonnull(),
        self.ice_reporter.as_nonnull(),
      )
    };
    let mut context = TypeFunctionContext::from_components(
      arena,
      builtins,
      scope,
      normalizer,
      runtime,
      ice,
      NonNull::from(&mut self.limits),
      NonNull::from(&mut subtyping),
    );

    // Safety: `arena` 是构造期注入、会话内存活的 TypeArena 地址；`add_type` 追加一个
    // FunctionType 节点并返回其 TypeId（arena 节点不回收），可变借用止于本语句。
    let function = unsafe { (*arena.as_ptr()).add_type(function_instance.clone()) };
    let result: FunctionGraphReductionResult =
      reduce_type_functions(function, Location::default(), &mut context, true);
    let mut errors: ErrorVec = ErrorVec::new();
    if result.blocked_types.size() != 0 || result.blocked_packs.size() != 0 {
      errors.push(TypeError {
        location: Location::default(),
        module_name: ModuleName::new(),
        data: TypeErrorData::UninhabitedTypeFunction(UninhabitedTypeFunction { ty: function }),
      });
      // Safety: `builtins` 指向只读的 BuiltinTypes 表（构造期注入、检查期内不变），
      // 这里只 Copy 出 never_type 句柄；与刚写入的 arena 是两块互不相干的内存。
      return (unsafe { (*builtins.as_ptr()).never_type }, errors);
    }
    if result.reduced_types.contains(&function) {
      return (function, errors);
    }
    // Safety: 同上一分支——builtins 表在归约后仍存活且 never_type 字段不再变更，
    // 本次读取只是取回句柄作为兜底结果。
    (unsafe { (*builtins.as_ptr()).never_type }, errors)
  }
}
