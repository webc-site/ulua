//! Source: `Analysis/src/AstJsonEncoder.cpp:1174-1518` (hand-ported)
// The C++ encoder's 55 `bool visit(class AstX*) override`s. Each trait hook
// receives the type-erased node pointer the dispatcher hands out and forwards
// to the typed inherent method (the per-override graph item).
use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
  ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
  ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
  ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
  ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup,
  ast_expr_if_else::AstExprIfElse, ast_expr_index_expr::AstExprIndexExpr,
  ast_expr_index_name::AstExprIndexName, ast_expr_interp_string::AstExprInterpString,
  ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
  ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
  ast_expr_varargs::AstExprVarargs, ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock,
  ast_stat_break::AstStatBreak, ast_stat_compound_assign::AstStatCompoundAssign,
  ast_stat_continue::AstStatContinue, ast_stat_declare_extern_type::AstStatDeclareExternType,
  ast_stat_declare_function::AstStatDeclareFunction, ast_stat_declare_global::AstStatDeclareGlobal,
  ast_stat_error::AstStatError, ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor,
  ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf,
  ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
  ast_stat_repeat::AstStatRepeat, ast_stat_return::AstStatReturn,
  ast_stat_type_alias::AstStatTypeAlias, ast_stat_while::AstStatWhile,
  ast_type_error::AstTypeError, ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
  ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
  ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
  ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
  ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
  ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
  ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion, ast_visitor::AstVisitor,
};

use crate::records::ast_json_encoder::AstJsonEncoder;
impl AstVisitor for AstJsonEncoder {
  fn visit_type_group(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit_ast_type_group(node as *mut AstTypeGroup) }
  }

  fn visit_type_singleton_bool(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit_ast_type_singleton_bool(node as *mut AstTypeSingletonBool) }
  }

  fn visit_type_singleton_string(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit_ast_type_singleton_string(node as *mut AstTypeSingletonString) }
  }

  fn visit_expr_group(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_group(node as *mut AstExprGroup)
  }

  fn visit_expr_constant_nil(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_constant_nil(node as *mut AstExprConstantNil)
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

  fn visit_expr_if_else(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_if_else(node as *mut AstExprIfElse)
  }

  fn visit_expr_interp_string(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_interp_string(node as *mut AstExprInterpString)
  }

  fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_local(node as *mut AstExprLocal)
  }

  fn visit_expr_global(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_global(node as *mut AstExprGlobal)
  }

  fn visit_expr_varargs(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_varargs(node as *mut AstExprVarargs)
  }

  fn visit_expr_call(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_call(node as *mut AstExprCall)
  }

  fn visit_expr_index_name(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_index_name(node as *mut AstExprIndexName)
  }

  fn visit_expr_index_expr(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_index_expr(node as *mut AstExprIndexExpr)
  }

  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_function(node as *mut AstExprFunction)
  }

  fn visit_expr_table(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_table(node as *mut AstExprTable)
  }

  fn visit_expr_unary(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_unary(node as *mut AstExprUnary)
  }

  fn visit_expr_binary(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_binary(node as *mut AstExprBinary)
  }

  fn visit_expr_type_assertion(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_type_assertion(node as *mut AstExprTypeAssertion)
  }

  fn visit_expr_error(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_error(node as *mut AstExprError)
  }

  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_block(node as *mut AstStatBlock)
  }

  fn visit_stat_if(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_if(node as *mut AstStatIf)
  }

  fn visit_stat_while(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_while(node as *mut AstStatWhile)
  }

  fn visit_stat_repeat(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_repeat(node as *mut AstStatRepeat)
  }

  fn visit_stat_break(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_break(node as *mut AstStatBreak)
  }

  fn visit_stat_continue(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_continue(node as *mut AstStatContinue)
  }

  fn visit_stat_return(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_return(node as *mut AstStatReturn)
  }

  fn visit_stat_expr(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_expr(node as *mut AstStatExpr)
  }

  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local(node as *mut AstStatLocal)
  }

  fn visit_stat_for(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_for(node as *mut AstStatFor)
  }

  fn visit_stat_for_in(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_for_in(node as *mut AstStatForIn)
  }

  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_assign(node as *mut AstStatAssign)
  }

  fn visit_stat_compound_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_compound_assign(node as *mut AstStatCompoundAssign)
  }

  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_function(node as *mut AstStatFunction)
  }

  fn visit_stat_local_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local_function(node as *mut AstStatLocalFunction)
  }

  fn visit_stat_type_alias(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_type_alias(node as *mut AstStatTypeAlias)
  }

  fn visit_stat_declare_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_declare_function(node as *mut AstStatDeclareFunction)
  }

  fn visit_stat_declare_global(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_declare_global(node as *mut AstStatDeclareGlobal)
  }

  fn visit_stat_declare_extern_type(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_declare_extern_type(node as *mut AstStatDeclareExternType)
  }

  fn visit_stat_error(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_error(node as *mut AstStatError)
  }

  fn visit_type_reference(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_reference(node as *mut AstTypeReference)
  }

  fn visit_type_table(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_table(node as *mut AstTypeTable)
  }

  fn visit_type_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_function(node as *mut AstTypeFunction)
  }

  fn visit_type_typeof(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_typeof(node as *mut AstTypeTypeof)
  }

  fn visit_type_optional(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_optional(node as *mut AstTypeOptional)
  }

  fn visit_type_union(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_union(node as *mut AstTypeUnion)
  }

  fn visit_type_intersection(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_intersection(node as *mut AstTypeIntersection)
  }

  fn visit_type_error(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_error(node as *mut AstTypeError)
  }

  fn visit_type_pack(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_pack(node as *mut AstTypePack)
  }

  fn visit_type_pack_explicit(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_pack_explicit(node as *mut AstTypePackExplicit)
  }

  fn visit_type_pack_variadic(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_pack_variadic(node as *mut AstTypePackVariadic)
  }

  fn visit_type_pack_generic(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_pack_generic(node as *mut AstTypePackGeneric)
  }
}
