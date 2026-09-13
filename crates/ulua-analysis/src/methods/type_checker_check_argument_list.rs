use alloc::{string::String, vec::Vec};

use ulua_ast::records::{ast_expr::AstExpr, location::Location};
use ulua_common::FFlag;

use crate::{
  functions::{
    begin_type_pack::begin as begin_no_log, begin_type_pack_alt_d::begin, end_type_pack::end,
    flatten_type_pack_alt_b::flatten, get_function_name_as_string::get_function_name_as_string,
    get_parameter_extents::get_parameter_extents, is_optional::is_optional,
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
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
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
        if loop_count > ulua_common::FInt::LuauTypeInferTypePackLoopLimit.get() {
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
          unsafe { get_parameter_extents(&state.log as *const _, param_pack, false) };
        let actual = {
          let mut count: usize = 0;
          let mut it = begin_no_log(arg_pack);
          let e = end(arg_pack);
          while it.operator_ne(&e) {
            count += 1;
            it.operator_inc();
          }
          count
        };
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

      if arg_iter.operator_eq(&end_iter) && param_iter.operator_eq(&end_iter) {
        let arg_tail = arg_iter.tail();
        let param_tail = param_iter.tail();

        // If we hit the end of both type packs simultaneously, we have to unify them.
        // But if one side has a free tail and the other has none at all, we create an empty pack and bind the free tail to that.

        if let Some(arg_tail) = arg_tail {
          if !state
            .log
            .txn_log_get_mutable::<FreeTypePack, TypePackId>(unsafe {
              state.log.follow_type_pack_id(arg_tail)
            })
            .is_null()
          {
            if let Some(param_tail) = param_tail {
              state.try_unify_type_pack_id_type_pack_id_bool_entry(param_tail, arg_tail, false);
            } else {
              state.log.replace_type_pack_id_type_pack_var(
                arg_tail,
                TypePackVar::from(TypePack {
                  head: Vec::new(),
                  tail: None,
                }),
              );
            }
          } else if let Some(param_tail) = param_tail {
            state.try_unify_type_pack_id_type_pack_id_bool_entry(arg_tail, param_tail, false);
          }
        } else if let Some(param_tail) = param_tail {
          // argTail is definitely empty
          if !state
            .log
            .txn_log_get_mutable::<FreeTypePack, TypePackId>(unsafe {
              state.log.follow_type_pack_id(param_tail)
            })
            .is_null()
          {
            state.log.replace_type_pack_id_type_pack_var(
              param_tail,
              TypePackVar::from(TypePack {
                head: Vec::new(),
                tail: None,
              }),
            );
          }
        }

        return;
      } else if arg_iter.operator_eq(&end_iter) {
        // Not enough arguments.

        // Might be ok if we are forwarding a vararg along.  This is a common thing to occur in nonstrict mode.
        if let Some(tail) = arg_iter.tail() {
          if !state
            .log
            .txn_log_get_mutable::<ErrorTypePack, TypePackId>(tail)
            .is_null()
          {
            // Unify remaining parameters so we don't leave any free-type_arguments hanging around.
            while param_iter.operator_ne(&end_iter) {
              let er = self.error_recovery_type_type_id(self.any_type);
              state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                er,
                *param_iter.operator_deref(),
                false,
                false,
                None,
              );
              param_iter.operator_inc();
            }
            return;
          } else if !state
            .log
            .txn_log_get_mutable::<VariadicTypePack, TypePackId>(tail)
            .is_null()
          {
            loop_count = 0;

            // Function is variadic and requires that all subsequent parameters
            // be compatible with a type.
            while param_iter.operator_ne(&end_iter) {
              let vtp = state
                .log
                .txn_log_get_mutable::<VariadicTypePack, TypePackId>(tail);
              state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                unsafe { (*vtp).ty },
                *param_iter.operator_deref(),
                false,
                false,
                None,
              );
              param_iter.operator_inc();

              if exceeds_loop_count!() {
                return;
              }
            }

            return;
          } else if !state
            .log
            .txn_log_get_mutable::<FreeTypePack, TypePackId>(tail)
            .is_null()
          {
            let mut rest: Vec<TypeId> = Vec::new();

            loop_count = 0;

            while param_iter.operator_ne(&end_iter) {
              rest.push(*param_iter.operator_deref());
              param_iter.operator_inc();

              if exceeds_loop_count!() {
                return;
              }
            }

            let var_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
              head: rest,
              tail: param_iter.tail(),
            }));
            state.try_unify_type_pack_id_type_pack_id_bool_entry(tail, var_pack, false);
            return;
          }
        }

        // If any remaining unfulfilled parameters are nonoptional, this is a problem.
        while param_iter.operator_ne(&end_iter) {
          let t = unsafe { state.log.follow_type_id(*param_iter.operator_deref()) };
          if is_optional(t) || !state.log.txn_log_get_mutable::<ErrorType, _>(t).is_null() {
            // ok
          } else {
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
          param_iter.operator_inc();
        }
      } else if param_iter.operator_eq(&end_iter) {
        // too many parameters passed
        if param_iter.tail().is_none() {
          loop_count = 0;

          while arg_iter.operator_ne(&end_iter) {
            // The use of unify here is deliberate. We don't want this unification
            // to be undoable.
            let er = self.error_recovery_type_scope_ptr(scope);
            self.unify_type_id_type_id_scope_ptr_location(
              er,
              *arg_iter.operator_deref(),
              scope,
              &state.location,
            );
            arg_iter.operator_inc();

            if exceeds_loop_count!() {
              return;
            }
          }
          report_count_mismatch_error!();
          return;
        }
        let tail = unsafe { state.log.follow_type_pack_id(param_iter.tail().unwrap()) };

        if !state
          .log
          .txn_log_get_mutable::<ErrorTypePack, TypePackId>(tail)
          .is_null()
        {
          // Function is variadic.  Ok.
          return;
        } else if !state
          .log
          .txn_log_get_mutable::<VariadicTypePack, TypePackId>(tail)
          .is_null()
        {
          loop_count = 0;

          // Function is variadic and requires that all subsequent parameters
          // be compatible with a type.
          let mut arg_index = param_index;
          while arg_iter.operator_ne(&end_iter) {
            let mut location = state.location;

            if arg_index < arg_locations.len() {
              location = arg_locations[arg_index];
            }

            state.location = location;
            let vtp = state
              .log
              .txn_log_get_mutable::<VariadicTypePack, TypePackId>(tail);
            state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
              *arg_iter.operator_deref(),
              unsafe { (*vtp).ty },
              false,
              false,
              None,
            );

            arg_iter.operator_inc();
            arg_index += 1;

            if exceeds_loop_count!() {
              return;
            }
          }

          return;
        } else if !state
          .log
          .txn_log_get_mutable::<FreeTypePack, TypePackId>(tail)
          .is_null()
        {
          loop_count = 0;

          // Create a type pack out of the remaining argument type_arguments
          // and unify it with the tail.
          let mut rest: Vec<TypeId> = Vec::new();
          while arg_iter.operator_ne(&end_iter) {
            rest.push(*arg_iter.operator_deref());
            arg_iter.operator_inc();

            if exceeds_loop_count!() {
              return;
            }
          }

          let var_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
            head: rest,
            tail: arg_iter.tail(),
          }));
          state.try_unify_type_pack_id_type_pack_id_bool_entry(var_pack, tail, false);

          return;
        } else if !state
          .log
          .txn_log_get_mutable::<FreeTypePack, TypePackId>(tail)
          .is_null()
        {
          state.log.replace_type_pack_id_type_pack_var(
            tail,
            TypePackVar::from(TypePack {
              head: Vec::new(),
              tail: None,
            }),
          );
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
        if FFlag::LuauInstantiateInSubtyping.get() {
          state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
            *arg_iter.operator_deref(),
            *param_iter.operator_deref(),
            false, // isFunctionCall
            false,
            None,
          );
        } else {
          let sub = *arg_iter.operator_deref();
          let sup = *param_iter.operator_deref();
          self.unify_with_instantiation_if_needed_type_id_type_id_scope_ptr_unifier(
            sub,
            sup,
            scope.clone(),
            state,
          );
        }
        arg_iter.operator_inc();
        param_iter.operator_inc();
      }

      param_index += 1;
    }
  }
}
