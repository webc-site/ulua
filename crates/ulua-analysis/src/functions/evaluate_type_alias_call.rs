//! Source: `Analysis/src/UserDefinedTypeFunction.cpp:99-217`
//!
//! Faithful port of `static int evaluateTypeAliasCall(lua_State* l)`. This is the
//! C Closure body registered (via `LUA_PUSHCCLOSURE`) for each parameterised
//! type alias visible to a user-defined type function: it deserializes the Lua
//! type arguments, saturates them against the alias' declared parameters,
//! instantiates the alias body, reduces any type functions inside it, then
//! serializes the result back into a (frozen) type userdata.
use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::{fflag, records::dense_hash_map::DenseHashMap};
use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_tolightuserdata::lua_tolightuserdata},
  macros::lua_upvalueindex::lua_upvalueindex,
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    deserialize_type_function_runtime_builder::deserialize_type_function_type_id_type_function_runtime_builder_state,
    follow_type::follow, get_type_function_runtime::get_type_function_runtime,
    get_type_user_data::get_type_user_data,
    reduce_type_functions_type_function::reduce_type_functions,
    saturate_arguments::saturate_arguments,
    serialize_type_function_runtime_builder::serialize_type_id_type_function_runtime_builder_state,
    to_string_error::to_string_type_error, to_string_type_function_error::to_string,
  },
  records::{
    apply_type_function::ApplyTypeFunction, arena_handle::Handle,
    freeze_type_function_types::FreezeTypeFunctionTypes, substitution::Substitution,
    txn_log::TxnLog, type_fun::TypeFun,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int evaluateTypeAliasCall(lua_State* L)`（`cpp/Analysis/src/UserDefinedTypeFunction.cpp:99`）。
pub unsafe fn evaluate_type_alias_call(l: *mut LuaState) -> i32 {
  unsafe {
    // TypeFun* tf = static_cast<TypeFun*>(lua_tolightuserdata(l, lua_upvalueindex(1)));
    let tf =
      lua_tolightuserdata(l as *mut lua_state::LuaState, lua_upvalueindex(1)) as *mut TypeFun;

    // TypeFunctionRuntime* runtime = getTypeFunctionRuntime(l);
    // TypeFunctionRuntimeBuilderState* runtimeBuilder = runtime->runtimeBuilder;
    let runtime = get_type_function_runtime(l);
    let runtime_builder: *mut TypeFunctionRuntimeBuilderState = (*runtime).runtime_builder;

    // ApplyTypeFunction applyTypeFunction{runtimeBuilder->ctx->arena};
    // builder state 的 ctx 是 Handle（类型编码非空，构造点为存活 `&mut` 借用），
    // 其 `arena` 亦为 NonNull，免裸指针往返直接物化句柄。
    let ctx = (*runtime_builder).ctx.get();
    let mut apply_type_function = ApplyTypeFunction {
      base: Substitution::substitution_new(TxnLog::empty(), Some(Handle::from_nonnull(ctx.arena))),
      encountered_forwarded_type: false,
      type_arguments: DenseHashMap::default(),
      type_pack_arguments: DenseHashMap::default(),
    };

    // int argumentCount = lua_gettop(l);
    // std::vector<TypeId> rawTypeArguments;
    let argument_count = lua_gettop(l as *mut lua_state::LuaState);
    let mut raw_type_arguments: Vec<TypeId> = Vec::new();

    // lua 栈参数从 1 起，arg_index 即上游的 i + 1
    for arg_index in 1..=argument_count {
      // TypeFunctionTypeId tfty = getTypeUserData(l, i + 1);
      let tfty: TypeFunctionTypeId = get_type_user_data(l, arg_index);
      // TypeId ty = deserialize(tfty, runtimeBuilder);
      let ty = deserialize_type_function_type_id_type_function_runtime_builder_state(
        tfty,
        runtime_builder,
      );

      // if (... ? !runtimeBuilder->errors.empty() : !runtimeBuilder->errors_DEPRECATED.empty())
      //     luaL_error(l, "failed to deserialize type at argument %d", i + 1);
      let has_errors = if fflag::LuauTypeFunctionStructuredErrors.get() {
        !(*runtime_builder).errors.is_empty()
      } else {
        !(*runtime_builder).errors_deprecated.is_empty()
      };
      if has_errors {
        ulua_vm::luaL_error!(
          l as *mut lua_state::LuaState,
          "failed to deserialize type at argument {}",
          arg_index
        );
      }

      // rawTypeArguments.push_back(ty);
      raw_type_arguments.push(ty);
    }

    // Check if we have enough arguments, by typical typechecking rules
    // size_t typesRequired = tf->typeParams.size();
    // size_t packsRequired = tf->typePackParams.size();
    let types_required = (*tf).type_params().len();
    let packs_required = (*tf).type_pack_params().len();

    // size_t typesProvided = rawTypeArguments.size() > typesRequired ? typesRequired : rawTypeArguments.size();
    let mut types_provided = if raw_type_arguments.len() > types_required {
      types_required
    } else {
      raw_type_arguments.len()
    };
    // size_t extraTypes = rawTypeArguments.size() > typesRequired ? rawTypeArguments.size() - typesRequired : 0;
    let extra_types = if raw_type_arguments.len() > types_required {
      raw_type_arguments.len() - types_required
    } else {
      0
    };
    // size_t packsProvided = 0;
    let mut packs_provided: usize = 0;

    // if (extraTypes != 0 && packsProvided == 0)
    if extra_types != 0 && packs_provided == 0 {
      // Extra types are only collected into a pack if a pack is expected
      if packs_required != 0 {
        packs_provided += 1;
      } else {
        types_provided += extra_types;
      }
    }

    let mut i = types_provided;
    while i < types_required {
      if let Some(param) = (*tf).type_params().get(i)
        && param.default_value.is_some()
      {
        types_provided += 1;
      }
      i += 1;
    }

    // for (size_t i = packsProvided; i < packsRequired; ++i)
    //     if (tf->typePackParams[i].default_value) packsProvided += 1;
    let mut i = packs_provided;
    while i < packs_required {
      if let Some(param) = (*tf).type_pack_params().get(i)
        && param.default_value.is_some()
      {
        packs_provided += 1;
      }
      i += 1;
    }

    // if (extraTypes == 0 && packsProvided + 1 == packsRequired)
    //     packsProvided += 1;
    if extra_types == 0 && packs_provided + 1 == packs_required {
      packs_provided += 1;
    }

    // if (typesProvided != typesRequired || packsProvided != packsRequired)
    //     luaL_error(l, "not enough arguments to call");
    if types_provided != types_required || packs_provided != packs_required {
      ulua_vm::luaL_error!(
        l as *mut lua_state::LuaState,
        "not enough arguments to call"
      );
    }

    // Prepare final types and packs
    // auto [types, packs] = saturateArguments(runtimeBuilder->ctx->arena, runtimeBuilder->ctx->builtins, *tf, rawTypeArguments, {});
    let arena_ref = &mut *ctx.arena.as_ptr();
    let builtins_ref = &mut *ctx.builtins.as_ptr();
    let empty_packs: Vec<TypePackId> = Vec::new();
    let (types, packs) = saturate_arguments(
      arena_ref,
      builtins_ref,
      &*tf,
      &raw_type_arguments,
      &empty_packs,
    );

    // for (size_t i = 0; i < types.size(); ++i)
    //     applyTypeFunction.type_arguments[tf->typeParams[i].ty] = types[i];
    for (param, &ty) in (*tf).type_params().iter().zip(types.iter()) {
      *apply_type_function.type_arguments.get_or_insert(param.ty) = ty;
    }

    // for (size_t i = 0; i < packs.size(); ++i)
    //     applyTypeFunction.typePackArguments[tf->typePackParams[i].tp] = packs[i];
    for (param, &tp) in (*tf).type_pack_params().iter().zip(packs.iter()) {
      *apply_type_function
        .type_pack_arguments
        .get_or_insert(param.tp) = tp;
    }

    // std::optional<TypeId> maybeInstantiated = applyTypeFunction.substitute(tf->type);
    let maybe_instantiated = apply_type_function.substitute_type_id((*tf).r#type());

    // if (!maybeInstantiated.has_value())
    // {
    //     luaL_error(l, "failed to instantiate type alias");
    //     return 1;
    // }
    let instantiated = match maybe_instantiated {
      Some(v) => v,
      None => {
        ulua_vm::luaL_error!(
          l as *mut lua_state::LuaState,
          "failed to instantiate type alias"
        );
      }
    };

    // TypeId target = follow(*maybeInstantiated);
    let target = follow(instantiated);

    // FunctionGraphReductionResult result = reduceTypeFunctions(target, Location{}, runtimeBuilder->ctx);
    let result = reduce_type_functions(
      target,
      Location::default(),
      // cpp 直接把 `runtimeBuilder->ctx`（NotNull 值）下传：Rust 侧入口签名收
      // `&mut TypeFunctionContext`，故物化 Handle 的可变视图（非空由类型编码）。
      (*runtime_builder).ctx.get_mut(),
      false,
    );

    // if (!result.errors.empty())
    //     luaL_error(l, "failed to reduce type function with: %s", to_string(result.errors.front()).c_str());
    if !result.errors.is_empty() {
      ulua_vm::luaL_error!(
        l as *mut lua_state::LuaState,
        "failed to reduce type function with: {}",
        to_string_type_error(&result.errors[0])
      );
    }

    // TypeFunctionTypeId serializedTy = serialize(follow(target), runtimeBuilder);
    let serialized_ty: TypeFunctionTypeId =
      serialize_type_id_type_function_runtime_builder_state(follow(target), runtime_builder);

    // if (!FFlag::LuauTypeFunctionRobustness)
    if !fflag::LuauTypeFunctionRobustness.get() && fflag::LuauTypeFunctionSupportsFrozen.get() {
      // FreezeTypeFunctionTypes freezer{}; freezer.run(serializedTy);
      let mut freezer = FreezeTypeFunctionTypes::new();
      freezer.base.run_type_function_type_id(serialized_ty);
    }

    // if (FFlag::LuauTypeFunctionStructuredErrors)
    if fflag::LuauTypeFunctionStructuredErrors.get() {
      // if (!runtimeBuilder->errors.empty())
      //     luaL_error(l, "%s", to_string(runtimeBuilder->errors.front()).c_str());
      let errors = &(*runtime_builder).errors;
      if !errors.is_empty() {
        ulua_vm::luaL_error!(l as *mut lua_state::LuaState, "{}", to_string(&errors[0]));
      }
    } else {
      // if (!runtimeBuilder->errors_DEPRECATED.empty())
      //     luaL_error(l, "%s", runtimeBuilder->errors_DEPRECATED.front().c_str());
      let errors_deprecated = &(*runtime_builder).errors_deprecated;
      if !errors_deprecated.is_empty() {
        ulua_vm::luaL_error!(l as *mut lua_state::LuaState, "{}", errors_deprecated[0]);
      }
    }

    // if (FFlag::LuauTypeFunctionRobustness)
    if fflag::LuauTypeFunctionRobustness.get() {
      // if (!serializedTy) luaL_error(l, "Complexity limit reached when passing a type to a type alias");
      if serialized_ty.is_null() {
        ulua_vm::luaL_error!(
          l as *mut lua_state::LuaState,
          "Complexity limit reached when passing a type to a type alias"
        );
      }

      if fflag::LuauTypeFunctionSupportsFrozen.get() {
        // FreezeTypeFunctionTypes freezer{}; freezer.run(serializedTy);
        let mut freezer = FreezeTypeFunctionTypes::new();
        freezer.base.run_type_function_type_id(serialized_ty);
      }
    }

    // allocTypeUserData(l, serializedTy->type, /* frozen */ true);
    let type_variant = (*serialized_ty).type_variant.clone();
    alloc_type_user_data(l, type_variant, true);

    // return 1;
    1
  }
}
