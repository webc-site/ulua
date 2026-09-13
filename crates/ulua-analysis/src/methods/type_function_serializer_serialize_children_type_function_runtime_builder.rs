use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType, extern_type::ExternType, function_type::FunctionType,
    generic_type::GenericType, intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType,
    type_function_serializer::TypeFunctionSerializer, type_function_type::TypeFunctionType,
    union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_variant::TypeFunctionTypeVariant,
    type_id::TypeId,
  },
};

impl TypeFunctionSerializer {
  pub fn serialize_children_type_id_type_function_type_id(
    &mut self,
    ty: TypeId,
    tfti: TypeFunctionTypeId,
  ) {
    if tfti.is_null() {
      return;
    }

    let ty = follow_type_id(ty);

    // SAFETY: tfti 已判空，由运行时构造方保证有效。
    let target = unsafe { &mut (*(tfti as *mut TypeFunctionType)).type_variant };

    // 各 arm 对照 C++ `serializeChildren` 的 if-else 链：
    // 源变体 `get<T>(ty)` 与目标 `TypeFunctionTypeVariant` 一一配对。
    if let Some(source) = get_type_id::<PrimitiveType>(ty)
      && let TypeFunctionTypeVariant::Primitive(target) = target
    {
      self.serialize_children_primitive_type_type_function_primitive_type(source, target);
    } else if let Some(source) = get_type_id::<UnknownType>(ty)
      && let TypeFunctionTypeVariant::Unknown(target) = target
    {
      self.serialize_children_unknown_type_type_function_unknown_type(source, target);
    } else if let Some(source) = get_type_id::<NeverType>(ty)
      && let TypeFunctionTypeVariant::Never(target) = target
    {
      self.serialize_children_never_type_type_function_never_type(source, target);
    } else if let Some(source) = get_type_id::<AnyType>(ty)
      && let TypeFunctionTypeVariant::Any(target) = target
    {
      self.serialize_children_any_type_type_function_any_type(source, target);
    } else if let Some(source) = get_type_id::<SingletonType>(ty)
      && let TypeFunctionTypeVariant::Singleton(target) = target
    {
      self.serialize_children_singleton_type_type_function_singleton_type(source, target);
    } else if let Some(source) = get_type_id::<UnionType>(ty)
      && let TypeFunctionTypeVariant::Union(target) = target
    {
      unsafe { self.serialize_children_union_type_type_function_union_type(source, target) };
    } else if let Some(source) = get_type_id::<IntersectionType>(ty)
      && let TypeFunctionTypeVariant::Intersection(target) = target
    {
      unsafe {
        self.serialize_children_intersection_type_type_function_intersection_type(source, target)
      };
    } else if let Some(source) = get_type_id::<NegationType>(ty)
      && let TypeFunctionTypeVariant::Negation(target) = target
    {
      unsafe { self.serialize_children_negation_type_type_function_negation_type(source, target) };
    } else if let Some(source) = get_type_id::<TableType>(ty)
      && let TypeFunctionTypeVariant::Table(target) = target
    {
      unsafe { self.serialize_children_table_type_type_function_table_type(source, target) };
    } else if let Some(source) = get_type_id::<MetatableType>(ty)
      && let TypeFunctionTypeVariant::Table(target) = target
    {
      self.serialize_children_metatable_type_type_function_table_type(source, target);
    } else if let Some(source) = get_type_id::<FunctionType>(ty)
      && let TypeFunctionTypeVariant::Function(target) = target
    {
      unsafe { self.serialize_children_function_type_type_function_function_type(source, target) };
    } else if let Some(source) = get_type_id::<ExternType>(ty)
      && let TypeFunctionTypeVariant::Extern(target) = target
    {
      unsafe { self.serialize_children_extern_type_type_function_extern_type(source, target) };
    } else if let Some(source) = get_type_id::<GenericType>(ty)
      && let TypeFunctionTypeVariant::Generic(target) = target
    {
      self.serialize_children_generic_type_type_function_generic_type(source, target);
    }
  }
}
