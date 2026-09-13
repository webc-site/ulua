use ulua_common::records::variant::Variant2;

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    is_approximately_falsy_type::is_approximately_falsy_type,
    is_approximately_truthy_type::is_approximately_truthy_type,
  },
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton, error_type::ErrorType,
    extern_type::ExternType, function_type::FunctionType, metatable_type::MetatableType,
    never_type::NeverType, primitive_type::PrimitiveType, singleton_type::SingletonType,
    table_type::TableType, type_simplifier::TypeSimplifier, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn basic_intersect_with_falsy(&self, target: TypeId) -> Option<TypeId> {
    let builtin_types = unsafe { &*self.builtin_types };
    let target = follow_type_id(target);

    if is_approximately_truthy_type(target) {
      return Some(builtin_types.never_type);
    }

    if is_approximately_falsy_type(target) {
      return Some(target);
    }

    if get_type_id::<NeverType>(target).is_some() || get_type_id::<ErrorType>(target).is_some() {
      return Some(target);
    }

    if get_type_id::<AnyType>(target).is_some() {
      // any = *error-type* | unknown, so falsy & any = *error-type* | falsy
      let arena = unsafe { &mut *self.arena.cast_mut() };
      return Some(arena.add_type(UnionType {
        options: alloc::vec![builtin_types.falsy_type, builtin_types.error_type],
      }));
    }

    if get_type_id::<UnknownType>(target).is_some() {
      return Some(builtin_types.falsy_type);
    }

    if get_type_id::<FunctionType>(target).is_some()
      || get_type_id::<TableType>(target).is_some()
      || get_type_id::<MetatableType>(target).is_some()
      || get_type_id::<ExternType>(target).is_some()
    {
      return Some(builtin_types.never_type);
    }

    if let Some(pt) = get_type_id::<PrimitiveType>(target) {
      return Some(match pt.r#type {
        PrimitiveType::NIL_TYPE => builtin_types.nil_type,
        PrimitiveType::BOOLEAN => builtin_types.false_type,
        _ => builtin_types.never_type,
      });
    }

    if let Some(st) = get_type_id::<SingletonType>(target) {
      return Some(
        if st.variant == Variant2::V0(BooleanSingleton::new(false)) {
          builtin_types.false_type
        } else {
          builtin_types.never_type
        },
      );
    }

    None
  }
}
