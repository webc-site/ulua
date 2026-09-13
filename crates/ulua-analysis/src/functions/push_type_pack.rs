use ulua_vm::{
  functions::{
    lua_createtable::lua_createtable, lua_l_error_l::lua_l_error_l, lua_rawseti::lua_rawseti,
    lua_setfield::lua_setfield,
  },
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_type_function_runtime_alt_n::get_type_function_type_pack_id,
  },
  records::{
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn push_type_pack(l: *mut LuaState, tp: TypeFunctionTypePackId) {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;

    let tftp = get_type_function_type_pack_id::<TypeFunctionTypePack>(tp);
    if !tftp.is_null() {
      lua_createtable(vm_l, 0, 2);

      if !(*tftp).head.is_empty() {
        lua_createtable(vm_l, (*tftp).head.len() as i32, 0);
        for (idx, el) in (*tftp).head.iter().enumerate() {
          alloc_type_user_data(l, (**el).type_variant.clone(), false);
          lua_rawseti(vm_l, -2, (idx + 1) as i32);
        }

        lua_setfield(vm_l, -2, c"head".as_ptr());
      }

      if let Some(tail) = (*tftp).tail {
        push_type_pack_tail(l, vm_l, tail);
        lua_setfield(vm_l, -2, c"tail".as_ptr());
      }
    } else {
      let tfvp = get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp);
      if !tfvp.is_null() {
        lua_createtable(vm_l, 0, 1);

        alloc_type_user_data(l, (*(*tfvp).type_id).type_variant.clone(), false);
        lua_setfield(vm_l, -2, c"tail".as_ptr());
      } else {
        let tfgp = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp);
        if !tfgp.is_null() {
          lua_createtable(vm_l, 0, 1);

          alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
              is_named: (*tfgp).is_named,
              is_pack: true,
              name: (*tfgp).name.clone(),
            }),
            false,
          );
          lua_setfield(vm_l, -2, c"tail".as_ptr());
        } else {
          lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!("unsupported type pack type"),
          );
        }
      }
    }
  }
}

unsafe fn push_type_pack_tail(
  l: *mut LuaState,
  vm_l: *mut lua_state::LuaState,
  tail: TypeFunctionTypePackId,
) {
  unsafe {
    let tfvp = get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tail);
    if !tfvp.is_null() {
      alloc_type_user_data(l, (*(*tfvp).type_id).type_variant.clone(), false);
      return;
    }

    let tfgp = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tail);
    if !tfgp.is_null() {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
          is_named: (*tfgp).is_named,
          is_pack: true,
          name: (*tfgp).name.clone(),
        }),
        false,
      );
      return;
    }

    lua_l_error_l(
      vm_l,
      c"%s".as_ptr(),
      core::format_args!("unsupported type pack type"),
    );
  }
}
