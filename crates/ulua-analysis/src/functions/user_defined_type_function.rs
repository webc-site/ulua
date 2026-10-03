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

use alloc::{boxed::Box, string::{String}, vec::Vec};
use ulua_ast::records::ast_name::AstName;
use ulua_common::{fflag, functions::{c_str::with_c_str, format::format, get_clock::get_clock}, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};
use ulua_vm::{functions::{lua_callbacks::lua_callbacks, lua_getfenv::lua_getfenv, lua_gettable::lua_gettable, lua_getthreaddata::lua_getthreaddata, lua_mainthread::lua_mainthread, lua_newthread::lua_newthread}, macros::lua_registryindex::LUA_REGISTRYINDEX, records::lua_state};
use crate::{enums::reduction::Reduction, functions::{alloc_type_user_data::alloc_type_user_data, check_result_for_error::check_result_for_error, check_result_for_error_deprecated::check_result_for_error_deprecated, deserialize_type_function_runtime_builder::deserialize_type_function_type_id_type_function_runtime_builder_state, evaluate_type_alias_call::evaluate_type_alias_call, follow_type::follow, get_mutable_type::get_mutable, get_type_user_data::get_type_user_data, is_pending::is_pending, is_type_user_data::is_type_user_data, reset_type_function_state::reset_type_function_state, serialize_type_function_runtime_builder::serialize_type_id_type_function_runtime_builder_state, to_string_type_function_error::to_string}, records::{arena_handle::{alias, alias_nn, alias_ref}, extern_type::ExternType, find_user_type_function_blockers::FindUserTypeFunctionBlockers, freeze_type_function_types::FreezeTypeFunctionTypes, generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait}, luau_temp_thread_popper::LuauTempThreadPopper, scoped_assign::ScopedAssign, time_limit_error::TimeLimitError, type_function_context::TypeFunctionContext, type_function_instance_type::TypeFunctionInstanceType, type_function_reduction_result::TypeFunctionReductionResult, type_function_runtime::TypeFunctionRuntime, type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState, user_cancel_error::UserCancelError, visit_key::VisitKey}, type_aliases::{type_function_type_id::{AsTypeFunctionType, TypeFunctionTypeId}, type_id::TypeId, type_pack_id::TypePackId}};
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

/// # Safety
/// 本回调只由 `user_defined_type_function` 经 `lua_callbacks` 安装、且只在同一
/// 次 `lua_pcall` 期间生效；调用方按 Lua C ABI 契约以「当前运行线程 + gc 阶段
/// 码」传入参数，`l` 必为非空存活 `lua_State*`。
unsafe extern "C-unwind" fn user_defined_type_function_interrupt(
  l: *mut lua_state::LuaState,
  _gc: i32,
) {
  // Safety: `lua_mainthread(l)` 对任意协程都返回其主线程（永不为空），而该主线程
  // 的 thread data 在 `TypeFunctionRuntime::prepare_state` 里无条件写入本 runtime
  // 地址，故 `lua_getthreaddata` 返回非空且指向活的 runtime——runtime 拥有该 VM
  //（`state` 字段），必然比在其上执行的回调长寿。随后只读 `limits`/`ice` 两个
  // 字段做超时与取消判定，不写任何共享状态；panic 只用于按 C++ 抛
  // TimeLimitError/UserCancelError 的语义展开出 VM。
  let main = lua_mainthread(alias_ref(l));
  let data = lua_getthreaddata(alias_ref(main));
  let ctx = data.cast::<TypeFunctionRuntime>();

  if let Some(finish_time) = alias_ref(ctx).limits.finish_time
    && get_clock() > finish_time
  {
    panic!(
      "{}",
      TimeLimitError::time_limit_error_time_limit_error(&alias_ref(ctx).ice.module_name)
    );
  }

  if let Some(token) = &alias_ref(ctx).limits.cancellation_token
    && token.requested()
  {
    panic!(
      "{}",
      UserCancelError::new(alias_ref(ctx).ice.module_name.clone())
    );
  }
}

