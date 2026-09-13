use alloc::string::String;

use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_assign::AstStatAssign,
    ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak, ast_stat_class::AstStatClass,
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
  rtti::ast_node_as,
};
use ulua_common::FFlag;

use crate::{
  enums::control_flow::ControlFlow,
  records::{generic_error::GenericError, type_checker::TypeChecker},
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData},
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat(&mut self, scope: &ScopePtr, program: &AstStat) -> ControlFlow {
    let block = unsafe { ast_node_as::<AstStatBlock>(program as *const AstStat as *mut AstNode) };
    if !block.is_null() {
      return self.check_scope_ptr_ast_stat_block(scope, unsafe { &*block });
    }

    let if_ = unsafe { ast_node_as::<AstStatIf>(program as *const AstStat as *mut AstNode) };
    if !if_.is_null() {
      return self.check_scope_ptr_ast_stat_if(scope, unsafe { &*if_ });
    }

    let while_ = unsafe { ast_node_as::<AstStatWhile>(program as *const AstStat as *mut AstNode) };
    if !while_.is_null() {
      return self.check_scope_ptr_ast_stat_while(scope, unsafe { &*while_ });
    }

    let repeat = unsafe { ast_node_as::<AstStatRepeat>(program as *const AstStat as *mut AstNode) };
    if !repeat.is_null() {
      return self.check_scope_ptr_ast_stat_repeat(scope, unsafe { &*repeat });
    }

    let break_ = unsafe { ast_node_as::<AstStatBreak>(program as *const AstStat as *mut AstNode) };
    if !break_.is_null() {
      return ControlFlow::Breaks;
    }

    let continue_ =
      unsafe { ast_node_as::<AstStatContinue>(program as *const AstStat as *mut AstNode) };
    if !continue_.is_null() {
      return ControlFlow::Continues;
    }

    let return_ =
      unsafe { ast_node_as::<AstStatReturn>(program as *const AstStat as *mut AstNode) };
    if !return_.is_null() {
      return self.check_scope_ptr_ast_stat_return(scope, unsafe { &*return_ });
    }

    let expr = unsafe { ast_node_as::<AstStatExpr>(program as *const AstStat as *mut AstNode) };
    if !expr.is_null() {
      self.check_expr_pack(scope, unsafe { &*(*expr).expr });
      return ControlFlow::None;
    }

    let local = unsafe { ast_node_as::<AstStatLocal>(program as *const AstStat as *mut AstNode) };
    if !local.is_null() {
      return self.check_scope_ptr_ast_stat_local(scope, unsafe { &*local });
    }

    let for_ = unsafe { ast_node_as::<AstStatFor>(program as *const AstStat as *mut AstNode) };
    if !for_.is_null() {
      return self.check_scope_ptr_ast_stat_for(scope, unsafe { &*for_ });
    }

    let for_in = unsafe { ast_node_as::<AstStatForIn>(program as *const AstStat as *mut AstNode) };
    if !for_in.is_null() {
      return self.check_scope_ptr_ast_stat_for_in(scope, unsafe { &*for_in });
    }

    let assign = unsafe { ast_node_as::<AstStatAssign>(program as *const AstStat as *mut AstNode) };
    if !assign.is_null() {
      return self.check_scope_ptr_ast_stat_assign(scope, unsafe { &*assign });
    }

    let compound_assign =
      unsafe { ast_node_as::<AstStatCompoundAssign>(program as *const AstStat as *mut AstNode) };
    if !compound_assign.is_null() {
      return self.check_scope_ptr_ast_stat_compound_assign(scope, unsafe { &*compound_assign });
    }

    if !unsafe { ast_node_as::<AstStatFunction>(program as *const AstStat as *mut AstNode) }
      .is_null()
      || !unsafe { ast_node_as::<AstStatLocalFunction>(program as *const AstStat as *mut AstNode) }
        .is_null()
    {
      self.ice_string_location(
        "Should not be calling two-argument check() on a function statement",
        &program.base.location,
      );
      return ControlFlow::None;
    }

    let typealias =
      unsafe { ast_node_as::<AstStatTypeAlias>(program as *const AstStat as *mut AstNode) };

    if !typealias.is_null() {
      return self.check_scope_ptr_ast_stat_type_alias(scope, unsafe { &*typealias });
    }

    let typefunction =
      unsafe { ast_node_as::<AstStatTypeFunction>(program as *const AstStat as *mut AstNode) };

    if !typefunction.is_null() {
      return self.check_scope_ptr_ast_stat_type_function(scope, unsafe { &*typefunction });
    }

    let declare_global =
      unsafe { ast_node_as::<AstStatDeclareGlobal>(program as *const AstStat as *mut AstNode) };

    if !declare_global.is_null() {
      return self.check_scope_ptr_ast_stat_declare_global(scope, unsafe { &*declare_global });
    }

    let declare_function =
      unsafe { ast_node_as::<AstStatDeclareFunction>(program as *const AstStat as *mut AstNode) };

    if !declare_function.is_null() {
      return self.check_scope_ptr_ast_stat_declare_function(scope, unsafe { &*declare_function });
    }

    let declare_extern_type =
      unsafe { ast_node_as::<AstStatDeclareExternType>(program as *const AstStat as *mut AstNode) };

    if !declare_extern_type.is_null() {
      return self
        .check_scope_ptr_ast_stat_declare_extern_type(scope, unsafe { &*declare_extern_type });
    }

    let error_statement =
      unsafe { ast_node_as::<AstStatError>(program as *const AstStat as *mut AstNode) };

    if !error_statement.is_null() {
      return self.check_scope_ptr_ast_stat_error(scope, unsafe { &*error_statement });
    }

    let class_statement =
      unsafe { ast_node_as::<AstStatClass>(program as *const AstStat as *mut AstNode) };
    if FFlag::DebugLuauUserDefinedClasses.get() && !class_statement.is_null() {
      self.report_error_location_type_error_data(
        unsafe { &(*(*class_statement).name).location },
        TypeErrorData::GenericError(GenericError::new(String::from(
          "class keyword is illegal here",
        ))),
      );
      return ControlFlow::None;
    }

    ControlFlow::None
  }
}
