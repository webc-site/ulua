//! `ambiguous_function_call` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  records::ambiguous_function_call::AmbiguousFunctionCall,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl AmbiguousFunctionCall {
  pub fn arguments(&self) -> TypePackId {
    self.arguments
  }
}

impl AmbiguousFunctionCall {
  pub fn function(&self) -> TypeId {
    self.function
  }
}

impl AmbiguousFunctionCall {
  pub fn new(function: TypeId, arguments: TypePackId) -> Self {
    Self {
      function,
      arguments,
    }
  }
}
