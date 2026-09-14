use crate::{
  records::substitution::Substitution,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// Resolves the C++ overload that `Substitution::replace(std::optional<Ty>)`
/// dispatches to via `replace(*ty)`: `replace(TypeId)` or `replace(TypePackId)`.
pub trait ReplaceInSubstitution: Sized {
  fn replace_in(self, subst: &mut Substitution) -> Self;
}

impl ReplaceInSubstitution for TypeId {
  fn replace_in(self, subst: &mut Substitution) -> Self {
    subst.replace_type_id(self)
  }
}

impl ReplaceInSubstitution for TypePackId {
  fn replace_in(self, subst: &mut Substitution) -> Self {
    subst.replace_type_pack_id(self)
  }
}

impl Substitution {
  pub fn replace_optional_ty<Ty: ReplaceInSubstitution>(&mut self, ty: Option<Ty>) -> Option<Ty> {
    ty.map(|inner| inner.replace_in(self))
  }
}
