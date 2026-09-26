use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type, get_type},
  records::{
    free_type::FreeType, generic_type::GenericType, instantiation_2::Instantiation2,
    never_type::NeverType, unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Instantiation2 {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(!self.subtyping.is_null() && !self.scope.is_null());
    LUAU_ASSERT!(get_type::get::<GenericType>(ty).is_some());

    // SAFETY: generic_substitutions 在 Instantiation2 存活期内有效。
    let subst_ty = follow_type::follow(
      *self
        .generic_substitutions
        .find(&ty)
        .expect("TypeId not found in generic_substitutions"),
    );
    let ft = get_type::get::<FreeType>(subst_ty);
    LUAU_ASSERT!(ft.is_some());
    // C++ `LUAU_ASSERT(ft)` 必命中，紧邻断言蕴含 Some。
    let ft = ft.expect("紧邻 LUAU_ASSERT(ft.is_some()) 蕴含");

    let lower_bound = ft.lower_bound;
    let upper_bound = ft.upper_bound;

    let res = if get_type::get::<NeverType>(follow_type::follow(lower_bound)).is_some() {
      upper_bound
    } else if get_type::get::<UnknownType>(follow_type::follow(upper_bound)).is_some() {
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

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let res = self
      .generic_pack_substitutions
      .find(&tp)
      .expect("TypePackId not found in generic_pack_substitutions");
    LUAU_ASSERT!(!res.is_null());
    let cleaned = *res;
    self.base.dont_traverse_into_type_pack_id(cleaned);
    cleaned
  }
}
