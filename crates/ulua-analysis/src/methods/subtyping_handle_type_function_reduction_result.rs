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
  /// # Safety
  /// - `self`：`builtin_types`/`arena`/`normalizer`/`type_function_runtime`/`ice_reporter`
  ///   五个裸成员会被原样交给 [`TypeFunctionContext`]（C++ 侧是 `NotNull` 成员），因此它们
  ///   必须已由 `Subtyping::subtyping*` 构造期注入为非空地址，并在本次归约（含被归约过程
  ///   回调的其它代码）期间存活、不被移动或释放。
  /// - `function_instance`：须指向类型 arena 中一个存活的
  ///   [`TypeFunctionInstanceType`]；本函数只 `clone()` 其内容再作为新的 `FunctionType`
  ///   写入 arena，不接管、也不失效该借用。
  /// - `scope`：对应 C++ `NotNull<Scope>` 形参，必须非空且存活到归约返回——它被存进
  ///   context 供类型函数实现读取，空指针会让 `NonNull::new_unchecked` 直接失效。
  pub unsafe fn handle_type_function_reduction_result(
    &mut self,
    function_instance: &TypeFunctionInstanceType,
    scope: *mut Scope,
  ) -> (TypeId, ErrorVec) {
    let mut subtyping = Subtyping::subtyping_owned(
      self.builtin_types,
      self.arena,
      self.normalizer_ptr(),
      self.type_function_runtime.as_ptr(),
      self.ice_reporter.as_ptr(),
    );
    // Safety: 逐项非空由上面的函数级契约给出——前五个来自构造期注入的裸成员、`scope`
    // 来自 C++ `NotNull<Scope>` 传参约定；本块只做 `*mut T -> NonNull<T>` 的地址位转换，
    // 不解引用、不派生引用，故与随后 context 内部对这些指针的读写不构成别名冲突。
    let (arena, builtins, scope, normalizer, runtime, ice) = unsafe {
      (
        NonNull::new_unchecked(self.arena.as_ptr()),
        NonNull::new_unchecked(self.builtin_types.as_ptr()),
        NonNull::new_unchecked(scope),
        NonNull::new_unchecked(self.normalizer_ptr()),
        NonNull::new_unchecked(self.type_function_runtime.as_ptr()),
        NonNull::new_unchecked(self.ice_reporter.as_ptr()),
      )
    };
    let context = TypeFunctionContext::from_components(
      arena,
      builtins,
      scope,
      normalizer,
      runtime,
      ice,
      NonNull::from(&mut self.limits),
      NonNull::from(&mut subtyping),
    );
    let mut context = context;

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
