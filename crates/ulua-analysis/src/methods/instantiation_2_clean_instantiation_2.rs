use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    free_type::FreeType, generic_type::GenericType, instantiation_2::Instantiation2,
    never_type::NeverType, unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl Instantiation2 {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(!self.subtyping.is_null() && !self.scope.is_null());
    LUAU_ASSERT!(get_type_id::<GenericType>(ty).is_some());

    // SAFETY: generic_substitutions 在 Instantiation2 存活期内有效。
    let subst_ty = follow_type_id(
      *self
        .generic_substitutions
        .find(&ty)
        .expect("TypeId not found in generic_substitutions"),
    );
    let ft = get_type_id::<FreeType>(subst_ty);
    LUAU_ASSERT!(ft.is_some());
    // C++ `LUAU_ASSERT(ft)` 必命中，此处 unwrap 安全。
    let ft = ft.unwrap();

    let lower_bound = ft.lower_bound;
    let upper_bound = ft.upper_bound;

    let res = if get_type_id::<NeverType>(follow_type_id(lower_bound)).is_some() {
      upper_bound
    } else if get_type_id::<UnknownType>(follow_type_id(upper_bound)).is_some() {
      lower_bound
    } else {
      // SAFETY: subtyping 在 Instantiation2 存活期内有效（上方已断言非空）。
      let r = unsafe {
        (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
          lower_bound,
          upper_bound,
          self.scope,
        )
      };
      if r.is_subtype {
        lower_bound
      } else {
        upper_bound
      }
    };

    self.base.dont_traverse_into_type_id(res);
    res
  }
}
