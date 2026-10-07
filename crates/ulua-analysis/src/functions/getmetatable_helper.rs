use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  functions::{find_metatable_entry::find_metatable_entry, follow_type, get_type},
  records::{
    any_type::AnyType,
    arena_handle::{Handle, alias, alias_ref},
    extern_type::ExternType,
    metatable_type::MetatableType,
    primitive_type::{PrimitiveType, Type as PrimitiveKind},
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
    union_type::UnionType,
  },
  type_aliases::{
    error_type::ErrorType, singleton_variant::SingletonVariantMember, type_id::TypeId,
  },
};

pub fn getmetatable_helper(
  target_ty: TypeId,
  location: &Location,
  ctx: &TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let target_ty = follow_type::follow(target_ty);
  let builtins = alias_ref(ctx.builtins.as_ptr());
  let arena = alias(ctx.arena.as_ptr());

  let mut result: Option<TypeId> = None;
  let mut erroneous = true;

  if get_type::get::<TableType>(target_ty).is_some() {
    erroneous = false;
  }

  if let Some(mt) = get_type::get::<MetatableType>(target_ty) {
    result = Some(mt.metatable());
    erroneous = false;
  }

  if let Some(class_type) = get_type::get::<ExternType>(target_ty) {
    result = class_type.metatable;
    erroneous = false;
  }

  if let Some(primitive) = get_type::get::<PrimitiveType>(target_ty) {
    if primitive.r#type == PrimitiveKind::Table {
      result = Some(arena.add_type(UnionType {
        options: Vec::from([builtins.table_type, builtins.nil_type]),
      }));
    } else {
      result = primitive.metatable;
    }
    erroneous = false;
  }

  if let Some(singleton) = get_type::get::<SingletonType>(target_ty) {
    if StringSingleton::get_if(&singleton.variant).is_some()
      && let Some(primitive_string) = get_type::get::<PrimitiveType>(builtins.string_type)
    {
      result = primitive_string.metatable;
    }
    erroneous = false;
  }

  if get_type::get::<AnyType>(target_ty).is_some() {
    result = Some(target_ty);
    erroneous = false;
  }

  if get_type::get::<ErrorType>(target_ty).is_some() {
    result = Some(target_ty);
    erroneous = false;
  }

  if erroneous {
    return TypeFunctionReductionResult::erroneous();
  }

  let mut dummy = Vec::new();
  let metatable_metamethod = find_metatable_entry(
    Handle::from_ptr(ctx.builtins.as_ptr()),
    &mut dummy,
    target_ty,
    "__metatable",
    *location,
  );

  if let Some(metatable_metamethod) = metatable_metamethod {
    return TypeFunctionReductionResult::reduction(metatable_metamethod);
  }

  TypeFunctionReductionResult::reduction(result.unwrap_or(builtins.nil_type))
}
