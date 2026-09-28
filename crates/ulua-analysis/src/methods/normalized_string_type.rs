//! `normalized_string_type` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::records::normalized_string_type::NormalizedStringType;

impl NormalizedStringType {
  pub fn includes(&self, str: &str) -> bool {
    if self.is_string() || (self.is_union() && self.singletons.contains_key(str)) {
      true
    } else {
      self.is_intersection() && !self.singletons.contains_key(str)
    }
  }
}

impl NormalizedStringType {
  pub fn is_intersection(&self) -> bool {
    self.is_cofinite
  }
}

impl NormalizedStringType {
  pub fn is_never(&self) -> bool {
    !self.is_cofinite && self.singletons.is_empty()
  }
}

impl NormalizedStringType {
  pub fn is_string(&self) -> bool {
    self.is_cofinite && self.singletons.is_empty()
  }
}

impl NormalizedStringType {
  pub fn is_union(&self) -> bool {
    !self.is_cofinite
  }
}

impl NormalizedStringType {
  pub fn reset_to_never(&mut self) {
    self.is_cofinite = false;
    self.singletons.clear();
  }
}

pub fn normalized_string_type_reset_to_string(normalized_string_type: &mut NormalizedStringType) {
  normalized_string_type.is_cofinite = true;
  normalized_string_type.singletons.clear();
}
