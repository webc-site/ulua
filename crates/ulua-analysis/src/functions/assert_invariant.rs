use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{
    get_singleton_type::get_singleton_type, get_type, is_plain_tyvar::is_plain_tyvar,
    is_shallow_inhabited::is_shallow_inhabited,
    is_subclass_type::is_subclass_extern_type_extern_type, tyvar_index::tyvar_index,
  },
  records::{
    any_type::AnyType,
    boolean_singleton::BooleanSingleton,
    extern_type::ExternType,
    function_type::FunctionType,
    metatable_type::MetatableType,
    never_type::NeverType,
    normalized_extern_type::NormalizedExternType,
    normalized_function_type::NormalizedFunctionType,
    normalized_string_type::NormalizedStringType,
    normalized_type::NormalizedType,
    primitive_type::{self, PrimitiveType},
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    type_ids::TypeIds,
    unknown_type::UnknownType,
  },
  type_aliases::{error_type::ErrorType, normalized_tyvars::NormalizedTyvars, type_id::TypeId},
};

fn is_normalized_prim(ty: TypeId, expected: primitive_type::Type) -> bool {
  if get_type::get::<NeverType>(ty).is_some() {
    true
  } else if let Some(ptv) = get_type::get::<PrimitiveType>(ty).as_ref() {
    ptv.r#type == expected
  } else {
    false
  }
}

fn is_normalized_top(ty: TypeId) -> bool {
  get_type::get::<NeverType>(ty).is_some()
    || get_type::get::<AnyType>(ty).is_some()
    || get_type::get::<UnknownType>(ty).is_some()
}

fn is_normalized_boolean(ty: TypeId) -> bool {
  is_normalized_prim(ty, PrimitiveType::BOOLEAN)
    || get_type::get::<SingletonType>(ty)
      .and_then(get_singleton_type::<BooleanSingleton>)
      .is_some()
}

fn is_normalized_error(ty: TypeId) -> bool {
  get_type::get::<NeverType>(ty).is_some() || get_type::get::<ErrorType>(ty).is_some()
}

fn is_normalized_string(ty: &NormalizedStringType) -> bool {
  ty.is_string()
    || ty.singletons.iter().all(|(str, &type_id)| {
      get_type::get::<SingletonType>(type_id)
        .and_then(get_singleton_type::<StringSingleton>)
        .is_some_and(|sstv| sstv.value == *str)
    })
}

fn are_normalized_extern_types(tys: &NormalizedExternType) -> bool {
  let extern_types = &tys.extern_types;

  for (ty, negations) in extern_types.iter() {
    let Some(etv) = get_type::get::<ExternType>(*ty) else {
      return false;
    };

    for &negation in &negations.order {
      let Some(nctv) = get_type::get::<ExternType>(negation) else {
        return false;
      };

      if !is_subclass_extern_type_extern_type(nctv, etv) {
        return false;
      }
    }

    for (other_ty, other_negations) in extern_types.iter() {
      if *other_ty == *ty {
        continue;
      }

      let Some(octv) = get_type::get::<ExternType>(*other_ty) else {
        return false;
      };

      if is_subclass_extern_type_extern_type(etv, octv) {
        let iss = |t: TypeId| -> bool {
          let c =
            get_type::get::<ExternType>(t).expect("order 元素已在上方循环逐个验证为 ExternType");
          is_subclass_extern_type_extern_type(etv, c)
        };

        if !other_negations.order.iter().any(|&t| iss(t)) {
          return false;
        }
      }
    }
  }

  true
}

fn are_normalized_functions(tys: &NormalizedFunctionType) -> bool {
  tys.parts.order.iter().all(|&ty| {
    get_type::get::<FunctionType>(ty).is_some() || get_type::get::<ErrorType>(ty).is_some()
  })
}

fn are_normalized_tables(tys: &TypeIds) -> bool {
  tys.order.iter().all(|&ty| {
    get_type::get::<TableType>(ty).is_some()
      || get_type::get::<MetatableType>(ty).is_some()
      || get_type::get::<PrimitiveType>(ty).is_some_and(|pt| pt.r#type == PrimitiveType::TABLE)
  })
}

fn is_normalized_tyvar(tyvars: &NormalizedTyvars) -> bool {
  tyvars.iter().all(|(tyvar, intersect)| {
    is_plain_tyvar(*tyvar)
      && is_shallow_inhabited(intersect)
      && intersect
        .tyvars
        .keys()
        .all(|other| tyvar_index(*other) > tyvar_index(*tyvar))
  })
}

pub(crate) fn assert_invariant(norm: &NormalizedType) {
  if !fflag::DebugLuauCheckNormalizeInvariant.get() {
    return;
  }

  LUAU_ASSERT!(is_normalized_top(norm.tops));
  LUAU_ASSERT!(is_normalized_boolean(norm.booleans));
  LUAU_ASSERT!(are_normalized_extern_types(&norm.extern_types));
  LUAU_ASSERT!(is_normalized_error(norm.errors));
  LUAU_ASSERT!(is_normalized_prim(norm.nils, PrimitiveType::NIL_TYPE));
  LUAU_ASSERT!(is_normalized_prim(norm.numbers, PrimitiveType::NUMBER));
  if fflag::LuauIntegerType2.get() {
    LUAU_ASSERT!(is_normalized_prim(norm.integers, PrimitiveType::INTEGER));
  }
  LUAU_ASSERT!(is_normalized_string(&norm.strings));
  LUAU_ASSERT!(is_normalized_prim(norm.threads, PrimitiveType::THREAD));
  LUAU_ASSERT!(is_normalized_prim(norm.buffers, PrimitiveType::BUFFER));
  LUAU_ASSERT!(are_normalized_functions(&norm.functions));
  LUAU_ASSERT!(are_normalized_tables(&norm.tables));
  LUAU_ASSERT!(is_normalized_tyvar(&norm.tyvars));
  for child in norm.tyvars.values() {
    assert_invariant(child);
  }
}
