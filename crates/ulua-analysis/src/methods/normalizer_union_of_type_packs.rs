//! `Normalizer::unionOfTypePacks`（Normalize.cpp:1796）——同形骨架见
//! [`crate::methods::normalizer_combine_type_packs`]。
use crate::{
  methods::normalizer_combine_type_packs::PackOp, records::normalizer::Normalizer,
  type_aliases::type_pack_id::TypePackId,
};

impl Normalizer {
  pub fn union_of_type_packs(&mut self, here: TypePackId, there: TypePackId) -> Option<TypePackId> {
    self.combine_type_packs(PackOp::Join, here, there)
  }
}
