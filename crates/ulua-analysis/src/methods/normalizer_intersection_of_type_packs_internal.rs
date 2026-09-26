//! `Normalizer::intersectionOfTypePacks_INTERNAL`（Normalize.cpp:3319 之后）
//! ——同形骨架见 [`crate::methods::normalizer_combine_type_packs`]。
use crate::{
  methods::normalizer_combine_type_packs::PackOp, records::normalizer::Normalizer,
  type_aliases::type_pack_id::TypePackId,
};

impl Normalizer {
  pub fn intersection_of_type_packs_internal(
    &mut self,
    here: TypePackId,
    there: TypePackId,
  ) -> Option<TypePackId> {
    self.combine_type_packs(PackOp::Meet, here, there)
  }
}
