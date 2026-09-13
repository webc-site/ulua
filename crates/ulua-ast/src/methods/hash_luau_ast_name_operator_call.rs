use crate::records::{ast_name::AstName, hash_luau_ast_name::HashAstName};

impl HashAstName {
  pub fn operator_call(&self, value: &AstName) -> usize {
    let ptr = value.value as usize;
    (ptr >> 4) ^ (ptr >> 9)
  }
}
