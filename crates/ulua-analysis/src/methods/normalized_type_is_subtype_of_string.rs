use crate::{enums::normalized_part::NormalizedPart, records::normalized_type::NormalizedType};

impl NormalizedType {
  pub fn is_subtype_of_string(&self) -> bool {
    // 同 `is_exactly_number`：`has_integers()` 自带 `LuauIntegerType2` 门控，
    // 原旗标双分支只差尾部一条 `!hasIntegers()`，此处坍缩为单条名单化判定。
    self.has_strings() && !self.has_parts_other_than(&[NormalizedPart::Strings])
  }
}
