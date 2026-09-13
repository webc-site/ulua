use crate::{
  enums::normalization_result::NormalizationResult,
  records::{normalized_type::NormalizedType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn intersect_normal_with_negation_ty(
    &mut self,
    to_negate: TypeId,
    intersect: &mut NormalizedType,
  ) -> NormalizationResult {
    self.consume_fuel();

    let normal = self.normalize(to_negate);
    let negated = self.negate_normal(&normal);

    match negated {
      Some(negated_type) => self.intersect_normals(intersect, &negated_type, 0),
      None => NormalizationResult::False,
    }
  }
}
