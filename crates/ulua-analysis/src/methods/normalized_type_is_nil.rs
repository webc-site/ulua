use crate::{enums::normalized_part::NormalizedPart, records::normalized_type::NormalizedType};

impl NormalizedType {
  pub fn is_nil(&self) -> bool {
    // 原判定链排除名单里没有 errors（含 error 的 nil 联合仍判为 nil）与 nils 自身，
    // 故二者进允许名单；`has_integers()` 自带旗标门控，尾部多一条即可。
    self.has_nils() && !self.has_parts_other_than(&[NormalizedPart::Nils, NormalizedPart::Errors])
  }
}
