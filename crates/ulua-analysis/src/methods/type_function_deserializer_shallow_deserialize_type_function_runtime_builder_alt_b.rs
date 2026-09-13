use alloc::{format, vec::Vec};
use core::ptr::null;

use crate::{
  functions::get_type_function_runtime_alt_n::get_type_function_type_pack_id,
  records::{
    type_function_deserializer::TypeFunctionDeserializer,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    type_function_kind::TypeFunctionKind, type_function_type_pack_id::TypeFunctionTypePackId,
    type_or_pack::TypeOrPack, type_pack_id::TypePackId,
  },
};
impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn shallow_deserialize_type_function_type_pack_id(
    &mut self,
    tp: TypeFunctionTypePackId,
  ) -> TypePackId {
    if let Some(it) = self.find_type_function_type_pack_id(tp) {
      return it;
    }

    unsafe {
      let ctx = &mut *(*self.state).ctx;
      let arena = ctx.arena.as_ptr();
      let target: TypePackId;

      if !get_type_function_type_pack_id::<TypeFunctionTypePack>(tp).is_null() {
        target = (*arena).add_type_pack_t(TypePack::new(Vec::new(), None));
      } else if !get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp).is_null() {
        target = (*arena).add_type_pack_t(VariadicTypePack::default());
      } else if let Some(g_pack) =
        get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp).as_ref()
      {
        if let Some(mapping) = self
          .generic_packs
          .iter()
          .rev()
          .find(|el| el.is_named == g_pack.is_named() && el.name == g_pack.name())
          .map(|el| el.r#type)
        {
          target = mapping;
        } else {
          self.push_runtime_error(format!(
            "Generic type pack '{}...' is not in a scope of the active generic function",
            g_pack.name()
          ));
          return null();
        }
      } else {
        (*ctx.ice.as_ptr()).ice_string(
                    "Deserializing user defined type function arguments: mysterious type is being deserialized",
                );
        return null();
      }

      *self.packs.get_or_insert(tp) = target;
      self
        .queue
        .push((TypeFunctionKind::V1(tp), TypeOrPack::V1(target)));
      target
    }
  }
}
