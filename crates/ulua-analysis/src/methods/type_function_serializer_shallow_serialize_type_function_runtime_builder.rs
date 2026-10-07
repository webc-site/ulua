use alloc::{collections::BTreeMap, format, vec::Vec};
use core::ptr::null;

use ulua_ast::records::location::Location;
use ulua_common::{fflag, records::variant::Variant2};

use crate::{
  enums::type_type_function_runtime::Type as TypeFunctionPrimitiveKind,
  functions::{
    follow_type, follow_type_pack, get_type, get_type_pack,
    to_string_to_string::{to_string_type_id, to_string_type_pack_id},
  },
  methods::type_function_serde_checks::{RUNTIME_WIRED, STATE_WIRED},
  records::{
    any_type::AnyType,
    boolean_singleton::BooleanSingleton,
    extern_type::ExternType,
    function_type::FunctionType,
    generic_type::GenericType,
    generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    never_type::NeverType,
    primitive_type::{PrimitiveType, Type as PrimitiveKind},
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    type_function_any_type::TypeFunctionAnyType,
    type_function_boolean_singleton::TypeFunctionBooleanSingleton,
    type_function_error::TypeFunctionError,
    type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_serializer::TypeFunctionSerializer,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_string_singleton::TypeFunctionStringSingleton,
    type_function_table_type::TypeFunctionTableType,
    type_function_type::TypeFunctionType,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_type_pack_var::TypeFunctionTypePackVar,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
    type_pack::TypePack,
    union_type::UnionType,
    unknown_type::UnknownType,
    unsupported_type::UnsupportedType,
    unsupported_type_pack::UnsupportedTypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    module_name_type::ModuleName, type_function_error_data::TypeFunctionErrorData,
    type_function_kind::TypeFunctionKind, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant,
    type_function_type_variant::TypeFunctionTypeVariant, type_id::TypeId, type_or_pack::TypeOrPack,
    type_pack_id::TypePackId,
  },
};

