use alloc::{collections::BTreeMap, string::String};
use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_getfield::lua_getfield, lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l,
    lua_l_typeerror_l::lua_l_typeerror_l, lua_next::lua_next, lua_pushnil::lua_pushnil,
  },
  macros::{
    lua_isnil::lua_isnil, lua_isnoneornil::lua_isnoneornil, lua_istable::lua_istable,
    lua_pop::lua_pop,
  },
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data, get_tag::get_tag,
    get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data, optional_type_user_data::optional_type_user_data,
  },
  records::{
    type_function_property::TypeFunctionProperty,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_string_singleton::TypeFunctionStringSingleton,
    type_function_table_indexer::TypeFunctionTableIndexer,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn create_table(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count > 3 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "types.newtable: expected 0-3 arguments, but got {}",
          argument_count
        ),
      );
    }

    let mut props: BTreeMap<String, TypeFunctionProperty> = BTreeMap::new();

    if lua_istable!(vm_l, 1) {
      lua_pushnil(vm_l);
      while lua_next(vm_l, 1) != 0 {
        let key = get_type_user_data(l, -2);

        let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
        if tfst.is_null() {
          let tag = get_tag(l, key);
          lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
              "types.newtable: expected to be given a singleton type, but got {} instead",
              tag
            ),
          );
        }

        let tfsst = (*tfst).variant.get_if::<TypeFunctionStringSingleton>();
        if tfsst.is_none() {
          let tag = get_tag(l, key);
          lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
              "types.newtable: expected to be given a string singleton type, but got {} instead",
              tag
            ),
          );
        }
        let tfsst = tfsst.unwrap();

        if lua_istable!(vm_l, -1) {
          lua_getfield(vm_l, -1, c"read".as_ptr());
          let mut read_ty: Option<TypeFunctionTypeId> = None;
          if !lua_isnil!(vm_l, -1) {
            read_ty = Some(get_type_user_data(l, -1));
          }
          lua_pop(vm_l, 1);

          lua_getfield(vm_l, -1, c"write".as_ptr());
          let mut write_ty: Option<TypeFunctionTypeId> = None;
          if !lua_isnil!(vm_l, -1) {
            write_ty = Some(get_type_user_data(l, -1));
          }
          lua_pop(vm_l, 1);

          let key_name = &tfsst.value;
          props.insert(key_name.clone(), TypeFunctionProperty { read_ty, write_ty });
        } else {
          let value = get_type_user_data(l, -1);
          let key_name = &tfsst.value;
          props.insert(
            key_name.clone(),
            TypeFunctionProperty {
              read_ty: Some(value),
              write_ty: Some(value),
            },
          );
        }

        lua_pop(vm_l, 1);
      }
    } else if !lua_isnoneornil!(vm_l, 1) {
      lua_l_typeerror_l(vm_l, 1, "table");
    }

    let mut indexer: Option<TypeFunctionTableIndexer> = None;
    if lua_istable!(vm_l, 2) {
      lua_getfield(vm_l, 2, c"index".as_ptr());
      let key_type = get_type_user_data(l, -1);
      lua_pop(vm_l, 1);

      lua_getfield(vm_l, 2, c"readresult".as_ptr());
      let value_type = get_type_user_data(l, -1);
      lua_pop(vm_l, 1);

      indexer = Some(TypeFunctionTableIndexer::new(key_type, value_type));
    } else if !lua_isnoneornil!(vm_l, 2) {
      lua_l_typeerror_l(vm_l, 2, "table");
    }

    let metatable = optional_type_user_data(l, 3);
    if let Some(mt) = metatable {
      let mt_table = get_type_function_type_id::<TypeFunctionTableType>(mt);
      if mt_table.is_null() {
        let tag = get_tag(l, mt);
        lua_l_error_l(
          vm_l,
          c"%s".as_ptr(),
          core::format_args!(
            "types.newtable: expected to be given a table type as a metatable, but got {} instead",
            tag
          ),
        );
      }
    }

    alloc_type_user_data(
      l,
      TypeFunctionTypeVariant::Table(TypeFunctionTableType {
        props,
        indexer,
        metatable,
      }),
      false,
    );
    1
  }
}
