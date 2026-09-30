//! `normalized_function_type` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::records::normalized_function_type::NormalizedFunctionType;

impl NormalizedFunctionType {
  pub fn is_never(&self) -> bool {
    !self.is_top && self.parts.empty()
  }
}

impl NormalizedFunctionType {
  pub fn reset_to_never(&mut self) {
    self.is_top = false;
    self.parts.clear();
  }
}

impl NormalizedFunctionType {
  pub fn reset_to_top(&mut self) {
    self.is_top = true;
    self.parts.clear();
  }
}
