//! Faithful port of `TypeFunctionCloner::shallowClone(TypeFunctionTypeId ty)`
//! (Analysis/src/TypeFunctionRuntime.cpp:2641-2713).
use alloc::{collections::BTreeMap, vec::Vec};
use core::ptr::null;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::variant::Variant2};

use crate::{
  enums::type_type_function_runtime::Type,
  functions::get_type_function_runtime_alt_o::get_type_function_type_id,
  records::{
    type_function_any_type::TypeFunctionAnyType,
    type_function_boolean_singleton::TypeFunctionBooleanSingleton,
    type_function_cloner::TypeFunctionCloner, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_string_singleton::TypeFunctionStringSingleton,
    type_function_table_type::TypeFunctionTableType, type_function_type::TypeFunctionType,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_type_pack_var::TypeFunctionTypePackVar,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
  },
  type_aliases::{
    type_function_kind::TypeFunctionKind, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
impl TypeFunctionCloner {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn shallow_clone_type_function_type_id(
    &mut self,
    ty: TypeFunctionTypeId,
  ) -> TypeFunctionTypeId {
    // if (auto it = find(ty))
    //     return *it;
    if let Some(it) = self.find_type_function_type_id(ty) {
      return it;
    }

    unsafe {
      let runtime = self.type_function_runtime;

      // Create a shallow serialization
      // TypeFunctionTypeId target = {};
      let mut target: TypeFunctionTypeId = null();

      if let Some(p) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
        match p.r#type {
          Type::NilType => {
            target = (*runtime).type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: Type::NilType,
              }),
            ));
          }
          Type::Boolean => {
            target = (*runtime).type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: Type::Boolean,
              }),
            ));
          }
          Type::Number => {
            target = (*runtime).type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: Type::Number,
              }),
            ));
          }
          Type::Integer => {
            target = (*runtime).type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: Type::Integer,
              }),
            ));
          }
          Type::String => {
            target = (*runtime).type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: Type::String,
              }),
            ));
          }
          Type::Thread => {
            target = (*runtime).type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: Type::Thread,
              }),
            ));
          }
          Type::Buffer => {
            target = (*runtime).type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: Type::Buffer,
              }),
            ));
          }
        }
      } else if !get_type_function_type_id::<TypeFunctionUnknownType>(ty).is_null() {
        target =
          (*runtime)
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Unknown(
              TypeFunctionUnknownType { _unused: None },
            )));
      } else if !get_type_function_type_id::<TypeFunctionNeverType>(ty).is_null() {
        target =
          (*runtime)
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Never(
              TypeFunctionNeverType { _unused: None },
            )));
      } else if !get_type_function_type_id::<TypeFunctionAnyType>(ty).is_null() {
        target =
          (*runtime)
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Any(
              TypeFunctionAnyType { _unused: None },
            )));
      } else if let Some(s) = get_type_function_type_id::<TypeFunctionSingletonType>(ty).as_ref() {
        if let Some(bs) = s.variant.get_if::<TypeFunctionBooleanSingleton>() {
          target = (*runtime).type_arena.allocate(TypeFunctionType::new(
            TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
              variant: Variant2::V0(TypeFunctionBooleanSingleton { value: bs.value }),
            }),
          ));
        } else if let Some(ss) = s.variant.get_if::<TypeFunctionStringSingleton>() {
          target = (*runtime).type_arena.allocate(TypeFunctionType::new(
            TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
              variant: Variant2::V1(TypeFunctionStringSingleton {
                value: ss.value.clone(),
              }),
            }),
          ));
        }
      } else if !get_type_function_type_id::<TypeFunctionUnionType>(ty).is_null() {
        target =
          (*runtime)
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Union(
              TypeFunctionUnionType {
                components: Vec::new(),
              },
            )));
      } else if !get_type_function_type_id::<TypeFunctionIntersectionType>(ty).is_null() {
        target = (*runtime).type_arena.allocate(TypeFunctionType::new(
          TypeFunctionTypeVariant::Intersection(TypeFunctionIntersectionType {
            components: Vec::new(),
          }),
        ));
      } else if !get_type_function_type_id::<TypeFunctionNegationType>(ty).is_null() {
        target =
          (*runtime)
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Negation(
              TypeFunctionNegationType { type_id: null() },
            )));
      } else if !get_type_function_type_id::<TypeFunctionTableType>(ty).is_null() {
        target =
          (*runtime)
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Table(
              TypeFunctionTableType {
                props: BTreeMap::new(),
                indexer: None,
                metatable: None,
              },
            )));
      } else if !get_type_function_type_id::<TypeFunctionFunctionType>(ty).is_null() {
        // TypeFunctionTypePackId empty_type_pack = typePackArena.allocate(TypeFunctionTypePack{});
        let empty_type_pack: TypeFunctionTypePackId =
          (*runtime)
            .type_pack_arena
            .allocate(TypeFunctionTypePackVar::new(
              TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
                head: Vec::new(),
                tail: None,
              }),
            ));
        target =
          (*runtime)
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Function(
              TypeFunctionFunctionType {
                generics: Vec::new(),
                generic_packs: Vec::new(),
                arg_types: empty_type_pack,
                ret_types: empty_type_pack,
                arg_names: Vec::new(),
              },
            )));
      } else if !get_type_function_type_id::<TypeFunctionExternType>(ty).is_null() {
        // Don't copy a class since they are immutable
        target = ty;
      } else if let Some(g) = get_type_function_type_id::<TypeFunctionGenericType>(ty).as_ref() {
        target =
          (*runtime)
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Generic(
              TypeFunctionGenericType {
                is_named: g.is_named,
                is_pack: g.is_pack,
                name: g.name.clone(),
              },
            )));
      } else {
        LUAU_ASSERT!(false /* "Unknown type" */);
      }

      // types[ty] = target;
      *self.types.get_or_insert(ty) = target;
      // queue.emplace_back(ty, target);
      self
        .queue
        .push((TypeFunctionKind::V0(ty), TypeFunctionKind::V0(target)));
      target
    }
  }
}
