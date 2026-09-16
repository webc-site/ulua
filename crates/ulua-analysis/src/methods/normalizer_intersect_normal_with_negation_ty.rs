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

    // C++ 中归一化失败返回空指针：视为不可驻留，返回 False
    let Some(normal) = self.try_normalize(to_negate) else {
      return NormalizationResult::False;
    };
    let negated = self.negate_normal(&normal);

    match negated {
      Some(negated_type) => self.intersect_normals(intersect, &negated_type, 0),
      None => NormalizationResult::False,
    }
  }
}
