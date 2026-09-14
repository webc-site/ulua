use core::ptr::{from_mut, null_mut};
use std::ptr::eq;

use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_noinline::LUAU_NOINLINE};

use crate::{
  functions::{follow_type::follow_type_id, get_mutable_type::get_mutable_type_id},
  records::{bound::Bound, r#type::Type},
  type_aliases::{bound_type::BoundType, type_id::TypeId, type_variant::TypeVariant},
};
LUAU_NOINLINE! {
    pub fn unifiable_bound_type_id_emplace_type_bound_type(
        ty: &mut Type,
        ty_arg: &mut TypeId,
    ) -> *mut Bound<TypeId> {

            LUAU_ASSERT!(!eq(ty, follow_type_id(*ty_arg)));
            // ty->ty.emplace<BoundType>(tyArg)
            ty.ty = TypeVariant::Bound(*ty_arg);
            get_mutable_type_id::<BoundType>(ty as *const Type as TypeId)
              .map_or(null_mut(), from_mut)

    }
}
