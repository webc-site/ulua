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
//! (by-value vecs + `&mut TypeFunctionContext`) so it can be wired into
//! `BuiltinTypeFunctions::user_func`.
// `FindUserTypeFunctionBlockers` overrides the bare type/type-pack `visit`s of
// its `TypeOnceVisitor` (`GenericTypeVisitor`) base. To make the base's
// `traverse(...)` dispatch through those overrides, the visitor must implement
// `GenericTypeVisitorTrait`. The overrides already exist as inherent methods
// (see `methods/find_user_type_function_blockers_visit_user_defined_type_function*`);
// this trait impl just forwards to them.
use alloc::{
  boxed::Box,
  string::{String, ToString},
  vec::Vec,
};

use ulua_ast::records::ast_name::AstName;
use ulua_common::{
  fflag,
  functions::{c_str::with_c_str, format::format, get_clock::get_clock},
  macros::luau_assert::LUAU_ASSERT,
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
    lua_pop::lua_pop, lua_pushcclosure::lua_pushcclosure,
    lua_pushlightuserdata::lua_pushlightuserdata, lua_registryindex::LUA_REGISTRYINDEX,
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
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_result::TypeFunctionReductionResult,
    type_function_runtime::TypeFunctionRuntime,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
    type_function_type::TypeFunctionType,
    user_cancel_error::UserCancelError,
    visit_key::VisitKey,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl GenericTypeVisitorTrait for FindUserTypeFunctionBlockers<'_> {
  type Seen = DenseHashSet<VisitKey>;

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
  _gc: i32,
) {
  // Safety: 本回调只由 VM 在 `user_defined_type_function` 内通过 `lua_callbacks(l_vm)`
  // 安装（且只在同一次 `lua_pcall` 期间生效），因此调用方是按 Lua C ABI 契约以
  // 「当前运行线程 + gc 阶段码」传入参数：`l` 必为非空 `lua_State*`。
  // `lua_mainthread(l)` 对任意协程都返回其主线程（永不为空），而该主线程的 thread
  // data 在 `TypeFunctionRuntime::prepare_state` 里无条件写入 `self as *mut
  // TypeFunctionRuntime`，故 `lua_getthreaddata` 返回非空且指向活的 runtime ——
  // runtime 拥有该 VM（`state` 字段），必然比在其上执行的回调长寿。随后只读
  // `limits`/`ice` 两个字段做超时与取消判定，不写任何共享状态；panic 只用于按
  // C++ 抛 TimeLimitError/UserCancelError 的语义展开出 VM。
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
  type_params: &[TypeId],
  _pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // SAFETY: 分派点物化的本次调用独占借用；函数体跨越 Luau VM FFI 边界，指针解引用
  // 集中于此块，故先将其降级为存续整块的裸句柄（借用期内无其它访问路径）。
  unsafe {
    let ctx_ptr: *mut TypeFunctionContext = ctx;

    // auto typeFunction = get_mutable<TypeFunctionInstanceType>(instance);
    // LUAU_ASSERT(typeFunction);
    let type_function = match get_mutable::<TypeFunctionInstanceType>(instance) {
      Some(tf) => tf,
      None => {
        // C++: LUAU_ASSERT(typeFunction) — 断言必命中；不会到达。
        LUAU_ASSERT!(false);
        return erroneous_result();
      }
    };

    // if (typeFunction->userFuncData.owner.expired())
    if type_function.user_func_data.owner.upgrade().is_none() {
      (*ctx_ptr)
        .ice
        .as_ref()
        .ice_string("user-defined type function module has expired");
      return erroneous_result();
    }

    // if (!typeFunction->userFuncName || !typeFunction->userFuncData.definition)
    if type_function.user_func_name.is_none() || type_function.user_func_data.definition.is_null() {
      (*ctx_ptr)
        .ice
        .as_ref()
        .ice_string("all user-defined type functions must have an associated function definition");
      return erroneous_result();
    }

    let type_function_runtime = (*ctx_ptr).type_function_runtime.as_ptr();

    // If type functions cannot be evaluated because of errors in the code, we do not generate any additional ones
    // if (!ctx->typeFunctionRuntime->allowEvaluation || typeFunction->userFuncData.definition->hasErrors)
    if !(*type_function_runtime).allow_evaluation
      || (*type_function.user_func_data.definition).has_errors
    {
      return unevaluable_result(ctx_ptr);
    }

    // FindUserTypeFunctionBlockers check{ctx};
    // cpp 的 `NotNull<TypeFunctionContext>` 实参即本帧独占借用的会话 ctx：访问者只在
    // 紧随其后的阻塞检查段内存活，故直接下传 `&mut` 借用（非空与存活性由类型保证，
    // 原 `NonNull::new_unchecked` 的 unchecked 构造随之消失）。
    let mut check = FindUserTypeFunctionBlockers::new(ctx);

    // for (auto typeParam : typeParams) check.traverse(follow(typeParam));
    for &type_param in type_params {
      check.traverse_type_id(follow(type_param));
    }

    // Check that our environment doesn't depend on any type aliases that are blocked
    // for (auto& [name, definition] : typeFunction->userFuncData.environmentAlias)
    //     if (definition.first->typeParams.empty() && definition.first->typePackParams.empty())
    //         check.traverse(follow(definition.first->type));
    // 环境映射只在求值期被读取（写入全部发生在 constraint generator 建实例阶段），
    // 故直接迭代，省掉一次快照 Vec 分配。
    for def_ptr in type_function
      .user_func_data
      .environment_alias
      .iter()
      .map(|(_, def)| def.0)
    {
      if (*def_ptr).type_params().is_empty() && (*def_ptr).type_pack_params().is_empty() {
        check.traverse_type_id(follow((*def_ptr).r#type()));
      }
    }

    // if (!check.blockingTypes.empty())
    //     return {std::nullopt, Reduction::MaybeOk, check.blockingTypes, {}};
    if !check.blocking_types.is_empty() {
      return TypeFunctionReductionResult::no_reduction(check.blocking_types.clone());
    }

    // Ensure that whole type function environment is registered
    // for (auto& [name, definition] : typeFunction->userFuncData.environmentFunction)
    for def_ptr in type_function
      .user_func_data
      .environment_function
      .iter()
      .map(|(_, def)| def.0)
    {
      // Cannot evaluate if a potential dependency couldn't be parsed
      // if (definition.first->hasErrors)
      if (*def_ptr).has_errors {
        return unevaluable_result(ctx_ptr);
      }

      // bool registrationFailed = ... registerFunction(definition.first).has_value()
      let registration_failed = if fflag::LuauTypeFunctionStructuredErrors.get() {
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
        return erroneous_result();
      }
    }

    // AstName name = typeFunction->userFuncData.definition->name;
    let name = (*type_function.user_func_data.definition).name;
    let name_str = ast_name_to_string(name);

    // lua_State* global = ctx->typeFunctionRuntime->state.get();
    let global = (*type_function_runtime).state.0;

    // if (global == nullptr)
    //     return {..., format("'%s' type function: cannot be evaluated in this context", name.value)};
    if global.is_null() {
      return erroneous_with(
        format(format_args!(
          "'{}' type function: cannot be evaluated in this context",
          name_str
        )),
        Vec::new(),
      );
    }

    // Separate sandboxed thread for individual execution and private globals
    // lua_State* l = lua_newthread(global);
    // luau_temp_thread_popper popper(global);
    let l_vm = lua_newthread(global as *mut lua_state::LuaState);
    let l = l_vm as *mut LuaState;
    let mut popper = LuauTempThreadPopper::new(global);

    // std::unique_ptr<TypeFunctionRuntimeBuilderState> runtimeBuilder = std::make_unique<...>(ctx);
    // builder state 的 ctx 由本帧的独占会话借用物化（句柄非空由类型编码，无 null 分支）。
    let mut runtime_builder: Box<TypeFunctionRuntimeBuilderState> =
      Box::new(TypeFunctionRuntimeBuilderState::new(ctx));
    let runtime_builder_ptr: *mut TypeFunctionRuntimeBuilderState = runtime_builder.as_mut();

    // ScopedAssign setRuntimeBuilder(ctx->typeFunctionRuntime->runtimeBuilder, runtimeBuilder.get());
    let _set_runtime_builder = ScopedAssign::new(
      &mut (*type_function_runtime).runtime_builder,
      runtime_builder_ptr,
    );
    // ScopedAssign enableReduction(ctx->normalizer->sharedState->reentrantTypeReduction, false);
    let shared_state = (*(*ctx_ptr).normalizer.as_ptr()).shared_state_ptr();
    let _enable_reduction = ScopedAssign::new(&mut (*shared_state).reentrant_type_reduction, false);

    // Build up the environment table of each function we have visible
    // for (auto& [_, curr] : typeFunction->userFuncData.environmentFunction)
    for (curr_ptr, curr_depth) in type_function
      .user_func_data
      .environment_function
      .iter()
      .map(|(_, def)| (def.0, def.1))
    {
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
      lua_pushlightuserdata(l_vm, curr_ptr.cast());
      lua_gettable(l_vm, LUA_REGISTRYINDEX);

      // if (!lua_isfunction(l, -1))
      if !ulua_vm::lua_isfunction!(l_vm, -1) {
        (*ctx_ptr)
          .ice
          .as_ref()
          .ice_string("user-defined type function reference cannot be found in the registry");
        return erroneous_result();
      }

      // Build up the environment of the current function, where some might not be visible
      // lua_getfenv(l, -1);
      // lua_setreadonly(l, -1, false);
      lua_getfenv(l_vm, -1);
      lua_setreadonly(l_vm, -1, 0);

      // for (auto& [name, definition] : typeFunction->userFuncData.environmentFunction)
      // 键名统一走 `set_env_field`：它把「字节串 → 补 NUL → `lua_setfield`」这一
      // `*const c_char` 收口集中到唯一一处。
      for (name, def_ptr, def_depth) in type_function
        .user_func_data
        .environment_function
        .iter()
        .map(|(n, d)| (n, d.0, d.1))
      {
        // Filter visibility based on original scope depth
        // if (definition.second >= curr.second)
        if def_depth >= curr_depth {
          // LUA_PUSHLIGHTUSERDATA(l, definition.first);
          // lua_gettable(l, LUA_REGISTRYINDEX);
          lua_pushlightuserdata(l_vm, def_ptr.cast());
          lua_gettable(l_vm, LUA_REGISTRYINDEX);

          // if (!lua_isfunction(l, -1)) break;
          if !ulua_vm::lua_isfunction!(l_vm, -1) {
            break; // Don't have to report an error here, we will visit each function in outer loop
          }

          // lua_setfield(l, -2, name.c_str());
          set_env_field(l_vm, name);
        }
      }

      // for (auto& [name, definition] : typeFunction->userFuncData.environmentAlias)
      for (name, def_ptr, def_depth) in type_function
        .user_func_data
        .environment_alias
        .iter()
        .map(|(n, d)| (n, d.0, d.1))
      {
        // Filter visibility based on original scope depth
        // if (definition.second >= curr.second)
        if def_depth >= curr_depth {
          // if (definition.first->typeParams.empty() && definition.first->typePackParams.empty())
          if (*def_ptr).type_params().is_empty() && (*def_ptr).type_pack_params().is_empty() {
            // TypeId ty = follow(definition.first->type);
            let ty = follow((*def_ptr).r#type());

            // This is checked at the top of the function, and should still be true.
            // LUAU_ASSERT(!isPending(ty, ctx->solver));
            LUAU_ASSERT!(!is_pending(ty, (*ctx_ptr).solver));

            // TypeFunctionTypeId serializedTy = serialize(ty, runtimeBuilder.get());
            let serialized_ty: TypeFunctionTypeId =
              serialize_type_id_type_function_runtime_builder_state(ty, runtime_builder_ptr);

            if fflag::LuauTypeFunctionRobustness.get() {
              // Only register aliases that are representable in type environment
              // if (serializedTy && (... ? errors.empty() : errors_DEPRECATED.empty()))
              let errors_empty = if fflag::LuauTypeFunctionStructuredErrors.get() {
                runtime_builder.errors.is_empty()
              } else {
                runtime_builder.errors_deprecated.is_empty()
              };
              if !serialized_ty.is_null() && errors_empty {
                if fflag::LuauTypeFunctionSupportsFrozen.get() {
                  let mut freezer = FreezeTypeFunctionTypes::new();
                  freezer.base.run_type_function_type_id(serialized_ty);
                }

                // allocTypeUserData(l, serializedTy->type, /* frozen */ true);
                let variant = (*(serialized_ty as *mut TypeFunctionType))
                  .type_variant
                  .clone();
                alloc_type_user_data(l, variant, true);
                // lua_setfield(l, -2, name.c_str());
                set_env_field(l_vm, name);
              }
            } else {
              if fflag::LuauTypeFunctionSupportsFrozen.get() {
                let mut freezer = FreezeTypeFunctionTypes::new();
                freezer.base.run_type_function_type_id(serialized_ty);
              }

              // Only register aliases that are representable in type environment
              let errors_empty = if fflag::LuauTypeFunctionStructuredErrors.get() {
                runtime_builder.errors.is_empty()
              } else {
                runtime_builder.errors_deprecated.is_empty()
              };
              if errors_empty {
                let variant = (*(serialized_ty as *mut TypeFunctionType))
                  .type_variant
                  .clone();
                alloc_type_user_data(l, variant, true);
                set_env_field(l_vm, name);
              }
            }
          } else {
            // LUA_PUSHLIGHTUSERDATA(l, definition.first);
            // LUA_PUSHCCLOSURE(l, evaluateTypeAliasCall, name.c_str(), 1);
            // lua_setfield(l, -2, name.c_str());
            // 一次补 NUL 同时喂 closure 的 debugname 与 setfield 的键：两个 callee
            // 都在调用期内把串 `lua_s_new` 驻留进 intern 表，指针不外存。
            lua_pushlightuserdata(l_vm, def_ptr.cast());
            with_c_str(name.as_bytes(), |c_name| {
              lua_pushcclosure(l_vm, Some(evaluate_type_alias_call_thunk), c_name, 1);
              lua_setfield(l_vm, -2, c_name);
            });
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
    lua_pushlightuserdata(
      l_vm,
      (type_function.user_func_data.definition as *mut ()).cast(),
    );
    lua_gettable(l_vm, LUA_REGISTRYINDEX);

    // if (!lua_isfunction(l, -1))
    if !ulua_vm::lua_isfunction!(l_vm, -1) {
      (*ctx_ptr)
        .ice
        .as_ref()
        .ice_string("user-defined type function reference cannot be found in the registry");
      return erroneous_result();
    }

    // resetTypeFunctionState(l);
    reset_type_function_state(l);

    // Push serialized arguments onto the stack
    // for (auto typeParam : typeParams)
    for &type_param in type_params {
      // TypeId ty = follow(typeParam);
      let ty = follow(type_param);
      // LUAU_ASSERT(!isPending(ty, ctx->solver));
      LUAU_ASSERT!(!is_pending(ty, (*ctx_ptr).solver));

      // TypeFunctionTypeId serializedTy = serialize(ty, runtimeBuilder.get());
      let serialized_ty: TypeFunctionTypeId =
        serialize_type_id_type_function_runtime_builder_state(ty, runtime_builder_ptr);

      // Check if there were any errors while serializing
      // structured / deprecated 两条错误通道的首个错误串（其余字段完全相同，故合一处）。
      let serialize_error = if fflag::LuauTypeFunctionStructuredErrors.get() {
        runtime_builder.errors.first().map(to_string)
      } else {
        runtime_builder.errors_deprecated.first().cloned()
      };
      if let Some(error) = serialize_error {
        return erroneous_with(error, Vec::new());
      }

      // if (FFlag::LuauTypeFunctionRobustness && !serializedTy)
      if fflag::LuauTypeFunctionRobustness.get() && serialized_ty.is_null() {
        return erroneous_with(
          "Complexity limit reached when passing a type to a type function".to_string(),
          Vec::new(),
        );
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

    if fflag::LuauTypeFunctionStructuredErrors.get() {
      // if (auto error = checkResultForError(l, name.value, lua_pcall(...)))
      //     return {..., to_string(*error), ctx->typeFunctionRuntime->messages};
      if let Some(error) = check_result_for_error(l, &name_str, pcall_result) {
        return erroneous_with(to_string(&error), (*type_function_runtime).messages.clone());
      }
    } else {
      // if (auto error = checkResultForError_DEPRECATED(l, name.value, lua_pcall(...)))
      //     return {..., std::move(error), ctx->typeFunctionRuntime->messages};
      if let Some(error) = check_result_for_error_deprecated(l, &name_str, pcall_result) {
        return erroneous_with(error, (*type_function_runtime).messages.clone());
      }
    }

    // If the return value is not a type userdata, return with error message
    // if (!isTypeUserData(l, 1))
    if !is_type_user_data(l, 1) {
      return erroneous_with(
        format(format_args!(
          "'{}' type function: returned a non-type value",
          name_str
        )),
        (*type_function_runtime).messages.clone(),
      );
    }

    // TypeFunctionTypeId retTypeFunctionTypeId = getTypeUserData(l, 1);
    let ret_type_function_type_id: TypeFunctionTypeId = get_type_user_data(l, 1);

    // structured / deprecated 两条错误通道只在「读哪份错误列表」上不同，反序列化流程一致。
    let structured_errors = fflag::LuauTypeFunctionStructuredErrors.get();
    let first_error = |builder: &TypeFunctionRuntimeBuilderState| {
      if structured_errors {
        builder.errors.first().map(to_string)
      } else {
        builder.errors_deprecated.first().cloned()
      }
    };

    // No errors should be present here since we should've returned already if any were raised during serialization.
    // LUAU_ASSERT(runtimeBuilder->errors.empty());
    LUAU_ASSERT!(first_error(&runtime_builder).is_none());

    // TypeId retTypeId = deserialize(retTypeFunctionTypeId, runtimeBuilder.get());
    let ret_type_id = deserialize_type_function_type_id_type_function_runtime_builder_state(
      ret_type_function_type_id,
      runtime_builder_ptr,
    );

    // At least 1 error occurred while deserializing
    // if (!runtimeBuilder->errors.empty())
    let result = match first_error(&runtime_builder) {
      Some(error) => erroneous_with(error, (*type_function_runtime).messages.clone()),
      None => TypeFunctionReductionResult {
        result: Some(ret_type_id),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: (*type_function_runtime).messages.clone(),
      },
    };

    // C++ `luau_temp_thread_popper` pops the temp thread in its destructor. The
    // Rust port models the destructor as an explicit method (no Drop impl),
    // so invoke it here at the single success exit, mirroring scope-end RAII.
    popper.luau_temp_thread_popper();
    result
  }
}

/// Helper: read an `AstName` into an owned `String`（空名 → ""）。
fn ast_name_to_string(name: AstName) -> String {
  name.as_str_or_empty().to_string()
}

/// cpp 的「已记 ICE，本轮直接失败」收口：`{nullopt, Erroneous, {}, {}, {}, {}}`。
fn erroneous_result() -> TypeFunctionReductionResult {
  TypeFunctionReductionResult::erroneous()
}

/// cpp 的「带错误消息失败」收口：`{nullopt, Erroneous, {}, {}, error, messages}`。
fn erroneous_with(error: String, messages: Vec<String>) -> TypeFunctionReductionResult {
  TypeFunctionReductionResult {
    result: None,
    reduction_status: Reduction::Erroneous,
    blocked_types: Vec::new(),
    blocked_packs: Vec::new(),
    error: Some(error),
    messages,
  }
}

/// cpp 的「本运行时不允许求值」收口：以 builtins 的 error 类型作结果、`MaybeOk`。
///
/// # Safety
/// `ctx_ptr` 须满足 [`error_type`] 的契约（非空且指向存活 `TypeFunctionContext`）。
unsafe fn unevaluable_result(ctx_ptr: *mut TypeFunctionContext) -> TypeFunctionReductionResult {
  TypeFunctionReductionResult {
    // Safety: 契约见本函数 `# Safety`，与 [`error_type`] 的前置一致。
    result: unsafe { Some(error_type(ctx_ptr)) },
    reduction_status: Reduction::MaybeOk,
    blocked_types: Vec::new(),
    blocked_packs: Vec::new(),
    error: None,
    messages: Vec::new(),
  }
}

/// 环境表键名 → `lua_setfield(l, -2, k)` 的 `*const c_char` 收口（本文件唯一一处，
/// alias 分支另有一处 pushcclosure+setfield 共用一次补齐）。
///
/// 保留 `with_c_str` 的理由：callee `lua_setfield` 位于 ulua-vm 的 C ABI 实现层
/// （§10 豁免层），形参仍是 `*const c_char` 并以 `strlen` 定长后 `lua_s_new` 驻留；
/// 而键名 `Name`（=`String`）缓冲既无内部 NUL 也无结尾 NUL，直传指针会读到缓冲区外
/// 字节。故只能现场补终止符——指针仅在闭包调用期内有效，callee 当场复制入 intern 表。
/// 待 ulua-vm 侧出现长度版 setfield（`impl LuaName` / `&[u8]`）后，本收口即可删除。
///
/// # Safety
/// `l` 须指向存活 `lua_State`，且栈顶是待写入的值、`-2` 处是可写表。
unsafe fn set_env_field(l: *mut lua_state::LuaState, name: &str) {
  with_c_str(name.as_bytes(), |c_name| unsafe {
    // Safety: `c_name` 由 `with_c_str` 补齐结尾 NUL 且在整个闭包调用期内存活；
    // `l`/栈槽契约由本函数调用方保证。
    lua_setfield(l, -2, c_name);
  });
}

/// Helper: `ctx->builtins->error_type`.
///
/// # Safety
/// `ctx_ptr` 须为非空且指向存活 `TypeFunctionContext` 的裸指针，且其 `builtins`
/// 字段须引用一个存活的 `BuiltinTypes`——本函数对二者连续解引用
/// （`(*(*ctx_ptr).builtins.as_ptr()).error_type`）以读取错误类型 id。
unsafe fn error_type(ctx_ptr: *mut TypeFunctionContext) -> TypeId {
  // Safety: `ctx_ptr` 的契约（见本函数 `# Safety`）保证它非空且指向存活的
  // `TypeFunctionContext`；`builtins` 字段类型为 `NonNull<BuiltinTypes>`，类型系统已排除
  // null，其指向的 BuiltinTypes 是比 ctx 长寿的会话单例。故两次解引用（取 builtins 指针、
  // 读 `error_type` 这个 `Copy` 句柄）均有效，且返回的 TypeId 在 arena 存活期内稳定。
  unsafe { (*(*ctx_ptr).builtins.as_ptr()).error_type }
}

/// `LuaCfunction` thunk for `evaluateTypeAliasCall`. The Closure is registered
/// via `LUA_PUSHCCLOSURE`, which expects a `LuaCfunction`
/// (`Option<unsafe fn(*mut LuaState) -> i32>`, Rust ABI).
unsafe extern "C-unwind" fn evaluate_type_alias_call_thunk(l: *mut lua_state::LuaState) -> i32 {
  // Safety: 本函数经 `lua_pushcclosure(l_vm, Some(evaluate_type_alias_call_thunk), name, 1)`
  // 注册为该 VM 的 Lua 闭包，Lua 调用约定保证被调用时 `l` 为当前运行线程的非空
  // `lua_State*`。`crate::type_aliases::lua_state::LuaState` 是不透明结构体，
  // 因此 `l as *mut LuaState` 是同一对象指针的视图转换（不改变地址/对齐），与被调函数
  // `evaluate_type_alias_call` 期望的形参类型完全一致；该函数自行校验 upvalue 1 里的
  // `TypeFun*` 轻用户数据，其有效性由注册处 `lua_pushlightuserdata` 写入的活指针保证。
  unsafe { evaluate_type_alias_call(l as *mut LuaState) }
}
