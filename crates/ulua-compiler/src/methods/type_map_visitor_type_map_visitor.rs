use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::ast_name::AstName;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::type_map_visitor::{TypeMapVisitor, TypeMapVisitorArgs};

impl<'a> TypeMapVisitor<'a> {
  pub fn new(args: TypeMapVisitorArgs<'a>) -> Self {
    Self {
      function_types: args.function_types,
      local_types: args.local_types,
      expr_types: args.expr_types,
      host_vector_type: args.host_vector_type,
      userdata_types: args.userdata_types,
      builtin_types: args.builtin_types,
      builtin_calls: args.builtin_calls,
      globals: args.globals,
      library_member_type_cb: args.library_member_type_cb,
      bytecode: args.bytecode,
      type_aliases: DenseHashMap::new(AstName::new()),
      type_alias_stack: Vec::new(),
      resolved_locals: DenseHashMap::new(null_mut()),
      resolved_exprs: DenseHashMap::new(null_mut()),
      function_return_types: DenseHashMap::new(null_mut()),
    }
  }
}
