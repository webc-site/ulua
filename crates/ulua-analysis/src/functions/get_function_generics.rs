use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::{lua_createtable::lua_createtable, lua_rawseti::lua_rawseti},
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_tag::get_tag,
    get_type_function_runtime::{get_type_function_type_id, get_type_function_type_pack_id},
    get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_tag,
  records::{
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
  },
  type_aliases::{lua_state::LuaState, type_function_type_variant::TypeFunctionTypeVariant},
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int getFunctionGenerics(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1464`）。
pub unsafe fn get_function_generics(l: *mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 按类型函数运行时约定传入并全程存活；`l as *mut lua_state::LuaState`
  // 是同一状态的重解释。`self_ty` 由 `get_type_user_data` 取得，`get_type_function_type_id::<
  // TypeFunctionFunctionType>` 按 RTTI class-index 下转；`tfft.is_null()` 分支内 `throw_type_error`
  // 返回 `!`（抛 Lua 错误不返回），故后续 `(*tfft).generics`/`(*el)`(arena 存活 TypeId) 解引用
  // 合法。generic_packs 元素按构造均为 TypeFunctionGenericTypePack，其 class-index 下转 gty 非空。
  // 单线程串行执行，无并发别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let self_ty = get_type_user_data(l, 1);

    let tfft = get_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
    lua_check_tag!(
      vm_l,
      tfft.is_null(),
      l,
      self_ty,
      "type.generics: expected self to be a function, but got {} instead"
    );

    lua_createtable(
      vm_l,
      ((*tfft).generics.len() + (*tfft).generic_packs.len()) as i32,
      0,
    );

    let mut pos: i32 = 1;

    for el in &(*tfft).generics {
      alloc_type_user_data(l, (*(*el)).type_variant.clone(), false);
      lua_rawseti(vm_l, -2, pos);
      pos += 1;
    }

    for el in &(*tfft).generic_packs {
      let gty = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(*el);
      LUAU_ASSERT!(!gty.is_null());
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
          is_named: (*gty).is_named,
          is_pack: true,
          name: (*gty).name.clone(),
        }),
        false,
      );
      lua_rawseti(vm_l, -2, pos);
      pos += 1;
    }

    1
  }
}
