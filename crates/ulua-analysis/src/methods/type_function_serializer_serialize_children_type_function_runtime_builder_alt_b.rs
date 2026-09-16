use crate::{
  functions::{follow_type_pack::follow_type_pack_id, get_type_pack::get_type_pack_id},
  records::{
    generic_type_pack::GenericTypePack, type_function_serializer::TypeFunctionSerializer,
    type_function_type_pack_var::TypeFunctionTypePackVar, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant, type_pack_id::TypePackId,
  },
};

impl TypeFunctionSerializer {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn serialize_children_type_pack_id_type_function_type_pack_id(
    &mut self,
    tp: TypePackId,
    tftp: TypeFunctionTypePackId,
  ) {
    if tftp.is_null() {
      return;
    }

    let tp = unsafe { follow_type_pack_id(tp) };

    // SAFETY: tftp 已判空，由运行时构造方保证有效。
    let target = unsafe { &mut (*(tftp as *mut TypeFunctionTypePackVar)).type_variant };

    if let Some(source) = get_type_pack_id::<TypePack>(tp)
      && let TypeFunctionTypePackVariant::V0(target) = target
    {
      unsafe { self.serialize_children_type_pack_type_function_type_pack(source, target) };
    } else if let Some(source) = get_type_pack_id::<VariadicTypePack>(tp)
      && let TypeFunctionTypePackVariant::V1(target) = target
    {
      unsafe {
        self.serialize_children_variadic_type_pack_type_function_variadic_type_pack(source, target)
      };
    } else if let Some(source) = get_type_pack_id::<GenericTypePack>(tp)
      && let TypeFunctionTypePackVariant::V2(target) = target
    {
      self.serialize_children_generic_type_pack_type_function_generic_type_pack(source, target);
    }
  }
}
