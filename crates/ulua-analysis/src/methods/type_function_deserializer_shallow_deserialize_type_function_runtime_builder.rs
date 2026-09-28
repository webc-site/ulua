use alloc::{format, vec::Vec};
use core::ptr::null;

use crate::{
  enums::{table_state::TableState, type_type_function_runtime::Type as TypeFunctionPrimitiveKind},
  functions::get_type_function_runtime::{
    get_type_function_type_id, get_type_function_type_pack_id,
  },
  records::{
    boolean_singleton::BooleanSingleton, function_type::FunctionType,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, singleton_type::SingletonType, string_singleton::StringSingleton,
    table_type::TableType, r#type::Type, type_function_any_type::TypeFunctionAnyType,
    type_function_deserializer::TypeFunctionDeserializer,
    type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType, type_function_type_pack::TypeFunctionTypePack,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack, type_pack::TypePack,
    union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    singleton_variant::SingletonVariant, type_function_kind::TypeFunctionKind,
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId,
  },
};

impl TypeFunctionDeserializer {
  pub(crate) fn shallow_deserialize_type_function_type_id(
    &mut self,
    ty: TypeFunctionTypeId,
  ) -> TypeId {
    if let Some(it) = self.find_type_function_type_id(ty) {
      return it;
    }

    let make_empty_table = || TableType {
      state: TableState::Sealed,
      ..Default::default()
    };

    // Safety: `self.state` 是 `TypeFunctionDeserializer` 由 RuntimeBuilder 经
    // `type_function_deserializer` setter 接线的 `*mut TypeFunctionRuntimeBuilderState`，非空且
    // 贯穿整段反序列化存活；`(*self.state).ctx` 是其会话 `Handle<TypeFunctionContext>`，
    // 由构造点的 `&mut` 借用接线，类型编码非空。`ctx.arena`/`builtins`/`ice` 为 NonNull
    // `as_ptr()` 取回的非空句柄：arena 写入
    // （`add_type`/`add_tv`/`add_type_pack_t`）是 bump 追加、不移动既有节点；`get_type_function_type_id::<T>(ty).as_ref()`
    // 依 RTTI class-index 命中才返回 Some⇒类型正确、`(*builtins).x_type` 只读 Copy；单线程独占驱动
    // 下对这些 arena/上下文的可变借用不并存。
    unsafe {
      let ctx = (*self.state).ctx.get_mut();
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

  /// # Safety
  /// 调用方须保证 `self.state` 非空且指向反序列化期间存活的 builder state，其 `ctx` 构造接线恒非空；
  /// 被解引用的 bump arena 块地址不移动，`tp` 为待反序列化的存活序列化型 pack 句柄。
  /// 单线程独占驱动下对这些 arena/上下文的可变借用不并存。cpp `Analysis/src/TypeFunctionRuntimeBuilder.cpp:826`。
  pub unsafe fn shallow_deserialize_type_function_type_pack_id(
    &mut self,
    tp: TypeFunctionTypePackId,
  ) -> TypePackId {
    if let Some(it) = self.find_type_function_type_pack_id(tp) {
      return it;
    }

    // Safety: 同 `shallow_deserialize_type_function_type_id`——`self.state` 为 RuntimeBuilder
    // 反序列化期间构造期接线、非空存活的会话裸句柄，`(*self.state).ctx` 是其 `Handle`（类型编码
    // 非空，构造点为真实 `&mut` 借用），`ctx.arena`/`ice` 经
    // NonNull `as_ptr()` 取回亦非空。arena 上的 `add_type_pack_t` 是 bump 追加、不移动既有节点；
    // `get_type_function_type_pack_id::<T>(tp)` 依 RTTI class-index 命中才判为该变体；单线程独占
    // 驱动下对这些 arena/上下文的可变借用不并存，末尾 `*self.packs.get_or_insert(tp)` 亦为自有 map。
    unsafe {
      let ctx = (*self.state).ctx.get_mut();
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
