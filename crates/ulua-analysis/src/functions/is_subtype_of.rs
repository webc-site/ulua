/// 对应 C++ 原生 `static int isSubtypeOf(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1835`）。
use ulua_vm::macros::lua_l_error::luaL_error;
use ulua_vm::records::lua_state::LuaState;

use crate::functions::{
  deserialize_type_function_runtime_builder::deserialize_type_function_type_id_type_function_runtime_builder_state,
  get_type_function_runtime::get_type_function_runtime, get_type_user_data::get_type_user_data,
};
pub(crate) fn is_subtype_of(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 按类型函数运行时约定传入并全程存活（经 `c_thunk!` 蹦床重建为独占 `&mut`）。`get_type_function_runtime(l)` 返回该 L 建栈时接线、非空且比本次调用长寿的 runtime，
  // 其 `runtime_builder` 是构造期布线的非空指针，`&mut *` 重建 builder 的可变借用，两次
  // deserialize 在其上写 errors；此后才经 `ctx.get()` 物化 `Handle<TypeFunctionContext>` 的共享借用，
  // 与 deserialize 的可写窗口串行不交叠。`ctx.subtyping`/`ctx.scope` 为构造期接线的 NonNull，非空且
  // 比 ctx 长寿，故 `(*ctx.subtyping.as_ptr()).is_subtype_..` 与入参 `ctx.scope.as_ptr()` 合法。
  // 错误路径 `luaL_error!` 抛错（longjmp）不返回。
  unsafe {
    let argument_count = l.get_top();
    if argument_count != 2 {
      luaL_error!(
        l.as_mut_ptr(),
        "type.issubtypeof: expected 2 arguments, but got {}",
        argument_count
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let arg = get_type_user_data(l, 2);

    let runtime = get_type_function_runtime(l).expect("runtime 于注册阶段挂载，会话内恒非空");
    let runtime_builder = &mut *(runtime.get_mut().runtime_builder);

    let sub_ty = deserialize_type_function_type_id_type_function_runtime_builder_state(
      self_ty,
      runtime_builder,
    );
    if !runtime_builder.errors.is_empty() || !runtime_builder.errors_deprecated.is_empty() {
      luaL_error!(l.as_mut_ptr(), "failed to deserialize the self type");
    }

    let super_ty =
      deserialize_type_function_type_id_type_function_runtime_builder_state(arg, runtime_builder);
    if !runtime_builder.errors.is_empty() || !runtime_builder.errors_deprecated.is_empty() {
      luaL_error!(l.as_mut_ptr(), "failed to deserialize the argument type");
    }

    // ctx 只在 deserialize 会话之后读取：借用延到最后使用处，不再与上面两次
    // `runtime_builder` 可变借用交叠。
    let ctx = runtime_builder.ctx.get();
    let result = (*ctx.subtyping.as_ptr()).is_subtype_type_id_type_id_not_null_scope(
      sub_ty,
      super_ty,
      // Safety: ctx.scope 为构造期接线的 NonNull<Scope>，非空且比 ctx 长寿，只读借用。
      &*ctx.scope.as_ptr(),
    );
    l.push_boolean(result.is_subtype);
    1
  }
}
