use core::{mem, ptr};

use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::global::Global,
  records::{value_visitor::ValueVisitor, variable::Variable},
};

impl ValueVisitor {
  pub fn new(
    globals: &mut DenseHashMap<AstName, Global>,
    variables: &mut DenseHashMap<*mut AstLocal, Variable>,
    class_locals: &mut DenseHashMap<AstName, *mut AstLocal>,
  ) -> Self {
    let globals_owned = mem::replace(globals, DenseHashMap::new(AstName::new()));
    let variables_owned = mem::replace(variables, DenseHashMap::new(ptr::null_mut()));
    let class_locals_owned = mem::replace(class_locals, DenseHashMap::new(AstName::new()));
    ValueVisitor {
      globals: globals_owned,
      variables: variables_owned,
      class_locals: class_locals_owned,
    }
  }
}
