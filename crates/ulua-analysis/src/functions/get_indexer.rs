use ulua_vm::{
  functions::{
    lua_createtable::lua_createtable, lua_gettop::lua_gettop, lua_pushnil::lua_pushnil,
    lua_setfield::lua_setfield,
  },
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
    lua_names::{FIELD_INDEX, FIELD_READ_RESULT, FIELD_WRITE_RESULT},
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  records::{
    type_function_extern_type::TypeFunctionExternType,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int getIndexer(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1644`）。
pub unsafe fn get_indexer(l: *mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活，`l as *mut lua_state::LuaState` 为同址
  // 重解释。`tftt`/`tfct` 按 class-index 下转，仅在 `!is_null()` 守卫后解引用；indexer 字段先经
  // `is_none()` 判定再 `.as_ref().unwrap()`（此时已确认 Some，unwrap 不会 panic），其 key_type/
  // value_type 是 arena 存活 TypeId（地址不移动）。各 `throw_type_error` 分支返回 `!` 不返回。
  // 单线程串行执行，无并发别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, != 1, "type.indexer: expected 1 arguments, but got {}");

    let self_ty = get_type_user_data(l, 1);

    let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if !tftt.is_null() {
      if (*tftt).indexer.is_none() {
        lua_pushnil(vm_l);
      } else {
        lua_createtable(vm_l, 0, 3);

        // Safety: else 支由同判据 is_none()==false 进入，indexer 必为 Some。
        let indexer = (*tftt).indexer.as_ref().expect(
          "else 支由同判据 is_none 为假进入，indexer 必为 Some（cpp 同位 is_none 后直 deref）",
        );
        alloc_type_user_data(l, (*indexer.key_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_INDEX.as_ptr().cast());
        alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_READ_RESULT.as_ptr().cast());
        alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_WRITE_RESULT.as_ptr().cast());
      }

      return 1;
    }

    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
    if !tfct.is_null() {
      if (*tfct).indexer.is_none() {
        lua_pushnil(vm_l);
      } else {
        lua_createtable(vm_l, 0, 3);

        // Safety: else 支由同判据 is_none()==false 进入，indexer 必为 Some。
        let indexer = (*tfct).indexer.as_ref().expect(
          "else 支由同判据 is_none 为假进入，indexer 必为 Some（cpp 同位 is_none 后直 deref）",
        );
        alloc_type_user_data(l, (*indexer.key_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_INDEX.as_ptr().cast());
        alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_READ_RESULT.as_ptr().cast());
        alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_WRITE_RESULT.as_ptr().cast());
      }

      return 1;
    }

    throw_type_error(
      vm_l,
      format_args!(
        "type.indexer: self to be either a table or class, but got {} instead",
        get_tag(l, self_ty)
      ),
    );
  }
}
