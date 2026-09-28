//! `incorrect_generic_parameter_count` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::records::{
  incorrect_generic_parameter_count::IncorrectGenericParameterCount, type_fun::TypeFun,
};

impl IncorrectGenericParameterCount {
  pub fn actual_parameters(&self) -> usize {
    self.actual_parameters
  }
}

impl IncorrectGenericParameterCount {
  pub fn name(&self) -> &str {
    &self.name
  }
}

impl IncorrectGenericParameterCount {
  pub fn type_fun(&self) -> &TypeFun {
    &self.type_fun
  }
}
