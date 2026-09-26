use alloc::{string::String, vec::Vec};

use ulua_ast::records::{ast_expr::AstExpr, location::Location};
use ulua_common::fflag;

use crate::{
  functions::{
    begin_type_pack::{begin as begin_no_log, begin_type_pack_id_txn_log as begin},
    end_type_pack::end,
    flatten_type_pack::flatten,
    get_function_name_as_string::get_function_name_as_string,
    get_parameter_extents::get_parameter_extents,
    is_optional::is_optional,
    is_variadic_type_pack::is_variadic,
  },
  records::{
    code_too_complex::CodeTooComplex,
    count_mismatch::{CountMismatch, CountMismatchContext},
    free_type_pack::FreeTypePack,
    generic_type_pack::GenericTypePack,
    type_checker::TypeChecker,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    unifier::Unifier,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    error_type::ErrorType, error_type_pack::ErrorTypePack, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl TypeChecker {
  pub(crate) fn check_argument_list(
    &mut self,
    scope: &ScopePtr,
    fun_name: &AstExpr,
    state: &mut Unifier,
    arg_pack: TypePackId,
    param_pack: TypePackId,
    arg_locations: &[Location],
  ) {
    // Important terminology refresher:
    // A function requires parameters.
    // To call a function, you supply arguments.
    let mut arg_iter = begin(arg_pack, &state.log as *const _);
    let mut param_iter = begin(param_pack, &state.log as *const _);
    let end_iter = end(arg_pack); // Important subtlety: All end TypePackIterators are equivalent

    let mut param_index: usize = 0;

    let mut loop_count: i32;

    // exceedsLoopCount lambda (returns true if the loop should bail)
    macro_rules! exceeds_loop_count {
      () => {{
        loop_count += 1;
        if loop_count > ulua_common::fint::LuauTypeInferTypePackLoopLimit.get() {
          state.report_error_location_type_error_data(
            state.location,
            TypeErrorData::CodeTooComplex(CodeTooComplex { _unused: None }),
          );
          self.report_error_code_too_complex(&state.location);
          true
        } else {
          false
        }
      }};
    }

    // reportCountMismatchError lambda
    macro_rules! report_count_mismatch_error {
      () => {{
        // For this case, we want the error span to cover every errant extra parameter
        let mut location = state.location;
        if !arg_locations.is_empty() {
          location = Location::new(
            state.location.begin,
            arg_locations[arg_locations.len() - 1].end,
          );
        }

        let mut name_path = alloc::string::String::new();

        if let Some(path) = get_function_name_as_string(fun_name) {
          name_path = path;
        }

        let (min_params, opt_max_params) =
          // Safety: `get_parameter_extents` 为 `unsafe fn`（契约：`log`/`tp` 在其
          // 存活期内有效）。`&state.log as *const _` 借自本函数持有 `&mut
          // Unifier` 的存活字段，本次同步调用内独占有效；`param_pack` 是消解
          // 会话期 arena 驻留包节点，仅只读遍历计数（cpp
          // `getParameterExtents` 同款入参形态）。
          unsafe { get_parameter_extents(&state.log as *const _, param_pack, false) };
        // 纯计数：直接用 std Iterator::count 等价替代三件套循环
        let actual = begin_no_log(arg_pack).count();
        state.report_error_location_type_error_data(
          location,
          TypeErrorData::CountMismatch(CountMismatch {
            expected: min_params,
            maximum: opt_max_params,
            actual,
            context: CountMismatchContext::Arg,
            is_variadic: false,
            function: name_path,
          }),
        );
      }};
    }

    loop {
      state.location = if param_index < arg_locations.len() {
        arg_locations[param_index]
      } else {
        state.location
      };

      if arg_iter == end_iter && param_iter == end_iter {
        let arg_tail = arg_iter.tail();
        let param_tail = param_iter.tail();

        // If we hit the end of both type packs simultaneously, we have to unify them.
        // But if one side has a free tail and the other has none at all, we create an empty pack and bind the free tail to that.

        if let Some(arg_tail) = arg_tail {
          if !state
            .log
            .txn_log_get_mutable::<FreeTypePack, TypePackId>(
              state.log.follow_type_pack_id(arg_tail),
            )
            .is_null()
          {
            if let Some(param_tail) = param_tail {
              state.try_unify_type_pack_id_type_pack_id_bool_entry(param_tail, arg_tail, false);
            } else {
              state
                .log
                .replace_type_pack_id_type_pack_var(arg_tail, TypePackVar::from(TypePack::empty()));
            }
          } else if let Some(param_tail) = param_tail {
            state.try_unify_type_pack_id_type_pack_id_bool_entry(arg_tail, param_tail, false);
          }
        } else if let Some(param_tail) = param_tail {
          // argTail is definitely empty
          if !state
            .log
            .txn_log_get_mutable::<FreeTypePack, TypePackId>(
              state.log.follow_type_pack_id(param_tail),
            )
            .is_null()
          {
            state
              .log
              .replace_type_pack_id_type_pack_var(param_tail, TypePackVar::from(TypePack::empty()));
          }
        }

        return;
      } else if arg_iter == end_iter {
        // Not enough arguments.

        // Might be ok if we are forwarding a vararg along.  This is a common thing to occur in nonstrict mode.
        if let Some(tail) = arg_iter.tail() {
          if state
            .log
            .txn_log_get::<ErrorTypePack, TypePackId>(tail)
            .is_some()
          {
            // Unify remaining parameters so we don't leave any free-type_arguments hanging around.
            while param_iter != end_iter {
              let er = self.error_recovery_type_type_id(self.any_type);
              state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                er,
                *param_iter.current(),
                false,
                false,
                None,
              );
              param_iter.advance();
            }
            return;
          } else if state
            .log
            .txn_log_get::<VariadicTypePack, TypePackId>(tail)
            .is_some()
          {
            loop_count = 0;

            // Function is variadic and requires that all subsequent parameters
            // be compatible with a type.
            while param_iter != end_iter {
              // 外层已确认该 tail 是 VariadicTypePack；每轮重取先于 try_unify
              // 的写访问，单线程时序串行（C++ getMutable 同一取法）。
              let vtp_ty = state
                .log
                .txn_log_get::<VariadicTypePack, TypePackId>(tail)
                .expect("外层 is_some 判定已对同一 tail 通过")
                .ty;
              state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                vtp_ty,
                *param_iter.current(),
                false,
                false,
                None,
              );
              param_iter.advance();

              if exceeds_loop_count!() {
                return;
              }
            }

            return;
          } else if state
            .log
            .txn_log_get::<FreeTypePack, TypePackId>(tail)
            .is_some()
          {
            let mut rest: Vec<TypeId> = Vec::new();

            loop_count = 0;

            while param_iter != end_iter {
              rest.push(*param_iter.current());
              param_iter.advance();

              if exceeds_loop_count!() {
                return;
              }
            }

            let var_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::new(
              rest,
              param_iter.tail(),
            )));
            state.try_unify_type_pack_id_type_pack_id_bool_entry(tail, var_pack, false);
            return;
          }
        }

        // If any remaining unfulfilled parameters are nonoptional, this is a problem.
        while param_iter != end_iter {
          let t = state.log.follow_type_id(*param_iter.current());
          if is_optional(t) || !state.log.txn_log_get_mutable::<ErrorType, _>(t).is_null() {
            // ok
          } else {
            // Safety: 同 report_count_mismatch_error 宏内证成——`&state.log`
            // 借自本函数持有的 `&mut Unifier` 活对象、同步调用内独占有效，
            // `param_pack` 为会话期 arena 驻留包节点，只读遍历计数。
            let (min_params, opt_max_params) =
              unsafe { get_parameter_extents(&state.log as *const _, param_pack, false) };

            let tail = flatten(param_pack, &state.log).1;
            let is_variadic_flag = tail.is_some_and(is_variadic);

            let mut name_path = String::new();

            if let Some(path) = get_function_name_as_string(fun_name) {
              name_path = path;
            }

            state.report_error_location_type_error_data(
              fun_name.base.location,
              TypeErrorData::CountMismatch(CountMismatch {
                expected: min_params,
                maximum: opt_max_params,
                actual: param_index,
                context: CountMismatchContext::Arg,
                is_variadic: is_variadic_flag,
                function: name_path,
              }),
            );
            return;
          }
          param_iter.advance();
        }
      } else if param_iter == end_iter {
        // too many parameters passed
        if param_iter.tail().is_none() {
          loop_count = 0;

          while arg_iter != end_iter {
            // The use of unify here is deliberate. We don't want this unification
            // to be undoable.
            let er = self.error_recovery_type_scope_ptr(scope);
            self.unify_type_id_type_id_scope_ptr_location(
              er,
              *arg_iter.current(),
              scope,
              &state.location,
            );
            arg_iter.advance();

            if exceeds_loop_count!() {
              return;
            }
          }
          report_count_mismatch_error!();
          return;
        }
        // 上方 `tail().is_none()` 分支含 return 早退，此处 tail 恒 Some。
        let tail = state.log.follow_type_pack_id(
          param_iter
            .tail()
            .expect("上方 is_none 分支已 return，此处必为 Some"),
        );

        if state
          .log
          .txn_log_get::<ErrorTypePack, TypePackId>(tail)
          .is_some()
        {
          // Function is variadic.  Ok.
          return;
        } else if state
          .log
          .txn_log_get::<VariadicTypePack, TypePackId>(tail)
          .is_some()
        {
          loop_count = 0;

          // Function is variadic and requires that all subsequent parameters
          // be compatible with a type.
          let mut arg_index = param_index;
          while arg_iter != end_iter {
            let mut location = state.location;

            if arg_index < arg_locations.len() {
              location = arg_locations[arg_index];
            }

            state.location = location;
            // 外层已确认该 tail 是 VariadicTypePack；每轮重取先于 try_unify
            // 的写访问，arg_iter 与读皆纯读、原参数序内无写副作用（C++
            // getMutable 同一取法）。
            let vtp_ty = state
              .log
              .txn_log_get::<VariadicTypePack, TypePackId>(tail)
              .expect("外层 is_some 判定已对同一 tail 通过")
              .ty;
            state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
              *arg_iter.current(),
              vtp_ty,
              false,
              false,
              None,
            );

            arg_iter.advance();
            arg_index += 1;

            if exceeds_loop_count!() {
              return;
            }
          }

          return;
        } else if state
          .log
          .txn_log_get::<FreeTypePack, TypePackId>(tail)
          .is_some()
        {
          loop_count = 0;

          // Create a type pack out of the remaining argument type_arguments
          // and unify it with the tail.
          let mut rest: Vec<TypeId> = Vec::new();
          while arg_iter != end_iter {
            rest.push(*arg_iter.current());
            arg_iter.advance();

            if exceeds_loop_count!() {
              return;
            }
          }

          let var_pack = self
            .add_type_pack_type_pack_var(TypePackVar::from(TypePack::new(rest, arg_iter.tail())));
          state.try_unify_type_pack_id_type_pack_id_bool_entry(var_pack, tail, false);

          return;
        } else if !state
          .log
          .txn_log_get_mutable::<FreeTypePack, TypePackId>(tail)
          .is_null()
        {
          state
            .log
            .replace_type_pack_id_type_pack_var(tail, TypePackVar::from(TypePack::empty()));
          return;
        } else if !state
          .log
          .txn_log_get_mutable::<GenericTypePack, TypePackId>(tail)
          .is_null()
        {
          report_count_mismatch_error!();
          return;
        }
      } else {
        if fflag::LuauInstantiateInSubtyping.get() {
          state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
            *arg_iter.current(),
            *param_iter.current(),
            false, // isFunctionCall
            false,
            None,
          );
        } else {
          let sub = *arg_iter.current();
          let sup = *param_iter.current();
          self.unify_with_instantiation_if_needed_type_id_type_id_scope_ptr_unifier(
            sub,
            sup,
            scope.clone(),
            state,
          );
        }
        arg_iter.advance();
        param_iter.advance();
      }

      param_index += 1;
    }
  }
}
