//! Source: `Compiler/src/Types.cpp:253-951`

use alloc::{string::String, vec::Vec};
use core::ffi::{c_char, c_void};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
  ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
  ast_expr_constant_number::AstExprConstantNumber, ast_expr_constant_string::AstExprConstantString,
  ast_expr_function::AstExprFunction, ast_expr_group::AstExprGroup,
  ast_expr_if_else::AstExprIfElse, ast_expr_index_expr::AstExprIndexExpr,
  ast_expr_index_name::AstExprIndexName, ast_expr_interp_string::AstExprInterpString,
  ast_expr_local::AstExprLocal, ast_expr_type_assertion::AstExprTypeAssertion,
  ast_expr_unary::AstExprUnary, ast_local::AstLocal, ast_name::AstName,
  ast_stat_block::AstStatBlock, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
  ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
  ast_stat_repeat::AstStatRepeat, ast_stat_type_alias::AstStatTypeAlias, ast_type::AstType,
  ast_visitor::AstVisitor,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType, records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::global::Global, records::builtin_ast_types::BuiltinAstTypes,
  type_aliases::library_member_type_callback::LibraryMemberTypeCallback,
};

#[derive(Debug)]
pub struct TypeMapVisitorArgs<'a> {
  pub function_types: &'a mut DenseHashMap<*mut AstExprFunction, String>,
  pub local_types: &'a mut DenseHashMap<*mut AstLocal, LuauBytecodeType>,
  pub expr_types: &'a mut DenseHashMap<*mut AstExpr, LuauBytecodeType>,
  pub host_vector_type: *const c_char,
  pub userdata_types: &'a DenseHashMap<AstName, u8>,
  pub builtin_types: &'a BuiltinAstTypes,
  pub builtin_calls: &'a DenseHashMap<*mut AstExprCall, i32>,
  pub globals: &'a DenseHashMap<AstName, Global>,
  pub library_member_type_cb: LibraryMemberTypeCallback,
  pub bytecode: &'a mut BytecodeBuilder,
}

#[derive(Debug)]
pub struct TypeMapVisitor<'a> {
  pub(crate) function_types: &'a mut DenseHashMap<*mut AstExprFunction, String>,
  pub(crate) local_types: &'a mut DenseHashMap<*mut AstLocal, LuauBytecodeType>,
  pub(crate) expr_types: &'a mut DenseHashMap<*mut AstExpr, LuauBytecodeType>,
  pub(crate) host_vector_type: *const c_char,
  pub(crate) userdata_types: &'a DenseHashMap<AstName, u8>,
  pub(crate) builtin_types: &'a BuiltinAstTypes,
  pub(crate) builtin_calls: &'a DenseHashMap<*mut AstExprCall, i32>,
  pub(crate) globals: &'a DenseHashMap<AstName, Global>,
  pub(crate) library_member_type_cb: LibraryMemberTypeCallback,
  pub(crate) bytecode: &'a mut BytecodeBuilder,

  pub(crate) type_aliases: DenseHashMap<AstName, *mut AstStatTypeAlias>,
  pub(crate) type_alias_stack: Vec<(AstName, *mut AstStatTypeAlias)>,
  pub(crate) resolved_locals: DenseHashMap<*mut AstLocal, *const AstType>,
  pub(crate) resolved_exprs: DenseHashMap<*mut AstExpr, *const AstType>,
  pub(crate) function_return_types: DenseHashMap<*mut AstLocal, *const AstType>,
}

impl<'a> AstVisitor for TypeMapVisitor<'a> {
  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_block(node as *mut AstStatBlock)
  }

  fn visit_stat_repeat(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_repeat(node as *mut AstStatRepeat)
  }

  fn visit_stat_for(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_for(node as *mut AstStatFor)
  }

  fn visit_stat_for_in(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_for_in(node as *mut AstStatForIn)
  }

  fn visit_stat_local_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local_function(node as *mut AstStatLocalFunction)
  }

  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_function(node as *mut AstExprFunction)
  }

  fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_local(node as *mut AstExprLocal)
  }

  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local(node as *mut AstStatLocal)
  }

  fn visit_expr_index_expr(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_index_expr(node as *mut AstExprIndexExpr)
  }

  fn visit_expr_index_name(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_index_name(node as *mut AstExprIndexName)
  }

  fn visit_expr_unary(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_unary(node as *mut AstExprUnary)
  }

  fn visit_expr_binary(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_binary(node as *mut AstExprBinary)
  }

  fn visit_expr_group(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_group(node as *mut AstExprGroup)
  }

  fn visit_expr_type_assertion(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_type_assertion(node as *mut AstExprTypeAssertion)
  }

  fn visit_expr_constant_bool(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_constant_bool(node as *mut AstExprConstantBool)
  }

  fn visit_expr_constant_number(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_constant_number(node as *mut AstExprConstantNumber)
  }

  fn visit_expr_constant_integer(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_constant_integer(node as *mut AstExprConstantInteger)
  }

  fn visit_expr_constant_string(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_constant_string(node as *mut AstExprConstantString)
  }

  fn visit_expr_interp_string(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_interp_string(node as *mut AstExprInterpString)
  }

  fn visit_expr_if_else(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_if_else(node as *mut AstExprIfElse)
  }

  fn visit_expr_call(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_call(node as *mut AstExprCall)
  }
}
