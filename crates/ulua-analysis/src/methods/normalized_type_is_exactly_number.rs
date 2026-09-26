use crate::{enums::normalized_part::NormalizedPart, records::normalized_type::NormalizedType};

impl NormalizedType {
  pub fn is_exactly_number(&self) -> bool {
    // `has_integers()` 自身按 `LuauIntegerType2` 门控（关旗标时恒 `false`），故原先的
    // 旗标双分支——差别仅是尾部多一条 `!hasIntegers()`——坍缩为单条名单化判定。
    self.has_numbers() && !self.has_parts_other_than(&[NormalizedPart::Numbers])
  }
}
