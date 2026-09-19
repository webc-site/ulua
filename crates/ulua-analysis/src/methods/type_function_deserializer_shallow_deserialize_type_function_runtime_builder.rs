use alloc::{format, vec::Vec};
use core::ptr::{null, null_mut};

use crate::{
  enums::{table_state::TableState, type_type_function_runtime::Type as TypeFunctionPrimitiveKind},
  functions::get_type_function_runtime_alt_o::get_type_function_type_id,
  records::{
    boolean_singleton::BooleanSingleton, function_type::FunctionType,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, singleton_type::SingletonType, string_singleton::StringSingleton,
    table_type::TableType, r#type::Type, type_function_any_type::TypeFunctionAnyType,
    type_function_deserializer::TypeFunctionDeserializer,
    type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType, type_level::TypeLevel,
    type_pack::TypePack, union_type::UnionType,
  },
  type_aliases::{
    props_type::Props, singleton_variant::SingletonVariant, tags::Tags,
    type_function_kind::TypeFunctionKind, type_function_type_id::TypeFunctionTypeId,
    type_id::TypeId, type_or_pack::TypeOrPack,
  },
};
impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn shallow_deserialize_type_function_type_id(
    &mut self,
    ty: TypeFunctionTypeId,
  ) -> TypeId {
    if let Some(it) = self.find_type_function_type_id(ty) {
      return it;
    }

    let make_empty_table = || TableType {
      props: Props::default(),
      indexer: None,
      state: TableState::Sealed,
      level: TypeLevel::default(),
      scope: null_mut(),
      name: None,
      synthetic_name: None,
      instantiated_type_params: Vec::new(),
      instantiated_type_pack_params: Vec::new(),
      definition_module_name: Default::default(),
      definition_location: Default::default(),
      bound_to: None,
      tags: Tags::default(),
      remaining_props: 0,
    };

    unsafe {
      let ctx = &mut *(*self.state).ctx;
      let arena = ctx.arena.as_ptr();
      let builtins = ctx.builtins.as_ptr();
      let target: TypeId;

      if let Some(p) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
        target = match p.r#type {
          TypeFunctionPrimitiveKind::NilType => (*builtins).nil_type,
          TypeFunctionPrimitiveKind::Boolean => (*builtins).boolean_type,
          TypeFunctionPrimitiveKind::Number => (*builtins).number_type,
          TypeFunctionPrimitiveKind::Integer => (*builtins).integer_type,
          TypeFunctionPrimitiveKind::String => (*builtins).string_type,
          TypeFunctionPrimitiveKind::Thread => (*builtins).thread_type,
          TypeFunctionPrimitiveKind::Buffer => (*builtins).buffer_type,
        };
      } else if !get_type_function_type_id::<TypeFunctionUnknownType>(ty).is_null() {
        target = (*builtins).unknown_type;
      } else if !get_type_function_type_id::<TypeFunctionNeverType>(ty).is_null() {
        target = (*builtins).never_type;
      } else if !get_type_function_type_id::<TypeFunctionAnyType>(ty).is_null() {
        target = (*builtins).any_type;
      } else if let Some(s) = get_type_function_type_id::<TypeFunctionSingletonType>(ty).as_ref() {
        if let Some(bs) = s.variant.get_if_0() {
          target = (*arena).add_type(SingletonType {
            variant: SingletonVariant::V0(BooleanSingleton { value: bs.value }),
          });
        } else if let Some(ss) = s.variant.get_if_1() {
          target = (*arena).add_type(SingletonType {
            variant: SingletonVariant::V1(StringSingleton {
              value: ss.value.clone(),
            }),
          });
        } else {
          (*ctx.ice.as_ptr()).ice_string(
                        "Deserializing user defined type function arguments: mysterious type is being deserialized",
                    );
          return null();
        }
      } else if !get_type_function_type_id::<TypeFunctionUnionType>(ty).is_null() {
        target = (*arena).add_tv(Type::from(UnionType {
          options: Vec::new(),
        }));
      } else if !get_type_function_type_id::<TypeFunctionIntersectionType>(ty).is_null() {
        target = (*arena).add_tv(Type::from(IntersectionType { parts: Vec::new() }));
      } else if !get_type_function_type_id::<TypeFunctionNegationType>(ty).is_null() {
        target = (*arena).add_type(NegationType::new((*builtins).unknown_type));
      } else if let Some(table) = get_type_function_type_id::<TypeFunctionTableType>(ty).as_ref() {
        if table.metatable.is_none() {
          target = (*arena).add_type(make_empty_table());
        } else {
          let empty_table = (*arena).add_type(make_empty_table());
          target = (*arena).add_type(MetatableType::new(empty_table, empty_table));
        }
      } else if !get_type_function_type_id::<TypeFunctionFunctionType>(ty).is_null() {
        let empty_type_pack = (*arena).add_type_pack_t(TypePack::new(Vec::new(), None));
        target = (*arena).add_type(FunctionType::function_type_new(
          empty_type_pack,
          empty_type_pack,
          None,
          false,
        ));
      } else if let Some(c) = get_type_function_type_id::<TypeFunctionExternType>(ty).as_ref() {
        target = c.extern_ty;
      } else if let Some(g) = get_type_function_type_id::<TypeFunctionGenericType>(ty).as_ref() {
        if g.is_pack() {
          self.push_runtime_error(format!(
            "Generic type pack '{}...' cannot be placed in a type position",
            g.name()
          ));
          return null();
        }

        if let Some(mapping) = self
          .generic_types
          .iter()
          .rev()
          .find(|el| el.is_named == g.is_named() && el.name == g.name())
          .map(|el| el.r#type)
        {
          target = mapping;
        } else {
          self.push_runtime_error(format!(
            "Generic type '{}' is not in a scope of the active generic function",
            g.name()
          ));
          return null();
        }
      } else {
        (*ctx.ice.as_ptr()).ice_string(
                    "Deserializing user defined type function arguments: mysterious type is being deserialized",
                );
        return null();
      }

      *self.types.get_or_insert(ty) = target;
      self
        .queue
        .push((TypeFunctionKind::V0(ty), TypeOrPack::V0(target)));
      target
    }
  }
}
