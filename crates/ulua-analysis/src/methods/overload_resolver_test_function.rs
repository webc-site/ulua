//! Source: `Analysis/src/OverloadResolver.cpp:457-548` (hand-ported)
//!
//! Test a single FunctionType against an argument list. Reduces type functions
//! and does a proper arity check.
use alloc::vec::Vec;
use core::{
  mem::take,
  ptr::{NonNull, null, null_mut},
};

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_set::DenseHashSet, variant::Variant2};

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::{
    are_unsatisfied_arguments_optional::are_unsatisfied_arguments_optional, follow_type, get_type,
    ignore_reasoning_for_return_type::ignore_reasoning_for_return_type,
    reduce_type_functions_type_function::reduce_type_functions,
  },
  records::{
    blocked_type::BlockedType, free_type::FreeType, function_type::FunctionType,
    overload_resolution::OverloadResolution, overload_resolver::OverloadResolver,
    pending_expansion_type::PendingExpansionType, type_error::TypeError,
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{
    error_vec::ErrorVec, type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl OverloadResolver<'_> {
  pub fn test_function(
    &mut self,
    result: &mut OverloadResolution,
    fn_ty: TypeId,
    args_pack: TypePackId,
    fn_location: Location,
    unique_types: *mut DenseHashSet<TypeId>,
  ) {
    let fn_ty = follow_type::follow(fn_ty);

    // TODO: This seems like the wrong spot to do this check.
    if get_type::get::<FreeType>(fn_ty).is_some()
      || get_type::get::<BlockedType>(fn_ty).is_some()
      || get_type::get::<PendingExpansionType>(fn_ty).is_some()
    {
      // TODO.  Luckily, these constraints are not yet used.
      let constraints = Vec::new();
      result.potential_overloads.push((fn_ty, constraints));
      return;
    }

    if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(fn_ty)
      && tfit.state == TypeFunctionInstanceState::Unsolved
    {
      // TODO.  Luckily, these constraints are not yet used.
      let constraints = Vec::new();
      result.potential_overloads.push((fn_ty, constraints));
      return;
    }

    let Some(ftv) = get_type::get::<FunctionType>(fn_ty) else {
      result.non_functions.push(fn_ty);
      return;
    };

    if !self.is_arity_compatible(args_pack, ftv.arg_types, self.builtin_types) {
      result.arity_mismatches.push(fn_ty);
      return;
    }

    // Safety: 以下六个 NonNull 字段源自 unsafe fn `OverloadResolver::new` 的入参
    // 契约——构造期注入的非空指针，比 resolver 长寿；`new_unchecked` 只把同一
    // 地址原值重建为 NonNull 句柄、不触碰对象，读写全程单线程串行。
    // Safety: arena 指向调用方传入的 &mut module.internal_types 活借用基址，本函数内存活。
    let arena = unsafe { NonNull::new_unchecked(self.arena.as_ptr()) };
    // Safety: builtin_types 为会话级 NotNull 内置类型表，长寿非空，此处仅取地址。
    let builtins = unsafe { NonNull::new_unchecked(self.builtin_types.as_ptr()) };
    // Safety: scope 是 `new` 入参里 find_innermost_scope 产出的模块作用域树内非空指针。
    let scope = unsafe { NonNull::new_unchecked(self.scope) };
    // Safety: normalizer 源自调用点 `&mut self.normalizer.as_ptr()` 活借用的基址，独占且存活。
    let normalizer = unsafe { NonNull::new_unchecked(self.normalizer.as_ptr()) };
    // Safety: type_function_runtime 为构造注入的会话级运行时单例地址，非空长寿。
    let type_function_runtime =
      unsafe { NonNull::new_unchecked(self.type_function_runtime.as_ptr()) };
    // Safety: ice 为构造注入的报告器地址（源自 unifier_state），比本次调用长寿。
    let ice = unsafe { NonNull::new_unchecked(self.ice.as_ptr()) };
    let mut context = TypeFunctionContext {
      arena,
      builtins,
      scope,
      normalizer,
      type_function_runtime,
      ice,
      limits: NonNull::from(self.limits),
      subtyping: NonNull::from(&mut self.subtyping),
      solver: null_mut(),
      constraint: null(),
      user_func_name: None,
      fresh_instances: Vec::new(),
    };
    let reduce_result =
      reduce_type_functions(fn_ty, self.call_loc, &mut context, /*force=*/ true);
    if !reduce_result.errors.is_empty() {
      result
        .incompatible_overloads
        .push((fn_ty, Variant2::V1(reduce_result.errors)));
      return;
    }

    // Safety: `self.arena.as_ptr()`/`self.builtin_types.as_ptr()` 为 `new` 契约下的非空长寿指针；
    // `add_type` 经 arena 可变借用追加一个 FunctionType 节点，与 reduce 阶段经
    // context 的写入先后串行、无并存借用；`any_type_pack` 仅只读拷贝句柄值。
    let prospective_function = {
      self
        .arena
        .get_mut()
        .add_type(FunctionType::function_type_new(
          args_pack,
          self.builtin_types.get().any_type_pack,
          None,
          false,
        ))
    };

    self.subtyping.unique_types = unique_types as *const DenseHashSet<TypeId>;
    let scope = self.scope;
    let mut r =
      self
        .subtyping
        .is_subtype_type_id_type_id_not_null_scope(fn_ty, prospective_function, scope);

    // Frustratingly, subtyping does not know about error suppression, so this
    // subtype test will probably fail due to the mismatched return types. Here,
    // we'll prune any SubtypingReasons that have anything to do with the return
    // type.
    ignore_reasoning_for_return_type(&mut r);

    if r.is_subtype {
      if r.assumed_constraints.is_empty() {
        result.ok.push(fn_ty);
      } else {
        result
          .potential_overloads
          .push((fn_ty, take(&mut r.assumed_constraints)));
      }
    } else if !r.generic_bounds_mismatches.is_empty() {
      let mut errors: ErrorVec = Vec::new();
      for gbm in r.generic_bounds_mismatches.iter() {
        errors.push(TypeError::type_error_location_type_error_data(
          fn_location,
          TypeErrorData::GenericBoundsMismatch(gbm.clone()),
        ));
      }
      result
        .incompatible_overloads
        .push((fn_ty, Variant2::V1(errors)));
    } else if are_unsatisfied_arguments_optional(&r.reasoning, args_pack, ftv.arg_types) {
      // Important!  Subtyping doesn't know anything about
      // optional arguments.  If the only reason subtyping
      // failed is because optional arguments were not provided,
      // then this overload is actually okay.
      if r.assumed_constraints.is_empty() {
        result.ok.push(fn_ty);
      } else {
        result
          .potential_overloads
          .push((fn_ty, take(&mut r.assumed_constraints)));
      }
    } else {
      result
        .incompatible_overloads
        .push((fn_ty, Variant2::V0(take(&mut r.reasoning))));
    }
  }
}
