use alloc::vec::Vec;
use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_getfield::lua_getfield, lua_gettable::lua_gettable, lua_gettop::lua_gettop,
    lua_l_error_l::lua_l_error_l, lua_l_typeerror_l::lua_l_typeerror_l, lua_objlen::lua_objlen,
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
    alloc_type_user_data::alloc_type_user_data,
    allocate_type_function_type_pack::allocate_type_function_type_pack, get_generics::get_generics,
    get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data, optional_type_user_data::optional_type_user_data,
  },
  records::{
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn create_function(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count > 3 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "types.newfunction: expected 0-3 arguments, but got {}",
          argument_count
        ),
      );
    }

    let arg_types: TypeFunctionTypePackId;

    if lua_istable!(vm_l, 1) {
      lua_getfield(vm_l, 1, c"head".as_ptr());
      lua_getfield(vm_l, 1, c"tail".as_ptr());

      arg_types = get_type_pack_runtime(l, -2, -1);

      lua_pop(vm_l, 2);
    } else if !lua_isnoneornil!(vm_l, 1) {
      lua_l_typeerror_l(vm_l, 1, "table");
    } else {
      arg_types = allocate_type_function_type_pack(
        l,
        TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
          head: Vec::new(),
          tail: None,
        }),
      );
    }

    let ret_types: TypeFunctionTypePackId;

    if lua_istable!(vm_l, 2) {
      lua_getfield(vm_l, 2, c"head".as_ptr());
      lua_getfield(vm_l, 2, c"tail".as_ptr());

      ret_types = get_type_pack_runtime(l, -2, -1);

      lua_pop(vm_l, 2);
    } else if !lua_isnoneornil!(vm_l, 2) {
      lua_l_typeerror_l(vm_l, 2, "table");
    } else {
      ret_types = allocate_type_function_type_pack(
        l,
        TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
          head: Vec::new(),
          tail: None,
        }),
      );
    }

    let (generic_types, generic_packs) = get_generics(l, 3, "types.newfunction");

    alloc_type_user_data(
      l,
      TypeFunctionTypeVariant::Function(TypeFunctionFunctionType {
        generics: generic_types,
        generic_packs,
        arg_types,
        ret_types,
        arg_names: Vec::new(),
      }),
      false,
    );

    1
  }
}

pub(crate) unsafe fn get_type_pack_runtime(
  l: *mut LuaState,
  head_idx: c_int,
  tail_idx: c_int,
) -> TypeFunctionTypePackId {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let mut head = Vec::new();

    if lua_istable!(vm_l, head_idx) {
      lua_pushvalue(vm_l, head_idx);

      for i in 1..=lua_objlen(vm_l, -1) {
        lua_pushinteger(vm_l, i);
        lua_gettable(vm_l, -2);

        if lua_isnil!(vm_l, -1) {
          lua_pop(vm_l, 1);
          break;
        }

        head.push(get_type_user_data(l, -1));
        lua_pop(vm_l, 1);
      }

      lua_pop(vm_l, 1);
    }

    let mut tail: Option<TypeFunctionTypePackId> = None;

    if let Some(type_id) = optional_type_user_data(l, tail_idx) {
      let gty = get_type_function_type_id::<TypeFunctionGenericType>(type_id);
      if !gty.is_null() && (*gty).is_pack {
        tail = Some(allocate_type_function_type_pack(
          l,
          TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
            is_named: (*gty).is_named,
            name: (*gty).name.clone(),
          }),
        ));
      } else {
        tail = Some(allocate_type_function_type_pack(
          l,
          TypeFunctionTypePackVariant::V1(TypeFunctionVariadicTypePack { type_id }),
        ));
      }
    }

    match tail {
      Some(t) if head.is_empty() => t,
      tail => allocate_type_function_type_pack(
        l,
        TypeFunctionTypePackVariant::V0(TypeFunctionTypePack { head, tail }),
      ),
    }
  }
}
