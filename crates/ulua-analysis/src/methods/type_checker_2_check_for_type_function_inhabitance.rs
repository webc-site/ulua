//! Faithful port of `TypeChecker2::checkForTypeFunctionInhabitance`
//! (TypeChecker2.cpp:496-508).
use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::{
  functions::reduce_type_functions_type_function::reduce_type_functions,
  records::{type_checker_2::TypeChecker2, type_function_context::TypeFunctionContext},
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  pub fn check_for_type_function_inhabitance(
    &mut self,
    instance: TypeId,
    location: Location,
  ) -> TypeId {
    if self.seen_type_function_instances.find(&instance).is_some() {
      return instance;
    }
    self.seen_type_function_instances.insert(instance);

    // TypeFunctionContext context{NotNull{&module->internalTypes}, builtinTypes,
    //     stack.back(), NotNull{&normalizer}, typeFunctionRuntime, ice, limits, subtyping};
    let context = {
      // Safety: `self.module` 是构造期注入的 NotNull<Module>（非空、随检查
      // 全程存活）；internal_types 为其内嵌字段，取地址必非空，&mut 借用半径
      // 止于 from_components 调用——本方法其余时间不再经 self 触碰该字段。
      let arena = unsafe { NonNull::new_unchecked(&mut (*self.module).internal_types) };
      // Safety: self.builtin_types.as_ptr() 对应 C++ NotNull<BuiltinTypes> 构造契约，
      // 非空且为分析期单例；new_unchecked 仅升格既有非空裸指针，不新建借用。
      let builtins = unsafe { NonNull::new_unchecked(self.builtin_types.as_ptr()) };
      // stack 非空是 TypeChecker2 的推进不变量（进入任何访问前必已压入当前
      // scope），expect 空栈只会确定性 panic 而非 UB；栈元素已句柄化
      // （Handle<Scope> 编码非空），`as_nonnull` 与原 `NonNull::new_unchecked` 逐位等价。
      let scope = self
        .stack
        .last()
        .expect("推进不变量：访问期栈内恒有当前 scope")
        .as_nonnull();
      // Safety: &mut self.normalizer 源自本方法唯一的 &mut self 借用，地址
      // 非空平凡成立；借用降为 NonNull 句柄后，context 存活期内（至
      // reduce_type_functions 返回）无人再经 self 访问 normalizer。
      let normalizer = unsafe { NonNull::new_unchecked(&mut self.normalizer) };
      // Safety: 构造期 NotNull<TypeFunctionRuntime> 契约：非空且随检查器存活。
      let type_function_runtime =
        unsafe { NonNull::new_unchecked(self.type_function_runtime.as_ptr()) };
      // Safety: 构造期 NotNull<InternalErrorReporter> 契约：非空且存活。
      let ice = unsafe { NonNull::new_unchecked(self.ice.as_ptr()) };
      // 构造期 NotNull<TypeCheckLimits> 契约已句柄化：`as_nonnull` 无判空分支、
      // 与原 `NonNull::new_unchecked(self.limits)` 逐位等价。
      let limits = self.limits.as_nonnull();
      // 构造期 NotNull<Subtyping> 契约：`subtyping_handle` 收敛 Option 判空
      // （未接线即确定性 panic），指向本结构体内嵌 `_subtyping`。
      let subtyping = self.subtyping_handle().as_nonnull();
      TypeFunctionContext::from_components(
        arena,
        builtins,
        scope,
        normalizer,
        type_function_runtime,
        ice,
        limits,
        subtyping,
      )
    };

    let mut context = context;
    let errors = reduce_type_functions(instance, location, &mut context, true).errors;

    if !self.is_error_suppressing_location_type_id(location, instance) {
      self.report_errors(errors);
    }
    instance
  }
}
