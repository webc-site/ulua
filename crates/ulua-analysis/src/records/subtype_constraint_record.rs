use crate::{enums::subtyping_variance::SubtypingVariance, type_aliases::type_id::TypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubtypeConstraintRecord {
  pub(crate) sub_ty: TypeId,
  pub(crate) super_ty: TypeId,
  pub(crate) variance: SubtypingVariance,
}
