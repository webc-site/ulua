use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  enums::reduction::Reduction,
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type::follow_type_id,
    get_type_alt_j::get_type_id,
  },
  records::{
    any_type::AnyType,
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
  let target_ty = follow_type_id(target_ty);
  // SAFETY: ctx 各 NonNull 字段由类型函数调度器初始化（C++ TypeFunction.h 同契约）。
  let builtins = unsafe { ctx.builtins.as_ref() };
  let arena = unsafe { &mut *ctx.arena.as_ptr() };

  let mut result: Option<TypeId> = None;
  let mut erroneous = true;

  if get_type_id::<TableType>(target_ty).is_some() {
    erroneous = false;
  }

  if let Some(mt) = get_type_id::<MetatableType>(target_ty) {
    result = Some(mt.metatable());
    erroneous = false;
  }

  if let Some(class_type) = get_type_id::<ExternType>(target_ty) {
    result = class_type.metatable;
    erroneous = false;
  }

  if let Some(primitive) = get_type_id::<PrimitiveType>(target_ty) {
    if primitive.r#type == PrimitiveKind::Table {
      result = Some(arena.add_type(UnionType {
        options: Vec::from([builtins.table_type, builtins.nil_type]),
      }));
    } else {
      result = primitive.metatable;
    }
    erroneous = false;
  }

  if let Some(singleton) = get_type_id::<SingletonType>(target_ty) {
    if StringSingleton::get_if(&singleton.variant).is_some()
      && let Some(primitive_string) = get_type_id::<PrimitiveType>(builtins.string_type)
    {
      result = primitive_string.metatable;
    }
    erroneous = false;
  }

  if get_type_id::<AnyType>(target_ty).is_some() {
    result = Some(target_ty);
    erroneous = false;
  }

  if get_type_id::<ErrorType>(target_ty).is_some() {
    result = Some(target_ty);
    erroneous = false;
  }

  if erroneous {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::Erroneous,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  let mut dummy = Vec::new();
  let metatable_metamethod = find_metatable_entry(
    ctx.builtins.as_ptr(),
    &mut dummy,
    target_ty,
    "__metatable",
    *location,
  );

  if let Some(metatable_metamethod) = metatable_metamethod {
    return TypeFunctionReductionResult {
      result: Some(metatable_metamethod),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  TypeFunctionReductionResult {
    result: Some(result.unwrap_or(builtins.nil_type)),
    reduction_status: Reduction::MaybeOk,
    blocked_types: Vec::new(),
    blocked_packs: Vec::new(),
    error: None,
    messages: Vec::new(),
  }
}
