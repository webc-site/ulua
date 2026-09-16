use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_mutable_type_function_runtime_alt_f::get_mutable_type_function_type_pack_id,
  records::{
    type_function_cloner::TypeFunctionCloner,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::type_function_type_pack_id::TypeFunctionTypePackId,
};

impl TypeFunctionCloner {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_type_function_type_pack_id_type_function_type_pack_id(
    &mut self,
    tp: TypeFunctionTypePackId,
    tftp: TypeFunctionTypePackId,
  ) {
    unsafe {
      let t_pack1: *mut TypeFunctionTypePack =
        get_mutable_type_function_type_pack_id::<TypeFunctionTypePack>(tp);
      let t_pack2: *mut TypeFunctionTypePack =
        get_mutable_type_function_type_pack_id::<TypeFunctionTypePack>(tftp);

      if !t_pack1.is_null() && !t_pack2.is_null() {
        self.clone_children_type_function_type_pack_type_function_type_pack(t_pack1, t_pack2);
      } else {
        let v_pack1: *mut TypeFunctionVariadicTypePack =
          get_mutable_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp);
        let v_pack2: *mut TypeFunctionVariadicTypePack =
          get_mutable_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tftp);

        if !v_pack1.is_null() && !v_pack2.is_null() {
          self.clone_children_type_function_variadic_type_pack_type_function_variadic_type_pack(
            v_pack1, v_pack2,
          );
        } else {
          let g_pack1: *mut TypeFunctionGenericTypePack =
            get_mutable_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp);
          let g_pack2: *mut TypeFunctionGenericTypePack =
            get_mutable_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tftp);

          if !g_pack1.is_null() && !g_pack2.is_null() {
            self.clone_children_type_function_generic_type_pack_type_function_generic_type_pack(
              g_pack1, g_pack2,
            );
          } else {
            LUAU_ASSERT!(false);
          }
        }
      }
    }
  }
}
