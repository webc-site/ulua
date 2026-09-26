use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_pushboolean::lua_pushboolean},
  macros::lua_l_error::luaL_error,
  records::lua_state,
};

use crate::{
  functions::{
    deserialize_type_function_runtime_builder::deserialize_type_function_type_id_type_function_runtime_builder_state,
    get_type_function_runtime::get_type_function_runtime, get_type_user_data::get_type_user_data,
  },
  records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int isSubtypeOf(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1835`）。
pub unsafe fn is_subtype_of(l: *mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 按类型函数运行时约定传入并全程存活，`l as *mut lua_state::LuaState` 为
  // 同址重解释。`get_type_function_runtime(l)` 返回该 L 建栈时接线、非空且比本次调用长寿的 runtime，
  // 其 `runtime_builder` 是构造期布线的非空指针，`&mut *` 重建 builder 的可变借用，其
  // `ctx` 是 `Handle<TypeFunctionContext>`（构造点为存活的 `&mut` 会话借用，类型编码非空），
  // 经 `get()` 物化共享借用——deserialize 写 builder.errors，读取的 `ctx.subtyping`/`ctx.scope`
  // 是不同对象，单线程串行下无别名重叠。`ctx.subtyping`/`ctx.scope` 为构造期接线的 NonNull，非空且
  // 比 ctx 长寿，故 `(*ctx.subtyping.as_ptr()).is_subtype_..` 与入参 `ctx.scope.as_ptr()` 合法。
  // 错误路径 `luaL_error!` 抛错（longjmp）不返回。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
      luaL_error!(
        vm_l,
        "type.issubtypeof: expected 2 arguments, but got {}",
        argument_count
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let arg = get_type_user_data(l, 2);

    let runtime = get_type_function_runtime(l);
    let runtime_builder = &mut *(*runtime).runtime_builder;
    let ctx = runtime_builder.ctx.get();

    let sub_ty = deserialize_type_function_type_id_type_function_runtime_builder_state(
      self_ty,
      runtime_builder as *mut TypeFunctionRuntimeBuilderState,
    );
    if !runtime_builder.errors.is_empty() || !runtime_builder.errors_deprecated.is_empty() {
      luaL_error!(vm_l, "failed to deserialize the self type");
    }

    let super_ty = deserialize_type_function_type_id_type_function_runtime_builder_state(
      arg,
      runtime_builder as *mut TypeFunctionRuntimeBuilderState,
    );
    if !runtime_builder.errors.is_empty() || !runtime_builder.errors_deprecated.is_empty() {
      luaL_error!(vm_l, "failed to deserialize the argument type");
    }

    let result = (*ctx.subtyping.as_ptr()).is_subtype_type_id_type_id_not_null_scope(
      sub_ty,
      super_ty,
      ctx.scope.as_ptr(),
    );
    lua_pushboolean(vm_l, result.is_subtype as i32);
    1
  }
}
