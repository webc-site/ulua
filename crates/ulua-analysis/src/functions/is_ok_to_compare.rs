use crate::{
  enums::normalization_result::NormalizationResult,
  records::{normalized_type::NormalizedType, normalizer::Normalizer},
};

pub(crate) fn is_ok_to_compare(
  normalizer: &mut Normalizer,
  types_have_intersection: NormalizationResult,
  norm_left: Option<&NormalizedType>,
  norm_right: Option<&NormalizedType>,
) -> bool {
  if NormalizationResult::False != types_have_intersection {
    return true;
  }

  let (Some(norm_left), Some(norm_right)) = (norm_left, norm_right) else {
    return true;
  };

  if norm_left.is_nil() || norm_right.is_nil() {
    return true;
  }

  let inhabited_left = normalizer.is_inhabited_normalized_type(norm_left);
  let inhabited_right = normalizer.is_inhabited_normalized_type(norm_right);

  if NormalizationResult::True != inhabited_left || NormalizationResult::True != inhabited_right {
    return true;
  }

  if !norm_left.strings.is_never() && !norm_right.strings.is_never() {
    return true;
  }

  false
}
