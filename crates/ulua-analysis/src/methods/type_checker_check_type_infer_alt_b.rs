use alloc::string::String;

use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak, ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_continue::AstStatContinue,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile,
  },
  rtti::ast_node_try_as,
};
use ulua_common::fflag;

use crate::{
  enums::control_flow::ControlFlow,
  records::{generic_error::GenericError, type_checker::TypeChecker},
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData},
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat(&mut self, scope: &ScopePtr, program: &AstStat) -> ControlFlow {
    // AstStat 以 base 字段内嵌 AstNode（repr(C) 单继承），下转走安全 ast_node_try_as。
    let node = &program.base;

    if let Some(block) = ast_node_try_as::<AstStatBlock>(node) {
      return self.check_scope_ptr_ast_stat_block(scope, block);
    }

    if let Some(if_) = ast_node_try_as::<AstStatIf>(node) {
      return self.check_scope_ptr_ast_stat_if(scope, if_);
    }

    if let Some(while_) = ast_node_try_as::<AstStatWhile>(node) {
      return self.check_scope_ptr_ast_stat_while(scope, while_);
    }

    if let Some(repeat) = ast_node_try_as::<AstStatRepeat>(node) {
      return self.check_scope_ptr_ast_stat_repeat(scope, repeat);
    }

    if ast_node_try_as::<AstStatBreak>(node).is_some() {
      return ControlFlow::Breaks;
    }

    if ast_node_try_as::<AstStatContinue>(node).is_some() {
      return ControlFlow::Continues;
    }

    if let Some(return_) = ast_node_try_as::<AstStatReturn>(node) {
      return self.check_scope_ptr_ast_stat_return(scope, return_);
    }

    if let Some(expr) = ast_node_try_as::<AstStatExpr>(node) {
      // SAFETY: AstStatExpr.expr 由解析器保证非空（表达式语句必有表达式）。
      self.check_expr_pack(scope, unsafe { &*expr.expr });
      return ControlFlow::None;
    }

    if let Some(local) = ast_node_try_as::<AstStatLocal>(node) {
      return self.check_scope_ptr_ast_stat_local(scope, local);
    }

    if let Some(for_) = ast_node_try_as::<AstStatFor>(node) {
      return self.check_scope_ptr_ast_stat_for(scope, for_);
    }

    if let Some(for_in) = ast_node_try_as::<AstStatForIn>(node) {
      return self.check_scope_ptr_ast_stat_for_in(scope, for_in);
    }

    if let Some(assign) = ast_node_try_as::<AstStatAssign>(node) {
      return self.check_scope_ptr_ast_stat_assign(scope, assign);
    }

    if let Some(compound_assign) = ast_node_try_as::<AstStatCompoundAssign>(node) {
      return self.check_scope_ptr_ast_stat_compound_assign(scope, compound_assign);
    }

    if ast_node_try_as::<AstStatFunction>(node).is_some()
      || ast_node_try_as::<AstStatLocalFunction>(node).is_some()
    {
      self.ice_string_location(
        "Should not be calling two-argument check() on a function statement",
        &program.base.location,
      );
      return ControlFlow::None;
    }

    let typealias = ast_node_try_as::<AstStatTypeAlias>(node);

    if let Some(typealias) = typealias {
      return self.check_scope_ptr_ast_stat_type_alias(scope, typealias);
    }

    let typefunction = ast_node_try_as::<AstStatTypeFunction>(node);

    if let Some(typefunction) = typefunction {
      return self.check_scope_ptr_ast_stat_type_function(scope, typefunction);
    }

    let declare_global = ast_node_try_as::<AstStatDeclareGlobal>(node);

    if let Some(declare_global) = declare_global {
      return self.check_scope_ptr_ast_stat_declare_global(scope, declare_global);
    }

    let declare_function = ast_node_try_as::<AstStatDeclareFunction>(node);

    if let Some(declare_function) = declare_function {
      return self.check_scope_ptr_ast_stat_declare_function(scope, declare_function);
    }

    let declare_extern_type = ast_node_try_as::<AstStatDeclareExternType>(node);

    if let Some(declare_extern_type) = declare_extern_type {
      return self.check_scope_ptr_ast_stat_declare_extern_type(scope, declare_extern_type);
    }

    let error_statement = ast_node_try_as::<AstStatError>(node);

    if let Some(error_statement) = error_statement {
      return self.check_scope_ptr_ast_stat_error(scope, error_statement);
    }

    if fflag::DebugLuauUserDefinedClasses.get()
      && let Some(class_statement) = ast_node_try_as::<AstStatClass>(node)
    {
      // SAFETY: AstStatClass.name 由解析器保证非空（类声明必有名字节点）。
      self.report_error_location_type_error_data(
        unsafe { &(*class_statement.name).location },
        TypeErrorData::GenericError(GenericError::new(String::from(
          "class keyword is illegal here",
        ))),
      );
      return ControlFlow::None;
    }

    ControlFlow::None
  }
}
