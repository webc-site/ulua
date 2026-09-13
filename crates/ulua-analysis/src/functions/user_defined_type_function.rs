//! Node: `cxx:Function:Luau.Analysis:Analysis/src/UserDefinedTypeFunction.cpp:219:userDefinedTypeFunction`
//! Source: `Analysis/src/UserDefinedTypeFunction.cpp:219-494`
//!
//! Faithful port of `TypeFunctionReductionResult<TypeId> userDefinedTypeFunction(...)`.
//! Evaluates a user-defined ("user") type function by running its compiled body
//! on a sandboxed Luau VM thread: it checks for blocking (pending) types,
//! registers the visible environment, serializes the type arguments into Lua
//! type userdata, calls the function under `lua_pcall`, then deserializes the
//! returned type userdata back into a `TypeId`.
//!
//! Signature is aligned to the canonical `ReducerFunction` fn-pointer shape
//! (by-value vecs + `*mut TypeFunctionContext`) so it can be wired into
//! `BuiltinTypeFunctions::user_func`.
// `FindUserTypeFunctionBlockers` overrides the bare type/type-pack `visit`s of
// its `TypeOnceVisitor` (`GenericTypeVisitor`) base. To make the base's
// `traverse(...)` dispatch through those overrides, the visitor must implement
// `GenericTypeVisitorTrait`. The overrides already exist as inherent methods
// (see `methods/find_user_type_function_blockers_visit_user_defined_type_function*`);
// this trait impl just forwards to them.
use alloc::{
  boxed::Box,
  ffi::CString,
  string::{String, ToString},
  vec::Vec,
};
use core::{
  ffi,
  ffi::{CStr, c_int, c_void},
  ptr::NonNull,
};