impl TypeFunctionSerializer {
  pub fn shallow_serialize_type_id(&mut self, ty: TypeId) -> TypeFunctionTypeId {
    let ty = follow_type::follow(ty);

    // if (auto it = find(ty)) return *it;
    if let Some(it) = self.types.get(&ty).copied() {
      return it;
    }

    // state/runtime 由 `type_function_serializer` 装配步接线，本轮序列化会话内恒已装配；
    // 原「判空后裸 `&mut *` 重建借用」的前提改由 Option + expect 承载（review.md §2）。
    // 两对象互异，单线程串行下同时重建两个 &mut 无在册别名。
    let state = self.state.expect(STATE_WIRED).get_mut();
    let runtime = self.type_function_runtime.expect(RUNTIME_WIRED).get_mut();

    // Create a shallow serialization
    let mut target: TypeFunctionTypeId = null();

    {
      if let Some(p) = get_type::get::<PrimitiveType>(ty).as_ref() {
        match p.r#type {
          PrimitiveKind::NilType => {
            target = runtime.type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: TypeFunctionPrimitiveKind::NilType,
              }),
            ));
          }
          PrimitiveKind::Boolean => {
            target = runtime.type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: TypeFunctionPrimitiveKind::Boolean,
              }),
            ));
          }
          PrimitiveKind::Number => {
            target = runtime.type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: TypeFunctionPrimitiveKind::Number,
              }),
            ));
          }
          PrimitiveKind::Integer => {
            target = runtime.type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: TypeFunctionPrimitiveKind::Integer,
              }),
            ));
          }
          PrimitiveKind::String => {
            target = runtime.type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: TypeFunctionPrimitiveKind::String,
              }),
            ));
          }
          PrimitiveKind::Thread => {
            target = runtime.type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: TypeFunctionPrimitiveKind::Thread,
              }),
            ));
          }
          PrimitiveKind::Buffer => {
            target = runtime.type_arena.allocate(TypeFunctionType::new(
              TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType {
                r#type: TypeFunctionPrimitiveKind::Buffer,
              }),
            ));
          }
          // case Function: case Table: default:
          _ => {
            if fflag::LuauTypeFunctionStructuredErrors.get() {
              state.errors.push(TypeFunctionError {
                location: Location::default(),
                module_name: ModuleName::new(),
                data: TypeFunctionErrorData::V0(UnsupportedType { r#type: ty }),
              });
            } else {
              state.errors_deprecated.push(format!(
                "Argument of primitive type {} is not currently serializable by type functions",
                to_string_type_id(ty)
              ));
            }
          }
        }
      } else if get_type::get::<UnknownType>(ty).is_some() {
        target =
          runtime
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Unknown(
              TypeFunctionUnknownType,
            )));
      } else if get_type::get::<NeverType>(ty).is_some() {
        target =
          runtime
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Never(
              TypeFunctionNeverType,
            )));
      } else if get_type::get::<AnyType>(ty).is_some() {
        target = runtime
          .type_arena
          .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Any(
            TypeFunctionAnyType,
          )));
      } else if let Some(s) = get_type::get::<SingletonType>(ty).as_ref() {
        if let Some(bs) = s.variant.get_if::<BooleanSingleton>() {
          target =
            runtime
              .type_arena
              .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Singleton(
                TypeFunctionSingletonType {
                  variant: Variant2::V0(TypeFunctionBooleanSingleton { value: bs.value }),
                },
              )));
        } else if let Some(ss) = s.variant.get_if::<StringSingleton>() {
          target =
            runtime
              .type_arena
              .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Singleton(
                TypeFunctionSingletonType {
                  variant: Variant2::V1(TypeFunctionStringSingleton {
                    value: ss.value.clone(),
                  }),
                },
              )));
        } else {
          if fflag::LuauTypeFunctionStructuredErrors.get() {
            state.errors.push(TypeFunctionError {
              location: Location::default(),
              module_name: ModuleName::new(),
              data: TypeFunctionErrorData::V0(UnsupportedType { r#type: ty }),
            });
          } else {
            state.errors_deprecated.push(format!(
              "Argument of singleton type {} is not currently serializable by type functions",
              to_string_type_id(ty)
            ));
          }
        }
      } else if get_type::get::<UnionType>(ty).is_some() {
        target =
          runtime
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Union(
              TypeFunctionUnionType {
                components: Vec::new(),
              },
            )));
      } else if get_type::get::<IntersectionType>(ty).is_some() {
        target = runtime.type_arena.allocate(TypeFunctionType::new(
          TypeFunctionTypeVariant::Intersection(TypeFunctionIntersectionType {
            components: Vec::new(),
          }),
        ));
      } else if get_type::get::<NegationType>(ty).is_some() {
        target =
          runtime
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Negation(
              TypeFunctionNegationType { type_id: null() },
            )));
      } else if get_type::get::<TableType>(ty).is_some()
        || get_type::get::<MetatableType>(ty).is_some()
      {
        target =
          runtime
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Table(
              TypeFunctionTableType {
                props: BTreeMap::new(),
                indexer: None,
                metatable: None,
              },
            )));
      } else if get_type::get::<FunctionType>(ty).is_some() {
        let empty_type_pack: TypeFunctionTypePackId =
          runtime
            .type_pack_arena
            .allocate(TypeFunctionTypePackVar::new(
              TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
                head: Vec::new(),
                tail: None,
              }),
            ));
        target =
          runtime
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
      } else if get_type::get::<ExternType>(ty).is_some() {
        // Since there aren't any new class types being created in type functions, we will
        // deserialize by using a direct reference to the original class
        target =
          runtime
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Extern(
              TypeFunctionExternType {
                props: BTreeMap::new(),
                indexer: None,
                metatable: None,
                read_parent: None,
                write_parent: None,
                extern_ty: ty,
              },
            )));
      } else if let Some(g) = get_type::get::<GenericType>(ty).as_ref() {
        let mut name = g.name.clone();

        if !g.explicit_name {
          name = format!("g{}", g.index);
        }

        target =
          runtime
            .type_arena
            .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Generic(
              TypeFunctionGenericType {
                is_named: g.explicit_name,
                is_pack: false,
                name,
              },
            )));
      } else {
        if fflag::LuauTypeFunctionStructuredErrors.get() {
          state.errors.push(TypeFunctionError {
            location: Location::default(),
            module_name: ModuleName::new(),
            data: TypeFunctionErrorData::V0(UnsupportedType { r#type: ty }),
          });
        } else {
          state.errors_deprecated.push(format!(
            "Argument of type {} is not currently serializable by type functions",
            to_string_type_id(ty)
          ));
        }
      }
    }

    // types[ty] = target;
    *self.types.get_or_insert(ty) = target;
    // queue.emplace_back(ty, target);
    self
      .queue
      .push((TypeOrPack::V0(ty), TypeFunctionKind::V0(target)));
    target
  }

  /// 对应 cpp `Analysis/src/TypeFunctionRuntimeBuilder.cpp:257`。state/runtime 的
  /// 装配前提已由 `Option<Handle<_>>` + expect 在函数内自证，调用点无须再携带
  /// `unsafe`（review.md §2：纯逻辑层 `pub unsafe fn` 待消灭）。
  pub fn shallow_serialize_type_pack_id(&mut self, tp: TypePackId) -> TypeFunctionTypePackId {
    let tp = follow_type_pack::follow(tp);

    // if (auto it = find(tp)) return *it;
    if let Some(it) = self.packs.get(&tp).copied() {
      return it;
    }

    // Safety 前提同 shallow_serialize_type_id：state/runtime 本轮会话内恒已装配。
    let state = self.state.expect(STATE_WIRED).get_mut();
    let runtime = self.type_function_runtime.expect(RUNTIME_WIRED).get_mut();

    // Create a shallow serialization
    let mut target: TypeFunctionTypePackId = null();

    {
      if get_type_pack::get::<TypePack>(tp).is_some() {
        target = runtime
          .type_pack_arena
          .allocate(TypeFunctionTypePackVar::new(
            TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
              head: Vec::new(),
              tail: None,
            }),
          ));
      } else if get_type_pack::get::<VariadicTypePack>(tp).is_some() {
        target = runtime
          .type_pack_arena
          .allocate(TypeFunctionTypePackVar::new(
            TypeFunctionTypePackVariant::V1(TypeFunctionVariadicTypePack { type_id: null() }),
          ));
      } else if let Some(g_pack) = get_type_pack::get::<GenericTypePack>(tp).as_ref() {
        let mut name = g_pack.name.clone();

        if !g_pack.explicit_name {
          name = format!("g{}", g_pack.index);
        }

        target = runtime
          .type_pack_arena
          .allocate(TypeFunctionTypePackVar::new(
            TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
              is_named: g_pack.explicit_name,
              name,
            }),
          ));
      } else {
        if fflag::LuauTypeFunctionStructuredErrors.get() {
          state.errors.push(TypeFunctionError {
            location: Location::default(),
            module_name: ModuleName::new(),
            data: TypeFunctionErrorData::V1(UnsupportedTypePack { pack: tp }),
          });
        } else {
          state.errors_deprecated.push(format!(
            "Argument of type pack {} is not currently serializable by type functions",
            to_string_type_pack_id(tp)
          ));
        }
      }
    }

    // packs[tp] = target;
    *self.packs.get_or_insert(tp) = target;
    // queue.emplace_back(tp, target);
    self
      .queue
      .push((TypeOrPack::V1(tp), TypeFunctionKind::V1(target)));
    target
  }
}
