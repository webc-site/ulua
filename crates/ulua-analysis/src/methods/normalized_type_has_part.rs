use crate::{enums::normalized_part::NormalizedPart, records::normalized_type::NormalizedType};

impl NormalizedType {
  /// 单个部件的非空判定：把 [`NormalizedPart`] 派发到对应的 `has_x()` 访问器，
  /// 使名单化判定（[`NormalizedType::has_parts_other_than`]）与逐条书写等价。
  pub fn has_part(&self, part: NormalizedPart) -> bool {
    match part {
      NormalizedPart::Tops => self.has_tops(),
      NormalizedPart::Booleans => self.has_booleans(),
      NormalizedPart::ExternTypes => self.has_extern_types(),
      NormalizedPart::Errors => self.has_errors(),
      NormalizedPart::Nils => self.has_nils(),
      NormalizedPart::Numbers => self.has_numbers(),
      NormalizedPart::Integers => self.has_integers(),
      NormalizedPart::Strings => self.has_strings(),
      NormalizedPart::Threads => self.has_threads(),
      NormalizedPart::Buffers => self.has_buffers(),
      NormalizedPart::Tables => self.has_tables(),
      NormalizedPart::Functions => self.has_functions(),
      NormalizedPart::Tyvars => self.has_tyvars(),
    }
  }
}
