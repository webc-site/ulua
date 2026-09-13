//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:900-911`
//!
//! Dispatch for `void deserializeChildren(TypeFunctionTypePackId tftp, TypePackId tp)`.
use crate::{
  functions::{
    get_mutable_type_function_runtime_alt_f::get_mutable_type_function_type_pack_id,
    get_mutable_type_pack::get_mutable_type_pack_id,
  },
  records::{
    generic_type_pack::GenericTypePack, type_function_deserializer::TypeFunctionDeserializer,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_function_type_pack_id::TypeFunctionTypePackId, type_pack_id::TypePackId},
};

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn deserialize_children_type_function_type_pack_id_type_pack_id(
    &mut self,
    tftp: TypeFunctionTypePackId,
    tp: TypePackId,
  ) {
    // 各 arm 对照 C++ `if (auto [x1, x2] = tuple{...}; x1 && x2)` 链。

    match (
      get_mutable_type_pack_id::<TypePack>(tp),
      // SAFETY: tftp 非空契约同 C++ get_mutable，由运行时构造方保证。
      unsafe { get_mutable_type_function_type_pack_id::<TypeFunctionTypePack>(tftp) },
    ) {
      (Some(t_pack1), t_pack2) if !t_pack2.is_null() => {
        unsafe { self.deserialize_children_type_function_type_pack_type_pack(t_pack2, t_pack1) };
        return;
      }
      _ => {}
    }

    match (get_mutable_type_pack_id::<VariadicTypePack>(tp), unsafe {
      get_mutable_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tftp)
    }) {
      (Some(v_pack1), v_pack2) if !v_pack2.is_null() => {
        unsafe {
          self.deserialize_children_type_function_variadic_type_pack_variadic_type_pack(
            v_pack2, v_pack1,
          )
        };
        return;
      }
      _ => {}
    }

    match (get_mutable_type_pack_id::<GenericTypePack>(tp), unsafe {
      get_mutable_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tftp)
    }) {
      (Some(g_pack1), g_pack2) if !g_pack2.is_null() => {
        self
          .deserialize_children_type_function_generic_type_pack_generic_type_pack(g_pack2, g_pack1);
        return;
      }
      _ => {}
    }

    // SAFETY: state 由反序列化构造方保证有效。
    unsafe {
      (*(*self.state).ctx).ice.as_ref().ice_string(
        "Deserializing user defined type function arguments: mysterious type is being deserialized",
      );
    }
  }
}
