//! Faithful port of
//! `static std::tuple<std::vector<TypeFunctionTypeId>, std::vector<TypeFunctionTypePackId>>
//!  getGenerics(lua_State* l, int idx, const char* fname)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1125-1177).
use alloc::vec::Vec;

use ulua_vm::{
  functions::{
    lua_gettable::lua_gettable, lua_l_typeerror_l::lua_l_typeerror_l, lua_objlen::lua_objlen,
    lua_pushinteger::lua_pushinteger, lua_pushvalue::lua_pushvalue,
  },
  macros::{
    lua_isnil::lua_isnil, lua_isnoneornil::lua_isnoneornil, lua_istable::lua_istable,
    lua_pop::lua_pop,
  },
  records::lua_state,
};

use crate::{
  functions::{
    allocate_type_function_type_pack::allocate_type_function_type_pack,
    get_type_function_runtime::{get_type_function_runtime, get_type_function_type_id},
    get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  records::{
    arena_handle::Handle, type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant,
  },
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_generics(
  l: *mut LuaState,
  idx: i32,
  fname: &str,
) -> (Vec<TypeFunctionTypeId>, Vec<TypeFunctionTypePackId>) {
  // Safety: `l` 由 Lua VM 按类型函数运行时约定传入并全程存活，`l as *mut lua_state::LuaState`
  // 为同一地址的重解释。`get_type_user_data`/`get_type_function_type_id::<TypeFunctionGenericType>`
  // 的下转按 class-index 判定，`gty` 仅在 `!is_null()` 守卫后才解引用读取 is_pack/is_named/name；
  // 类型函数数据存活于本次调用。错误分支 `throw_type_error` 返回 `!` 不返回，`lua_l_typeerror_l`
  // 之后不再解引用任何指针。单线程串行遍历，push/gettable/pop 栈操作平衡、无并发别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    // 注册期写入主线程 thread data 的非空 runtime（null 由 Handle::from_ptr 收敛为 panic）。
    let runtime = Handle::from_ptr(get_type_function_runtime(l));

    let mut types: Vec<TypeFunctionTypeId> = Vec::new();
    let mut packs: Vec<TypeFunctionTypePackId> = Vec::new();

    if lua_istable!(vm_l, idx) {
      lua_pushvalue(vm_l, idx);

      let mut i: i32 = 1;
      while i <= lua_objlen(vm_l, -1) {
        lua_pushinteger(vm_l, i);
        lua_gettable(vm_l, -2);

        if lua_isnil!(vm_l, -1) {
          lua_pop(vm_l, 1);
          break;
        }

        // TypeFunctionTypeId ty = getTypeUserData(l, -1);
        let ty = get_type_user_data(l, -1);

        // if (auto gty = get<TypeFunctionGenericType>(ty))
        let gty = get_type_function_type_id::<TypeFunctionGenericType>(ty);
        if !gty.is_null() {
          if (*gty).is_pack {
            packs.push(allocate_type_function_type_pack(
              runtime,
              TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
                is_named: (*gty).is_named,
                name: (*gty).name.clone(),
              }),
            ));
          } else {
            if !packs.is_empty() {
              throw_type_error(
                vm_l,
                format_args!("{}: generic type cannot follow a generic pack", fname),
              );
            }

            types.push(ty);
          }
        } else {
          throw_type_error(
            vm_l,
            format_args!("{}: table member was not a generic type", fname),
          );
        }

        lua_pop(vm_l, 1);
        i += 1;
      }

      lua_pop(vm_l, 1);
    } else if !lua_isnoneornil!(vm_l, idx) {
      lua_l_typeerror_l(vm_l, idx, "table");
    }

    (types, packs)
  }
}
