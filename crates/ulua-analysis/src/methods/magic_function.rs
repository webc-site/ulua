//! `magic_function` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::records::{
  magic_function::MagicFunction, magic_function_type_check_context::MagicFunctionTypeCheckContext,
};

impl Drop for MagicFunction {
  fn drop(&mut self) {
    // virtual ~MagicFunction() {}
  }
}

impl MagicFunction {
  pub fn type_check(&self, _context: &MagicFunctionTypeCheckContext) -> bool {
    false
  }
}