pub fn user_defined_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  _pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // §2 收口形态：原「整壳 unsafe」已拆除——业务逻辑一律持普通引用；裸指针仅
  // 作注册表/lightuserdata 身份键值（§2(b) 既有子系统约定，见
  // `records/user_defined_function_data.rs` 字段注），解引用全部经
  // `arena_handle` 的 `alias`/`alias_ref`/`alias_nn` chokepoint 物化；余下每处
  // `unsafe {}` 紧贴一个 ulua-vm `pub unsafe fn` 或本 crate 带 `# Safety` 的
  // VM 会话函数，前提逐条对应其文档。
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
    ctx
      .ice()
      .ice_string("user-defined type function module has expired");
    return erroneous_result();
  }

  let definition_ptr = type_function.user_func_data.definition;

  // if (!typeFunction->userFuncName || !typeFunction->userFuncData.definition)
  if type_function.user_func_name.is_none() || definition_ptr.is_null() {
    ctx
      .ice()
      .ice_string("all user-defined type functions must have an associated function definition");
    return erroneous_result();
  }

  // §2：runtime 经 `alias_nn` chokepoint 物化为独占借用（NonNull 句柄由构造期
  // 接线保证存活，契约见 `records/arena_handle.rs` 模块头），下游不再逐处裸解引用。
  let runtime = alias_nn(ctx.type_function_runtime);

  // If type functions cannot be evaluated because of errors in the code, we do not generate any additional ones
  // if (!ctx->typeFunctionRuntime->allowEvaluation || typeFunction->userFuncData.definition->hasErrors)
  if !runtime.allow_evaluation || alias_ref(definition_ptr).has_errors {
    return unevaluable_result(ctx);
  }

  // FindUserTypeFunctionBlockers check{ctx};
  // cpp 的 `NotNull<TypeFunctionContext>` 实参即本帧的会话借用：访问者只在
  // 紧随其后的阻塞检查段内存活，故直接下传 `&mut` 借用（非空与存活性由类型
  // 保证）。
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
    let alias_def = alias_ref(def_ptr);
    if alias_def.type_params().is_empty() && alias_def.type_pack_params().is_empty() {
      check.traverse_type_id(follow(alias_def.r#type()));
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
    if alias_ref(def_ptr).has_errors {
      return unevaluable_result(ctx);
    }

    // bool registrationFailed = ... registerFunction(definition.first).has_value()
    // `register_function` 为引用签名：arena 节点句柄经 `alias` 物化借用。
    let registration_failed = if fflag::LuauTypeFunctionStructuredErrors.get() {
      runtime.register_function(alias(def_ptr)).is_some()
    } else {
      runtime
        .register_function_deprecated(alias(def_ptr))
        .is_some()
    };
    if registration_failed {
      // Failure to register at this point means that original definition had to error out and should not
      // have been present in the environment
      ctx
        .ice()
        .ice_string("user-defined type function reference cannot be registered");
      return erroneous_result();
    }
  }

  // AstName name = typeFunction->userFuncData.definition->name;
  let name = alias_ref(definition_ptr).name;
  let name_str = ast_name_to_string(name);

  // lua_State* global = ctx->typeFunctionRuntime->state.get();
  let global = runtime.state.0;

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
  // SAFETY: VM 边界——`global` 为 runtime 懒建并已判非空的主线程存活 state，单
  // 线程串行驱动；新线程由其拥有、活至 popper 弹出（与原 cpp 时序逐字一致）。
  let l_thread = unsafe { lua_newthread(global.cast::<lua_state::LuaState>()) };
  // 对 VM 的栈操作一律经 `l_vm` 这一独占借用走安全方法/安全入口；需要裸句柄的
  // C-ABI 入口（如 `lua_callbacks`）直接取同一地址的 `l_thread`。
  let l_vm = alias(l_thread);
  let mut popper = LuauTempThreadPopper::new(global);

  // std::unique_ptr<TypeFunctionRuntimeBuilderState> runtimeBuilder = std::make_unique<...>(ctx);
  // builder state 的 ctx 由本帧的会话借用物化（句柄非空由类型编码，无 null 分支）。
  let mut runtime_builder: Box<TypeFunctionRuntimeBuilderState> =
    Box::new(TypeFunctionRuntimeBuilderState::new(ctx));
  let runtime_builder_ptr = runtime_builder.as_mut();

  // ScopedAssign setRuntimeBuilder(ctx->typeFunctionRuntime->runtimeBuilder, runtimeBuilder.get());
  let _set_runtime_builder = ScopedAssign::new(&mut runtime.runtime_builder, runtime_builder_ptr);
  // ScopedAssign enableReduction(ctx->normalizer->sharedState->reentrantTypeReduction, false);
  let shared_state = ctx.normalizer_mut().shared_state_ptr();
  let _enable_reduction =
    ScopedAssign::new(&mut alias(shared_state).reentrant_type_reduction, false);

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
    if runtime.initialized.find(&curr_ptr).is_some() {
      continue;
    }
    // ctx->typeFunctionRuntime->initialized.insert(curr.first);
    runtime.initialized.insert(curr_ptr);

    // LUA_PUSHLIGHTUSERDATA(l, curr.first);
    // lua_gettable(l, LUA_REGISTRYINDEX);
    // SAFETY: VM 边界——`l_vm` 为本帧独占存活线程（popper 已记其父）；仅取节点
    // 地址值作身份键，被调方按契约不解引用。
    unsafe { l_vm.push_lightuserdata(curr_ptr.cast()) };
    lua_gettable(l_vm, LUA_REGISTRYINDEX);

    // if (!lua_isfunction(l, -1))
    if !l_vm.is_function(-1) {
      ctx
        .ice()
        .ice_string("user-defined type function reference cannot be found in the registry");
      return erroneous_result();
    }

    // Build up the environment of the current function, where some might not be visible
    // lua_getfenv(l, -1);
    // lua_setreadonly(l, -1, false);
    lua_getfenv(l_vm, -1);
    l_vm.set_readonly(-1, false);

    // for (auto& [name, definition] : typeFunction->userFuncData.environmentFunction)
    // 键名统一走 `set_field_str`（字节切片直传，NUL 补写收口在 ulua-vm 一处）。
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
        // SAFETY: 同上——身份键按地址值透传。
        unsafe { l_vm.push_lightuserdata(def_ptr.cast()) };
        lua_gettable(l_vm, LUA_REGISTRYINDEX);

        // if (!lua_isfunction(l, -1)) break;
        if !l_vm.is_function(-1) {
          break; // Don't have to report an error here, we will visit each function in outer loop
        }

        // lua_setfield(l, -2, name.c_str());
        l_vm.set_field_str(-2, name);
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
        let alias_def = alias_ref(def_ptr);
        // if (definition.first->typeParams.empty() && definition.first->typePackParams.empty())
        if alias_def.type_params().is_empty() && alias_def.type_pack_params().is_empty() {
          // TypeId ty = follow(definition.first->type);
          let ty = follow(alias_def.r#type());

          // This is checked at the top of the function, and should still be true.
          // LUAU_ASSERT(!isPending(ty, ctx->solver));
          LUAU_ASSERT!(!is_pending(ty, ctx.solver));

          // TypeFunctionTypeId serializedTy = serialize(ty, runtimeBuilder.get());
          let serialized_ty: TypeFunctionTypeId =
            serialize_type_id_type_function_runtime_builder_state(ty, &mut runtime_builder);

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
              // arena 节点读取经 `AsTypeFunctionType::as_type` 门面收口（块地址
              // 稳定，见 `type_aliases/type_function_type_id.rs`）。
              let variant = serialized_ty.as_type().type_variant.clone();
              // SAFETY: VM 边界——`l` 为本帧独占存活线程；被调函数 `# Safety`
              // 契约（见 alloc_type_user_data.rs）的会话前提此刻逐项成立。
              unsafe { alloc_type_user_data(l_vm, variant, true) };
              // lua_setfield(l, -2, name.c_str());
              l_vm.set_field_str(-2, name);
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
              let variant = serialized_ty.as_type().type_variant.clone();
              // SAFETY: VM 边界——同上。
              unsafe { alloc_type_user_data(l_vm, variant, true) };
              l_vm.set_field_str(-2, name);
            }
          }
        } else {
          // LUA_PUSHLIGHTUSERDATA(l, definition.first);
          // LUA_PUSHCCLOSURE(l, evaluateTypeAliasCall, name.c_str(), 1);
          // lua_setfield(l, -2, name.c_str());
          // 一次补 NUL 同时喂 closure 的 debugname 与 setfield 的键：两个 callee
          // 都在调用期内把串 `lua_s_new` 驻留进 intern 表，指针不外存。
          // SAFETY: 同上——身份键按地址值透传。
          unsafe { l_vm.push_lightuserdata(def_ptr.cast()) };
          with_c_str(name.as_bytes(), |c_name| {
            // SAFETY: VM 边界——`c_name` 由 `with_c_str` 保证调用期内 NUL 结尾
            // 有效；thunk 契约见其 `# Safety`；nup=1 与上方刚压入的
            // lightuserdata 配对。
            unsafe { l_vm.push_c_closure(Some(evaluate_type_alias_call_thunk), c_name, 1) }
          });
          l_vm.set_field_str(-2, name);
        }
      }
    }

    // lua_setreadonly(l, -1, true);
    // lua_pop(l, 2);
    l_vm.set_readonly(-1, true);
    l_vm.pop(2);
  }

  // Fetch the function we want to evaluate
  // LUA_PUSHLIGHTUSERDATA(l, typeFunction->userFuncData.definition);
  // lua_gettable(l, LUA_REGISTRYINDEX);
  // SAFETY: 同上——身份键按地址值透传。
  unsafe { l_vm.push_lightuserdata(definition_ptr.cast()) };
  lua_gettable(l_vm, LUA_REGISTRYINDEX);

  // if (!lua_isfunction(l, -1))
  if !l_vm.is_function(-1) {
    ctx
      .ice()
      .ice_string("user-defined type function reference cannot be found in the registry");
    return erroneous_result();
  }

  // resetTypeFunctionState(l);
  reset_type_function_state(l_vm);

  // Push serialized arguments onto the stack
  // for (auto typeParam : typeParams)
  for &type_param in type_params {
    // TypeId ty = follow(typeParam);
    let ty = follow(type_param);
    // LUAU_ASSERT(!isPending(ty, ctx->solver));
    LUAU_ASSERT!(!is_pending(ty, ctx.solver));

    // TypeFunctionTypeId serializedTy = serialize(ty, runtimeBuilder.get());
    let serialized_ty: TypeFunctionTypeId =
      serialize_type_id_type_function_runtime_builder_state(ty, &mut runtime_builder);

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
    let variant = serialized_ty.as_type().type_variant.clone();
    // SAFETY: VM 边界——同上（本帧独占存活线程）。
    unsafe { alloc_type_user_data(l_vm, variant, false) };
  }

  // Set up an interrupt handler for type functions to respect type checking limits and LSP cancellation requests.
  // lua_callbacks(l)->interrupt = [](lua_State* l, int gc) { ... };
  // SAFETY: VM 边界——`l_thread` 为本帧独占存活线程，`lua_callbacks` 返回其常驻
  // 回调表指针（契约见其函数文档）；此处只写 interrupt 槽一次，回调生效窗口即
  // 下方 `pcall`，thunk 的 `# Safety` 前提在窗口内逐项成立。
  unsafe { alias(lua_callbacks(l_thread)) }.interrupt = Some(user_defined_type_function_interrupt);

  // ctx->typeFunctionRuntime->messages.clear();
  runtime.messages.clear();

  // lua_pcall(l, int(typeParams.size()), 1, 0)
  let pcall_result = l_vm.pcall(type_params.len() as i32, 1, 0);

  if fflag::LuauTypeFunctionStructuredErrors.get() {
    // if (auto error = checkResultForError(l, name.value, lua_pcall(...)))
    //     return {..., to_string(*error), ctx->typeFunctionRuntime->messages};
    if let Some(error) = check_result_for_error(l_vm, &name_str, pcall_result) {
      return erroneous_with(to_string(&error), runtime.messages.clone());
    }
  } else {
    // if (auto error = checkResultForError_DEPRECATED(l, name.value, lua_pcall(...)))
    //     return {..., std::move(error), ctx->typeFunctionRuntime->messages};
    if let Some(error) = check_result_for_error_deprecated(l_vm, &name_str, pcall_result) {
      return erroneous_with(error, runtime.messages.clone());
    }
  }

  // If the return value is not a type userdata, return with error message
  // if (!isTypeUserData(l, 1))
  if !is_type_user_data(l_vm, 1) {
    return erroneous_with(
      format(format_args!(
        "'{}' type function: returned a non-type value",
        name_str
      )),
      runtime.messages.clone(),
    );
  }

  // TypeFunctionTypeId retTypeFunctionTypeId = getTypeUserData(l, 1);
  let ret_type_function_type_id: TypeFunctionTypeId = get_type_user_data(l_vm, 1);

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
    &mut runtime_builder,
  );

  // At least 1 error occurred while deserializing
  // if (!runtimeBuilder->errors.empty())
  let result = match first_error(&runtime_builder) {
    Some(error) => erroneous_with(error, runtime.messages.clone()),
    None => TypeFunctionReductionResult {
      result: Some(ret_type_id),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: runtime.messages.clone(),
    },
  };

  // C++ `luau_temp_thread_popper` pops the temp thread in its destructor. The
  // Rust port models the destructor as an explicit method (no Drop impl),
  // so invoke it here at the single success exit, mirroring scope-end RAII.
  popper.luau_temp_thread_popper();
  result
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
/// builtins 读取经 `TypeFunctionContext::builtins` chokepoint 门面，全程安全。
fn unevaluable_result(ctx: &TypeFunctionContext) -> TypeFunctionReductionResult {
  TypeFunctionReductionResult {
    result: Some(ctx.builtins().error_type),
    reduction_status: Reduction::MaybeOk,
    blocked_types: Vec::new(),
    blocked_packs: Vec::new(),
    error: None,
    messages: Vec::new(),
  }
}

/// `LuaCfunction` thunk for `evaluateTypeAliasCall`. The Closure is registered
/// via `LUA_PUSHCCLOSURE`, which expects a `LuaCfunction`
/// (`Option<unsafe fn(*mut LuaState) -> i32>`, Rust ABI).
///
/// # Safety
/// 本函数经 `push_c_closure` 注册为该 VM 的 Lua 闭包；Lua 调用约定保证被调用
/// 时 `l` 为当前运行线程的非空存活 `lua_State*`。
unsafe extern "C-unwind" fn evaluate_type_alias_call_thunk(l: *mut lua_state::LuaState) -> i32 {
  // Safety: 本蹦床是 C-ABI 边界，形参形状由 `lua_CFunction` 约定固定；`l` 由 VM 在
  // 本次闭包调用帧上给出，非空且指向本次调用独占的存活线程，故可重建为 `&mut`，
  // 借用窗严格止于 `evaluate_type_alias_call` 返回。该函数只读 upvalue 1 里的
  // `TypeFun*` 轻用户数据，其有效性由注册处 `lua_pushlightuserdata` 写入的活指针保证。
  unsafe { evaluate_type_alias_call(&mut *l) }
}
