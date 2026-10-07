//! `multiple_nonviable_overloads` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::records::multiple_nonviable_overloads::MultipleNonviableOverloads;

impl MultipleNonviableOverloads {
  pub fn attempted_arg_count(&self) -> usize {
    self.attempted_arg_count
  }
}

impl MultipleNonviableOverloads {
  pub fn new(attempted_arg_count: usize) -> Self {
    Self {
      attempted_arg_count,
    }
  }
}
