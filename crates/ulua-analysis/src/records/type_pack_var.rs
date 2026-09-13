use core::ptr::null_mut;

use crate::{
  records::{
    blocked_type_pack::BlockedTypePack, free_type_pack::FreeTypePack,
    generic_type_pack::GenericTypePack, type_arena::TypeArena,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack,
    type_pack_variant::TypePackVariant,
  },
};
#[derive(Debug, Clone)]
pub struct TypePackVar {
  pub(crate) ty: TypePackVariant,
  pub(crate) persistent: bool,
  pub(crate) owning_arena: *mut TypeArena,
}

/// `TypeArena::add_type_pack_t<T: Into<TypePackVar>>` is designed to accept a
/// bare type-pack variant (mirroring C++ `arena->addTypePack(VariadicTypePack{...})`).
/// These `From` impls wrap each variant struct into a non-persistent
/// `TypePackVar`, exactly as the C++ `TypePackVar(variant)` constructor does.
macro_rules! impl_from_variant_for_type_pack_var {
  ($variant:ident, $ty:path) => {
    impl From<$ty> for TypePackVar {
      fn from(v: $ty) -> Self {
        TypePackVar {
          ty: TypePackVariant::$variant(v),
          persistent: false,
          owning_arena: core::ptr::null_mut(),
        }
      }
    }
  };
}

impl_from_variant_for_type_pack_var!(Free, FreeTypePack);
impl_from_variant_for_type_pack_var!(Error, ErrorTypePack);
impl_from_variant_for_type_pack_var!(Generic, GenericTypePack);
impl_from_variant_for_type_pack_var!(TypePack, TypePack);
impl_from_variant_for_type_pack_var!(Variadic, VariadicTypePack);
impl_from_variant_for_type_pack_var!(Blocked, BlockedTypePack);
impl_from_variant_for_type_pack_var!(TypeFunctionInstance, TypeFunctionInstanceTypePack);

impl TypePackVar {
  pub fn new(ty: TypePackVariant) -> Self {
    Self {
      ty,
      persistent: false,
      owning_arena: null_mut(),
    }
  }

  pub fn new_with_persistence(ty: TypePackVariant, persistent: bool) -> Self {
    Self {
      ty,
      persistent,
      owning_arena: null_mut(),
    }
  }

  pub fn is_persistent(&self) -> bool {
    self.persistent
  }

  pub fn owning_arena(&self) -> *mut TypeArena {
    self.owning_arena
  }
}

impl From<BoundTypePack> for TypePackVar {
  fn from(v: BoundTypePack) -> Self {
    TypePackVar {
      ty: TypePackVariant::Bound(v.bound_to),
      persistent: false,
      owning_arena: null_mut(),
    }
  }
}
