//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:866-898`
//!
//! Dispatch for `void deserializeChildren(TypeFunctionTypeId tfti, TypeId ty)`.
//! Each arm pairs the source `Type` variant (`get_mutable<T>(ty)`) with the runtime
//! variant (`get_mutable<TypeFunctionT>(tfti)`) and forwards to the leaf overload
//! `deserializeChildren(tfX, X)`. Table vs metatable share the runtime
//! `TypeFunctionTableType`, disambiguated by `metatable.has_value()`.
use crate::{
  functions::{
    get_mutable_type::get_mutable_type_id,
    get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id,
  },
  records::{
    any_type::AnyType, extern_type::ExternType, function_type::FunctionType,
    generic_type::GenericType, intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType,
    type_function_any_type::TypeFunctionAnyType,
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
    type_function_unknown_type::TypeFunctionUnknownType, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{type_function_type_id::TypeFunctionTypeId, type_id::TypeId},
};

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn deserialize_children_type_function_type_id_type_id(
    &mut self,
    tfti: TypeFunctionTypeId,
    ty: TypeId,
  ) {
    // 各 arm 对照 C++ `if (auto [x1, x2] = tuple{...}; x1 && x2)` 链：
    // 源侧 `getMutable<T>(ty)`（Option），运行时侧 `getMutable<TypeFunctionT>(tfti)`（裸指针判空）。

    match (
      get_mutable_type_id::<PrimitiveType>(ty),
      // SAFETY: tfti 非空契约同 C++ get_mutable，由运行时构造方保证。
      unsafe { get_mutable_type_function_type_id::<TypeFunctionPrimitiveType>(tfti) },
    ) {
      (Some(p1), p2) if !p2.is_null() => {
        self.deserialize_children_type_function_primitive_type_primitive_type(p2, p1);
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<UnknownType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionUnknownType>(tfti)
    }) {
      (Some(u1), u2) if !u2.is_null() => {
        self.deserialize_children_type_function_unknown_type_unknown_type(u2, u1);
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<NeverType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionNeverType>(tfti)
    }) {
      (Some(n1), n2) if !n2.is_null() => {
        self.deserialize_children_type_function_never_type_never_type(n2, n1);
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<AnyType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionAnyType>(tfti)
    }) {
      (Some(a1), a2) if !a2.is_null() => {
        self.deserialize_children_type_function_any_type_any_type(a2, a1);
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<SingletonType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionSingletonType>(tfti)
    }) {
      (Some(s1), s2) if !s2.is_null() => {
        self.deserialize_children_type_function_singleton_type_singleton_type(s2, s1);
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<UnionType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionUnionType>(tfti)
    }) {
      (Some(u1), u2) if !u2.is_null() => {
        unsafe { self.deserialize_children_type_function_union_type_union_type(u2, u1) };
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<IntersectionType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionIntersectionType>(tfti)
    }) {
      (Some(i1), i2) if !i2.is_null() => {
        unsafe {
          self.deserialize_children_type_function_intersection_type_intersection_type(i2, i1)
        };
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<NegationType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionNegationType>(tfti)
    }) {
      (Some(n1), n2) if !n2.is_null() => {
        unsafe { self.deserialize_children_type_function_negation_type_negation_type(n2, n1) };
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<TableType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionTableType>(tfti)
    }) {
      // C++: `t1 && t2 && !t2->metatable.has_value()`
      (Some(t1), t2) if !t2.is_null() && unsafe { (*t2).metatable.is_none() } => {
        unsafe { self.deserialize_children_type_function_table_type_table_type(t2, t1) };
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<MetatableType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionTableType>(tfti)
    }) {
      // C++: `m1 && m2 && m2->metatable.has_value()`
      (Some(m1), m2) if !m2.is_null() && unsafe { (*m2).metatable.is_some() } => {
        unsafe { self.deserialize_children_type_function_table_type_metatable_type(m2, m1) };
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<FunctionType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionFunctionType>(tfti)
    }) {
      (Some(f1), f2) if !f2.is_null() => {
        unsafe { self.deserialize_children_type_function_function_type_function_type(f2, f1) };
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<ExternType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionExternType>(tfti)
    }) {
      (Some(c1), c2) if !c2.is_null() => {
        self.deserialize_children_type_function_extern_type_extern_type(c2, c1);
        return;
      }
      _ => {}
    }

    match (get_mutable_type_id::<GenericType>(ty), unsafe {
      get_mutable_type_function_type_id::<TypeFunctionGenericType>(tfti)
    }) {
      (Some(g1), g2) if !g2.is_null() => {
        self.deserialize_children_type_function_generic_type_generic_type(g2, g1);
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
