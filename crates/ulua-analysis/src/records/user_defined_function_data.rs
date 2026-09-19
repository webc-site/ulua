use alloc::{string::String, sync::Weak};
use core::ptr::null_mut;

use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{module::Module, type_fun::TypeFun},
  type_aliases::name_type::Name,
};
#[derive(Debug, Clone)]
pub struct UserDefinedFunctionData {
  /// Store a weak module reference to ensure the lifetime requirements are preserved
  pub(crate) owner: Weak<Module>,

  /// References to AST elements are owned by the Module allocator which also stores this type
  pub(crate) definition: *mut AstStatTypeFunction,

  pub(crate) environment_function: DenseHashMap<Name, (*mut AstStatTypeFunction, usize)>,
  pub(crate) environment_alias: DenseHashMap<Name, (*mut TypeFun, usize)>,
}

impl UserDefinedFunctionData {
  pub(crate) fn new(owner: Weak<Module>) -> Self {
    Self {
      owner,
      definition: null_mut(),
      environment_function: DenseHashMap::new(String::new()),
      environment_alias: DenseHashMap::new(String::new()),
    }
  }

  /// Creates an empty instance with no owning module (used when no owner is present).
  pub(crate) fn new_empty() -> Self {
    Self {
      owner: Weak::new(),
      definition: null_mut(),
      environment_function: DenseHashMap::new(String::new()),
      environment_alias: DenseHashMap::new(String::new()),
    }
  }
}
