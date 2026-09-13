use crate::records::subtyping_result::SubtypingResult;

impl SubtypingResult {
  pub fn negate(result: &SubtypingResult) -> SubtypingResult {
    SubtypingResult {
      is_subtype: !result.is_subtype,
      normalization_too_complex: result.normalization_too_complex,
      ..result.clone()
    }
  }
}
