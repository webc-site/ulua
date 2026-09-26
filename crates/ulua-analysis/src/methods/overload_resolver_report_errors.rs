//! Source: `Analysis/src/OverloadResolver.cpp:289-453` (hand-ported)
use alloc::string::String;

use ulua_ast::records::{ast_expr::AstExpr, location::Location};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{subtyping_variance::SubtypingVariance, value::Value},
  functions::{
    flatten_type_pack::flatten_type_pack_id,
    get_argument_index::get_argument_index,
    get_parameter_extents::get_parameter_extents,
    get_type_pack,
    is_path_on_argument_list::is_path_on_argument_list,
    is_variadic_type_pack::is_variadic,
    should_suppress_errors_type_utils::should_suppress_errors_not_null_normalizer_type_pack_id,
    traverse_for_flattened_pack::traverse_for_flattened_pack,
    traverse_for_pack_type_path::traverse_for_pack,
    traverse_type_path::{traverse_type_pack_root as traverse_pack_root, traverse_type_root},
  },
  records::{
    count_mismatch::{CountMismatch, CountMismatchContext},
    function_type::FunctionType,
    generic_type_pack::GenericTypePack,
    internal_error::InternalError,
    normalization_too_complex::NormalizationTooComplex,
    overload_resolver::OverloadResolver,
    subtyping_reasoning::SubtypingReasoning,
    txn_log::TxnLog,
    type_error::TypeError,
    type_pack_mismatch::TypePackMismatch,
  },
  type_aliases::{
    error_vec::ErrorVec, module_name_type::ModuleName, type_error_data::TypeErrorData,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl OverloadResolver<'_> {
  /// 取第 idx 个参数位置；idx 越界取最后一个参数，无参数取函数位置（C++ 同逻辑）。
  fn arg_location_at(
    arg_exprs: &[*mut AstExpr],
    idx: Option<usize>,
    fn_location: Location,
  ) -> Location {
    if let Some(idx) = idx
      && idx < arg_exprs.len()
    {
      // Safety: `arg_exprs` 即 C++ `reportErrors` 形参 `const std::vector<AstExpr*>&`
      // 的直译，每个元素来自调用点 `Call` 表达式的 `args`（AST arena 分配、
      // 非空且存活至本次重载消解结束）；`idx < len` 已在上方守卫，此处仅
      // 读出 `Copy` 的 `base.location`。
      unsafe { (*arg_exprs[idx]).base.location }
    } else if let Some(&last) = arg_exprs.last() {
      // Safety: `last` 与索引分支同源——实参列表非空时其尾元素同样是 AST
      // arena 的非空 `*mut AstExpr`，只读 `location` 字段（C++
      // `argExprs.back()->location`）。
      unsafe { (*last).base.location }
    } else {
      fn_location
    }
  }

  pub fn report_errors(
    &self,
    errors: &mut ErrorVec,
    fn_ty: TypeId,
    fn_location: Location,
    module_name: &ModuleName,
    arg_pack: TypePackId,
    arg_exprs: &[*mut AstExpr],
    reason: &SubtypingReasoning,
  ) {
    let argument_index = get_argument_index(&reason.sub_path, fn_ty);

    let mut arg_location = Self::arg_location_at(arg_exprs, argument_index, fn_location);

    // Safety: `self.builtin_types.as_ptr()` 对应 C++ `OverloadResolver` 的
    // `NotNull<BuiltinTypes> builtinTypes`——构造期注入即保证非空，内建类型
    // 构建完成后不再改写，随整个解析过程存活；此处只读其 `Copy` 内建句柄。
    let empty_type_pack = self.builtin_types.get().empty_type_pack;

    // Safety: `self.arena.as_ptr()` 对应 C++ `NotNull<TypeArena> arena`，非空且在本次
    // 消解期间有效；`add_type` 是对它的临时 `&mut` 再借用，语句结束即归还，
    // 与同表达式内对 `BuiltinTypes` 单例的只读字段访问（`any_type_pack`）
    // 是两个互不相交的对象（C++ 同调用 `arena->addType(FunctionType{argPack,
    // builtinTypes->anyTypePack})`）。
    let prospective_function = {
      self
        .arena
        .get_mut()
        .add_type(FunctionType::function_type_new(
          arg_pack,
          self.builtin_types.get().any_type_pack,
          None,
          false,
        ))
    };

    let failed_sub_pack: Option<TypePackId> = traverse_for_pack(
      prospective_function,
      &reason.super_path,
      // Safety: `self.builtin_types.get()` 把证过非空、构造后不可变的 `BuiltinTypes`
      // 单例借用为 `&`，与 C++ `traverseForPack` 的 const 入参同义。
      { self.builtin_types.get() },
      // Safety: `self.arena.as_ptr()` 对应 C++ `NotNull<TypeArena>`，非空且本次消解
      // 期间有效；`&mut` 再借用随本调用结束归还，对 arena 的可变访问均为
      // 顺序短借用、不并存。`prospective_function` 是刚加入 arena 的节点。
      { self.arena.get_mut() },
    );
    let failed_super_pack: Option<TypePackId> = traverse_for_pack(
      fn_ty,
      &reason.sub_path,
      // Safety: 单例只读再借用（同上条证成）。
      { self.builtin_types.get() },
      // Safety: arena 独占短借用；上一条调用的 `&mut` 已在其返回时结束，
      // 两次借用时序不重叠。`fn_ty` 为候选函数类型（arena 驻留节点）。
      { self.arena.get_mut() },
    );

    if let Some(fsp) = failed_super_pack
      && get_type_pack::get::<GenericTypePack>(fsp).is_some()
    {
      // Safety: 与函数头 `empty_type_pack` 同一解引用——非空单例的只读
      // `Copy` 内建句柄（C++ `failedSubPack.value_or(builtinTypes->emptyTypePack)`）。
      let given = failed_sub_pack.unwrap_or({ self.builtin_types.get().empty_type_pack });
      self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_pack_id_optional_type_pack_id(
        errors,
        arg_location,
        module_name,
        reason,
        failed_super_pack,
        Some(given),
      );
      return;
    }

    // If the mismatch is on the argument list itself, then the wrong number of parameters were passed.
    if is_path_on_argument_list(&reason.sub_path) {
      // If insufficiently many parameters are passed, we expect an empty
      // subPath.
      //
      // If too many parameters are passed, we expect a slice subPath which
      // points to the start of the unsatisfied arguments, and a superPath
      // which points at the tail of the parameter list.
      //
      // Sometimes, the superPath includes generic substitutions.  We need to
      // take this into account when computing the expected parameter count.

      if failed_super_pack.is_none() {
        errors.push(TypeError::type_error_location_module_name_type_error_data(
          fn_location,
          module_name.clone(),
          TypeErrorData::InternalError(InternalError::new(
            "Malformed SubtypingReasoning".to_string(),
          )),
        ));
        return;
      }

      // Safety: 直译 C++ `arena->addTypePack(traverseForFlattenedPack(fnTy,
      // subPath, builtinTypes.get(), arena))`。`self.builtin_types.get()` 是
      // 单例只读借用；`self.arena.get_mut()` 传入 `traverse_for_flattened_pack`
      // 后随该调用返回即结束，随后 `self.arena.get_mut().add_type_pack_t` 才对
      // arena 重新取独占借用——两次 `&mut` 时序衔接、不重叠。`fn_ty` 是
      // arena 驻留的候选函数类型。
      let required_mapped_args = {
        self
          .arena
          .get_mut()
          .add_type_pack_t(traverse_for_flattened_pack(
            fn_ty,
            &reason.sub_path,
            self.builtin_types.get(),
            self.arena.get_mut(),
          ))
      };
      let (params_head, _params_tail) = flatten_type_pack_id(required_mapped_args);
      let (arg_head, arg_tail) = flatten_type_pack_id(arg_pack);

      let arg_count = arg_head.len();
      // Safety: `get_parameter_extents` 为 `unsafe fn`（契约：`log`/`tp` 在其
      // 存活期内有效）。`TxnLog::empty()` 返回进程级只读空日志单例的
      // `*const TxnLog`（永不失效，对应 C++ 静态 `TxnLog::empty()`）；
      // `required_mapped_args` 是刚写入 arena 的驻留 pack（C++
      // `getParameterExtents(TxnLog::empty(), requiredMappedArgs)`）。
      let (_min_params, opt_max_params) =
        unsafe { get_parameter_extents(TxnLog::empty(), required_mapped_args, false) };

      match should_suppress_errors_not_null_normalizer_type_pack_id(
        self.normalizer.as_ptr(),
        arg_pack,
      )
      .error_suppression_value()
      {
        Value::Suppress => return,
        Value::NormalizationFailed => {
          errors.push(TypeError::type_error_location_module_name_type_error_data(
            fn_location,
            module_name.clone(),
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          ));
          return;
        }
        Value::DoNotSuppress => {}
      }

      // failedSuperPack is guaranteed Some here (checked above).
      match should_suppress_errors_not_null_normalizer_type_pack_id(
        self.normalizer.as_ptr(),
        required_mapped_args,
      )
      .error_suppression_value()
      {
        Value::Suppress => return,
        Value::NormalizationFailed => {
          errors.push(TypeError::type_error_location_module_name_type_error_data(
            fn_location,
            module_name.clone(),
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          ));
          return;
        }
        Value::DoNotSuppress => {}
      }

      let is_variadic_flag = match arg_tail {
        Some(t) => is_variadic(t),
        None => false,
      };

      if is_variadic_flag {
        // Not actually a count mismatch!  This can happen if the
        // required parameters are a generic pack that has not been
        // satisfied.
        let given = failed_sub_pack.unwrap_or(empty_type_pack);
        self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_pack_id_optional_type_pack_id(
          errors,
          arg_location,
          module_name,
          reason,
          failed_super_pack,
          Some(given),
        );
      } else {
        errors.push(TypeError::type_error_location_module_name_type_error_data(
          fn_location,
          module_name.clone(),
          TypeErrorData::CountMismatch(CountMismatch {
            expected: params_head.len(),
            maximum: opt_max_params,
            actual: arg_count,
            context: CountMismatchContext::Arg,
            is_variadic: is_variadic_flag,
            function: String::new(),
          }),
        ));
      }

      return;
    }

    if argument_index.is_some() {
      // The first path component should always be PackField::Arguments
      LUAU_ASSERT!(reason.sub_path.components.len() > 1);
      let mut super_path_tail = reason.super_path.clone();
      super_path_tail.components.remove(0);

      let failed_sub = traverse_pack_root(
        arg_pack,
        &super_path_tail,
        // Safety: 函数头已证 `self.builtin_types.as_ptr()` 非空且单例不可变，
        // 共享借用只存活于本调用。
        { self.builtin_types.get() },
        // Safety: 函数头已证 `self.arena.as_ptr()` 为有效 arena；此处独占短借用，
        // 前面对 arena 的所有 `&mut` 借用均已随语句结束归还，无并存别名。
        { self.arena.get_mut() },
      );
      let failed_super = traverse_type_root(
        fn_ty,
        &reason.sub_path,
        // Safety: 单例只读共享借用，存活期止于本调用（同 failed_sub）。
        { self.builtin_types.get() },
        // Safety: arena 独占短借用，`failed_sub` 的借用已结束，两次不重叠；
        // 返回的 pack/type 句柄均驻留该 arena。
        { self.arena.get_mut() },
      );

      self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_or_pack_optional_type_or_pack(
        errors,
        arg_location,
        module_name,
        reason,
        failed_super,
        failed_sub,
      );
      return;
    }

    if let Some(fsp) = failed_sub_pack
      && failed_super_pack.is_none()
      && get_type_pack::get::<GenericTypePack>(fsp).is_some()
    {
      errors.push(TypeError::type_error_location_module_name_type_error_data(
        arg_location,
        module_name.clone(),
        TypeErrorData::TypePackMismatch(TypePackMismatch {
          wanted_tp: fsp,
          given_tp: empty_type_pack,
          reason: String::new(),
        }),
      ));
    }

    if let (Some(fsp), Some(fsup)) = (failed_sub_pack, failed_super_pack) {
      // If a bug in type inference occurs, we may have a mismatch in the return packs.
      // This happens when inference incorrectly leaves the result type of a function free.
      // If this happens, we don't want to explode, so we'll use the function's location.
      // idx=None → 取最后一个实参，无实参则退回函数位置。
      arg_location = Self::arg_location_at(arg_exprs, None, fn_location);

      let error_suppression =
        should_suppress_errors_not_null_normalizer_type_pack_id(self.normalizer.as_ptr(), fsp)
          .or_else(&should_suppress_errors_not_null_normalizer_type_pack_id(
            self.normalizer.as_ptr(),
            fsup,
          ));
      if error_suppression.error_suppression_value() == Value::Suppress {
        return;
      }

      // 上游 C++ 为分立三臂：Covariant/Invariant 各推 (fsp, fsup)，Contravariant 推 (fsup, fsp)，
      // 仅 wanted/given 交换；此处映射为 (wanted, given) 序对后收口为单次 push。
      let (wanted_tp, given_tp) = match reason.variance {
        SubtypingVariance::Contravariant => (fsup, fsp),
        SubtypingVariance::Covariant | SubtypingVariance::Invariant => (fsp, fsup),
        _ => {
          LUAU_ASSERT!(false);
          return;
        }
      };
      errors.push(TypeError::type_error_location_module_name_type_error_data(
        arg_location,
        module_name.clone(),
        TypeErrorData::TypePackMismatch(TypePackMismatch {
          wanted_tp,
          given_tp,
          reason: String::new(),
        }),
      ));
    }
  }
}