use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;
use ulua_common::{
  FFlag,
  functions::{format::format, get_clock::get_clock},
  records::dense_hash_set::DenseHashSet,
};
use ulua_vm::{
  functions::{
    lua_callbacks::lua_callbacks, lua_getfenv::lua_getfenv, lua_gettable::lua_gettable,
    lua_getthreaddata::lua_getthreaddata, lua_mainthread::lua_mainthread,
    lua_newthread::lua_newthread, lua_pcall::lua_pcall, lua_setfield::lua_setfield,
    lua_setreadonly::lua_setreadonly,
  },
  macros::{
    LUA_PUSHCCLOSURE, LUA_PUSHLIGHTUSERDATA, lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_state,
};

use crate::{
  enums::reduction::Reduction,
  functions::{
    alloc_type_user_data::alloc_type_user_data, check_result_for_error::check_result_for_error,
    check_result_for_error_deprecated::check_result_for_error_deprecated,
    deserialize_type_function_runtime_builder::deserialize_type_function_type_id_type_function_runtime_builder_state,
    evaluate_type_alias_call::evaluate_type_alias_call, follow_type::follow,
    get_mutable_type::get_mutable, get_type_user_data::get_type_user_data, is_pending::is_pending,
    is_type_user_data::is_type_user_data, reset_type_function_state::reset_type_function_state,
    serialize_type_function_runtime_builder::serialize_type_id_type_function_runtime_builder_state,
    to_string_type_function_error::to_string,
  },
  records::{
    extern_type::ExternType,
    find_user_type_function_blockers::FindUserTypeFunctionBlockers,
    freeze_type_function_types::FreezeTypeFunctionTypes,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    luau_temp_thread_popper::LuauTempThreadPopper,
    scoped_assign::ScopedAssign,
    time_limit_error::TimeLimitError,
    type_fun::TypeFun,
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_result::TypeFunctionReductionResult,
    type_function_runtime::TypeFunctionRuntime,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
    type_function_type::TypeFunctionType,
    user_cancel_error::UserCancelError,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl GenericTypeVisitorTrait for FindUserTypeFunctionBlockers {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    FindUserTypeFunctionBlockers::visit_type_id(self, ty)
  }

  fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    FindUserTypeFunctionBlockers::visit_type_pack_id(self, tp)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    FindUserTypeFunctionBlockers::visit_type_id_extern_type(self, ty, etv)
  }
}

// Interrupt handler for type functions: respects type checking limits and LSP
// cancellation requests. C++ throws `TimeLimitError`/`UserCancelError`; the Rust
// port surfaces those via `panic!` (the same mechanism the ported throw-methods
// use), unwinding out of the VM call.
unsafe extern "C-unwind" fn user_defined_type_function_interrupt(
  l: *mut lua_state::LuaState,
  _gc: c_int,
) {
  unsafe {
    let main = lua_mainthread(l);
    let data = lua_getthreaddata(main);
    let ctx = data as *const TypeFunctionRuntime;

    if let Some(finish_time) = (*ctx).limits.finish_time
      && get_clock() > finish_time
    {
      panic!(
        "{}",
        TimeLimitError::time_limit_error_time_limit_error(&(*ctx).ice.module_name)
      );
    }

    if let Some(token) = &(*ctx).limits.cancellation_token
      && token.requested()
    {
      panic!("{}", UserCancelError::new((*ctx).ice.module_name.clone()));
    }
  }
}

pub fn user_defined_type_function(
  instance: TypeId,
  type_params: Vec<TypeId>,
  _pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // SAFETY: reducer 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效；
  // 函数体跨越 Luau VM FFI 边界，指针解引用集中于此块。
  unsafe {
    let ctx_ptr = ctx;

    // auto typeFunction = get_mutable<TypeFunctionInstanceType>(instance);
    // LUAU_ASSERT(typeFunction);
    let type_function = match get_mutable::<TypeFunctionInstanceType>(instance) {
      Some(tf) => tf,
      None => {
        // C++: LUAU_ASSERT(typeFunction) — 断言必命中；不会到达。
        ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
        return TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::Erroneous,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: None,
          messages: Vec::new(),
        };
      }
    };

    // if (typeFunction->userFuncData.owner.expired())
    if type_function.user_func_data.owner.upgrade().is_none() {
      (*ctx_ptr)
        .ice
        .as_ref()
        .ice_string("user-defined type function module has expired");
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    // if (!typeFunction->userFuncName || !typeFunction->userFuncData.definition)
    if type_function.user_func_name.is_none() || type_function.user_func_data.definition.is_null() {
      (*ctx_ptr)
        .ice
        .as_ref()
        .ice_string("all user-defined type functions must have an associated function definition");
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    let type_function_runtime = (*ctx_ptr).type_function_runtime.as_ptr();

    // If type functions cannot be evaluated because of errors in the code, we do not generate any additional ones
    // if (!ctx->typeFunctionRuntime->allowEvaluation || typeFunction->userFuncData.definition->hasErrors)
    if !(*type_function_runtime).allow_evaluation
      || (*type_function.user_func_data.definition).has_errors
    {
      return TypeFunctionReductionResult {
        result: Some(error_type(ctx_ptr)),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    // FindUserTypeFunctionBlockers check{ctx};
    let mut check = FindUserTypeFunctionBlockers::new(NonNull::new_unchecked(ctx_ptr));

    // for (auto typeParam : typeParams) check.traverse(follow(typeParam));
    for &type_param in type_params.iter() {
      check.traverse_type_id(follow(type_param));
    }

    // Check that our environment doesn't depend on any type aliases that are blocked
    // for (auto& [name, definition] : typeFunction->userFuncData.environmentAlias)
    //     if (definition.first->typeParams.empty() && definition.first->typePackParams.empty())
    //         check.traverse(follow(definition.first->type));
    {
      let alias_entries: Vec<*mut TypeFun> = type_function
        .user_func_data
        .environment_alias
        .iter()
        .map(|(_name, def)| def.0)
        .collect();
      for tf in alias_entries {
        if (*tf).type_params().is_empty() && (*tf).type_pack_params().is_empty() {
          check.traverse_type_id(follow((*tf).r#type()));
        }
      }
    }

    // if (!check.blockingTypes.empty())
    //     return {std::nullopt, Reduction::MaybeOk, check.blockingTypes, {}};
    if !check.blocking_types.is_empty() {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::MaybeOk,
        blocked_types: check.blocking_types.clone(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    // Ensure that whole type function environment is registered
    // for (auto& [name, definition] : typeFunction->userFuncData.environmentFunction)
    {
      let func_entries: Vec<(*mut AstStatTypeFunction, usize)> = type_function
        .user_func_data
        .environment_function
        .iter()
        .map(|(_name, def)| (def.0, def.1))
        .collect();

      for (def_ptr, _depth) in func_entries {
        // Cannot evaluate if a potential dependency couldn't be parsed
        // if (definition.first->hasErrors)
        if (*def_ptr).has_errors {
          return TypeFunctionReductionResult {
            result: Some(error_type(ctx_ptr)),
            reduction_status: Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
          };
        }

        // bool registrationFailed = ... registerFunction(definition.first).has_value()
        let registration_failed = if FFlag::LuauTypeFunctionStructuredErrors.get() {
          (*type_function_runtime)
            .register_function(def_ptr)
            .is_some()
        } else {
          (*type_function_runtime)
            .register_function_deprecated(def_ptr)
            .is_some()
        };
        if registration_failed {
          // Failure to register at this point means that original definition had to error out and should not
          // have been present in the environment
          (*ctx_ptr)
            .ice
            .as_ref()
            .ice_string("user-defined type function reference cannot be registered");
          return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
          };
        }
      }
    }

    // AstName name = typeFunction->userFuncData.definition->name;
    let name = (*type_function.user_func_data.definition).name;
    let name_str = ast_name_to_string(name.value);

    // lua_State* global = ctx->typeFunctionRuntime->state.get();
    let global = (*type_function_runtime).state.0;

    // if (global == nullptr)
    //     return {..., format("'%s' type function: cannot be evaluated in this context", name.value)};
    if global.is_null() {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: Some(format(format_args!(
          "'{}' type function: cannot be evaluated in this context",
          name_str
        ))),
        messages: Vec::new(),
      };
    }

    // Separate sandboxed thread for individual execution and private globals
    // lua_State* l = lua_newthread(global);
    // luau_temp_thread_popper popper(global);
    let l_vm = lua_newthread(global as *mut lua_state::LuaState);
    let l = l_vm as *mut LuaState;
    let mut popper = LuauTempThreadPopper::new(global);

    // std::unique_ptr<TypeFunctionRuntimeBuilderState> runtimeBuilder = std::make_unique<...>(ctx);
    let mut runtime_builder: Box<TypeFunctionRuntimeBuilderState> =
      Box::new(TypeFunctionRuntimeBuilderState::new(ctx_ptr));
    let runtime_builder_ptr: *mut TypeFunctionRuntimeBuilderState = runtime_builder.as_mut();

    // ScopedAssign setRuntimeBuilder(ctx->typeFunctionRuntime->runtimeBuilder, runtimeBuilder.get());
    let _set_runtime_builder = ScopedAssign::new(
      &mut (*type_function_runtime).runtime_builder,
      runtime_builder_ptr,
    );
    // ScopedAssign enableReduction(ctx->normalizer->sharedState->reentrantTypeReduction, false);
    let shared_state = (*(*ctx_ptr).normalizer.as_ptr()).shared_state;
    let _enable_reduction = ScopedAssign::new(&mut (*shared_state).reentrant_type_reduction, false);

    // Build up the environment table of each function we have visible
    // for (auto& [_, curr] : typeFunction->userFuncData.environmentFunction)
    let curr_entries: Vec<(*mut AstStatTypeFunction, usize)> = type_function
      .user_func_data
      .environment_function
      .iter()
      .map(|(_name, def)| (def.0, def.1))
      .collect();

    for (curr_ptr, curr_depth) in curr_entries {
      // Environment table has to be filled only once in the current execution context
      // if (ctx->typeFunctionRuntime->initialized.find(curr.first)) continue;
      if (*type_function_runtime)
        .initialized
        .find(&curr_ptr)
        .is_some()
      {
        continue;
      }
      // ctx->typeFunctionRuntime->initialized.insert(curr.first);
      (*type_function_runtime).initialized.insert(curr_ptr);

      // LUA_PUSHLIGHTUSERDATA(l, curr.first);
      // lua_gettable(l, LUA_REGISTRYINDEX);
      LUA_PUSHLIGHTUSERDATA::LUA_PUSHLIGHTUSERDATA(l, curr_ptr as *mut c_void);
      lua_gettable(l_vm, LUA_REGISTRYINDEX);

      // if (!lua_isfunction(l, -1))
      if !ulua_vm::lua_isfunction!(l_vm, -1) {
        (*ctx_ptr)
          .ice
          .as_ref()
          .ice_string("user-defined type function reference cannot be found in the registry");
        return TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::Erroneous,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: None,
          messages: Vec::new(),
        };
      }

      // Build up the environment of the current function, where some might not be visible
      // lua_getfenv(l, -1);
      // lua_setreadonly(l, -1, false);
      lua_getfenv(l_vm, -1);
      lua_setreadonly(l_vm, -1, 0);

      // for (auto& [name, definition] : typeFunction->userFuncData.environmentFunction)
      let func_env: Vec<(CString, *mut AstStatTypeFunction, usize)> = type_function
        .user_func_data
        .environment_function
        .iter()
        .map(|(name, def)| (CString::new(name.as_bytes()).unwrap(), def.0, def.1))
        .collect();

      for (name_c, def_ptr, def_depth) in func_env.iter() {
        // Filter visibility based on original scope depth
        // if (definition.second >= curr.second)
        if *def_depth >= curr_depth {
          // LUA_PUSHLIGHTUSERDATA(l, definition.first);
          // lua_gettable(l, LUA_REGISTRYINDEX);
          LUA_PUSHLIGHTUSERDATA::LUA_PUSHLIGHTUSERDATA(l, *def_ptr as *mut c_void);
          lua_gettable(l_vm, LUA_REGISTRYINDEX);

          // if (!lua_isfunction(l, -1)) break;
          if !ulua_vm::lua_isfunction!(l_vm, -1) {
            break; // Don't have to report an error here, we will visit each function in outer loop
          }

          // lua_setfield(l, -2, name.c_str());
          lua_setfield(l_vm, -2, name_c.as_ptr());
        }
      }

      // for (auto& [name, definition] : typeFunction->userFuncData.environmentAlias)
      let alias_env: Vec<(CString, *mut TypeFun, usize)> = type_function
        .user_func_data
        .environment_alias
        .iter()
        .map(|(name, def)| (CString::new(name.as_bytes()).unwrap(), def.0, def.1))
        .collect();

      for (name_c, def_ptr, def_depth) in alias_env.iter() {
        // Filter visibility based on original scope depth
        // if (definition.second >= curr.second)
        if *def_depth >= curr_depth {
          // if (definition.first->typeParams.empty() && definition.first->typePackParams.empty())
          if (**def_ptr).type_params().is_empty() && (**def_ptr).type_pack_params().is_empty() {
            // TypeId ty = follow(definition.first->type);
            let ty = follow((**def_ptr).r#type());

            // This is checked at the top of the function, and should still be true.
            // LUAU_ASSERT(!isPending(ty, ctx->solver));
            ulua_common::macros::luau_assert::LUAU_ASSERT!(!is_pending(ty, (*ctx_ptr).solver));

            // TypeFunctionTypeId serializedTy = serialize(ty, runtimeBuilder.get());
            let serialized_ty: TypeFunctionTypeId =
              serialize_type_id_type_function_runtime_builder_state(ty, runtime_builder_ptr);

            if FFlag::LuauTypeFunctionRobustness.get() {
              // Only register aliases that are representable in type environment
              // if (serializedTy && (... ? errors.empty() : errors_DEPRECATED.empty()))
              let errors_empty = if FFlag::LuauTypeFunctionStructuredErrors.get() {
                runtime_builder.errors.is_empty()
              } else {
                runtime_builder.errors_deprecated.is_empty()
              };
              if !serialized_ty.is_null() && errors_empty {
                if FFlag::LuauTypeFunctionSupportsFrozen.get() {
                  let mut freezer = FreezeTypeFunctionTypes::new();
                  freezer.base.run_type_function_type_id(serialized_ty);
                }

                // allocTypeUserData(l, serializedTy->type, /* frozen */ true);
                let variant = (*(serialized_ty as *mut TypeFunctionType))
                  .type_variant
                  .clone();
                alloc_type_user_data(l, variant, true);
                // lua_setfield(l, -2, name.c_str());
                lua_setfield(l_vm, -2, name_c.as_ptr());
              }
            } else {
              if FFlag::LuauTypeFunctionSupportsFrozen.get() {
                let mut freezer = FreezeTypeFunctionTypes::new();
                freezer.base.run_type_function_type_id(serialized_ty);
              }

              // Only register aliases that are representable in type environment
              let errors_empty = if FFlag::LuauTypeFunctionStructuredErrors.get() {
                runtime_builder.errors.is_empty()
              } else {
                runtime_builder.errors_deprecated.is_empty()
              };
              if errors_empty {
                let variant = (*(serialized_ty as *mut TypeFunctionType))
                  .type_variant
                  .clone();
                alloc_type_user_data(l, variant, true);
                lua_setfield(l_vm, -2, name_c.as_ptr());
              }
            }
          } else {
            // LUA_PUSHLIGHTUSERDATA(l, definition.first);
            // LUA_PUSHCCLOSURE(l, evaluateTypeAliasCall, name.c_str(), 1);
            // lua_setfield(l, -2, name.c_str());
            LUA_PUSHLIGHTUSERDATA::LUA_PUSHLIGHTUSERDATA(l, *def_ptr as *mut c_void);
            LUA_PUSHCCLOSURE::LUA_PUSHCCLOSURE(
              l_vm,
              Some(evaluate_type_alias_call_thunk),
              name_c.as_ptr(),
              1,
            );
            lua_setfield(l_vm, -2, name_c.as_ptr());
          }
        }
      }

      // lua_setreadonly(l, -1, true);
      // lua_pop(l, 2);
      lua_setreadonly(l_vm, -1, 1);
      lua_pop(l_vm, 2);
    }

    // Fetch the function we want to evaluate
    // LUA_PUSHLIGHTUSERDATA(l, typeFunction->userFuncData.definition);
    // lua_gettable(l, LUA_REGISTRYINDEX);
    LUA_PUSHLIGHTUSERDATA::LUA_PUSHLIGHTUSERDATA(
      l,
      type_function.user_func_data.definition as *mut c_void,
    );
    lua_gettable(l_vm, LUA_REGISTRYINDEX);

    // if (!lua_isfunction(l, -1))
    if !ulua_vm::lua_isfunction!(l_vm, -1) {
      (*ctx_ptr)
        .ice
        .as_ref()
        .ice_string("user-defined type function reference cannot be found in the registry");
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    // resetTypeFunctionState(l);
    reset_type_function_state(l);

    // Push serialized arguments onto the stack
    // for (auto typeParam : typeParams)
    for &type_param in type_params.iter() {
      // TypeId ty = follow(typeParam);
      let ty = follow(type_param);
      // LUAU_ASSERT(!isPending(ty, ctx->solver));
      ulua_common::macros::luau_assert::LUAU_ASSERT!(!is_pending(ty, (*ctx_ptr).solver));

      // TypeFunctionTypeId serializedTy = serialize(ty, runtimeBuilder.get());
      let serialized_ty: TypeFunctionTypeId =
        serialize_type_id_type_function_runtime_builder_state(ty, runtime_builder_ptr);

      // Check if there were any errors while serializing
      if FFlag::LuauTypeFunctionStructuredErrors.get() {
        if !runtime_builder.errors.is_empty() {
          return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: Some(to_string(&runtime_builder.errors[0])),
            messages: Vec::new(),
          };
        }
      } else {
        if !runtime_builder.errors_deprecated.is_empty() {
          return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: Some(runtime_builder.errors_deprecated[0].clone()),
            messages: Vec::new(),
          };
        }
      }

      // if (FFlag::LuauTypeFunctionRobustness && !serializedTy)
      if FFlag::LuauTypeFunctionRobustness.get() && serialized_ty.is_null() {
        return TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::Erroneous,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: Some(
            "Complexity limit reached when passing a type to a type function".to_string(),
          ),
          messages: Vec::new(),
        };
      }

      // allocTypeUserData(l, serializedTy->type);
      let variant = (*(serialized_ty as *mut TypeFunctionType))
        .type_variant
        .clone();
      alloc_type_user_data(l, variant, false);
    }

    // Set up an interrupt handler for type functions to respect type checking limits and LSP cancellation requests.
    // lua_callbacks(l)->interrupt = [](lua_State* l, int gc) { ... };
    (*lua_callbacks(l_vm)).interrupt = Some(user_defined_type_function_interrupt);

    // ctx->typeFunctionRuntime->messages.clear();
    (*type_function_runtime).messages.clear();

    // lua_pcall(l, int(typeParams.size()), 1, 0)
    let pcall_result = lua_pcall(l_vm, type_params.len() as i32, 1, 0);

    let result: TypeFunctionReductionResult;

    if FFlag::LuauTypeFunctionStructuredErrors.get() {
      // if (auto error = checkResultForError(l, name.value, lua_pcall(...)))
      //     return {..., to_string(*error), ctx->typeFunctionRuntime->messages};
      if let Some(error) = check_result_for_error(l, &name_str, pcall_result) {
        return TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::Erroneous,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: Some(to_string(&error)),
          messages: (*type_function_runtime).messages.clone(),
        };
      }
    } else {
      // if (auto error = checkResultForError_DEPRECATED(l, name.value, lua_pcall(...)))
      //     return {..., std::move(error), ctx->typeFunctionRuntime->messages};
      if let Some(error) = check_result_for_error_deprecated(l, &name_str, pcall_result) {
        return TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::Erroneous,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: Some(error),
          messages: (*type_function_runtime).messages.clone(),
        };
      }
    }

    // If the return value is not a type userdata, return with error message
    // if (!isTypeUserData(l, 1))
    if !is_type_user_data(l, 1) {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: Some(format(format_args!(
          "'{}' type function: returned a non-type value",
          name_str
        ))),
        messages: (*type_function_runtime).messages.clone(),
      };
    }

    // TypeFunctionTypeId retTypeFunctionTypeId = getTypeUserData(l, 1);
    let ret_type_function_type_id: TypeFunctionTypeId = get_type_user_data(l, 1);

    if FFlag::LuauTypeFunctionStructuredErrors.get() {
      // No errors should be present here since we should've returned already if any were raised during serialization.
      // LUAU_ASSERT(runtimeBuilder->errors.empty());
      ulua_common::macros::luau_assert::LUAU_ASSERT!(runtime_builder.errors.is_empty());

      // TypeId retTypeId = deserialize(retTypeFunctionTypeId, runtimeBuilder.get());
      let ret_type_id = deserialize_type_function_type_id_type_function_runtime_builder_state(
        ret_type_function_type_id,
        runtime_builder_ptr,
      );

      // At least 1 error occurred while deserializing
      // if (!runtimeBuilder->errors.empty())
      if !runtime_builder.errors.is_empty() {
        result = TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::Erroneous,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: Some(to_string(&runtime_builder.errors[0])),
          messages: (*type_function_runtime).messages.clone(),
        };
      } else {
        result = TypeFunctionReductionResult {
          result: Some(ret_type_id),
          reduction_status: Reduction::MaybeOk,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: None,
          messages: (*type_function_runtime).messages.clone(),
        };
      }
    } else {
      // LUAU_ASSERT(runtimeBuilder->errors_DEPRECATED.size() == 0);
      ulua_common::macros::luau_assert::LUAU_ASSERT!(runtime_builder.errors_deprecated.is_empty());

      // TypeId retTypeId = deserialize(retTypeFunctionTypeId, runtimeBuilder.get());
      let ret_type_id = deserialize_type_function_type_id_type_function_runtime_builder_state(
        ret_type_function_type_id,
        runtime_builder_ptr,
      );

      // if (runtimeBuilder->errors_DEPRECATED.size() > 0)
      if !runtime_builder.errors_deprecated.is_empty() {
        result = TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::Erroneous,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: Some(runtime_builder.errors_deprecated[0].clone()),
          messages: (*type_function_runtime).messages.clone(),
        };
      } else {
        result = TypeFunctionReductionResult {
          result: Some(ret_type_id),
          reduction_status: Reduction::MaybeOk,
          blocked_types: Vec::new(),
          blocked_packs: Vec::new(),
          error: None,
          messages: (*type_function_runtime).messages.clone(),
        };
      }
    }

    // C++ `luau_temp_thread_popper` pops the temp thread in its destructor. The
    // Rust port models the destructor as an explicit method (no Drop impl),
    // so invoke it here at the single success exit, mirroring scope-end RAII.
    popper.luau_temp_thread_popper();
    result
  }
}

/// Helper: read an `AstName.value` (`*const c_char`) into an owned `String`.
unsafe fn ast_name_to_string(value: *const ffi::c_char) -> String {
  unsafe {
    if value.is_null() {
      String::new()
    } else {
      CStr::from_ptr(value).to_string_lossy().into_owned()
    }
  }
}

/// Helper: `ctx->builtins->error_type`.
unsafe fn error_type(ctx_ptr: *mut TypeFunctionContext) -> TypeId {
  unsafe { (*(*ctx_ptr).builtins.as_ptr()).error_type }
}

/// `LuaCfunction` thunk for `evaluateTypeAliasCall`. The Closure is registered
/// via `LUA_PUSHCCLOSURE`, which expects a `LuaCfunction`
/// (`Option<unsafe fn(*mut LuaState) -> c_int>`, Rust ABI).
unsafe extern "C-unwind" fn evaluate_type_alias_call_thunk(l: *mut lua_state::LuaState) -> c_int {
  unsafe { evaluate_type_alias_call(l as *mut LuaState) }
}
