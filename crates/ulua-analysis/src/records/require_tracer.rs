use alloc::{string::String, vec::Vec};
use core::{
  ffi::{CStr, c_void},
  ptr::null_mut,
};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
  ast_expr_local::AstExprLocal, ast_local::AstLocal, ast_node::AstNode,
  ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal, ast_visitor::AstVisitor,
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::{
  records::{
    file_resolver::FileResolver, module_info::ModuleInfo, require_trace_result::RequireTraceResult,
  },
  type_aliases::module_name_type::ModuleName,
};
impl DenseDefault for ModuleInfo {
  fn dense_default() -> Self {
    Self {
      name: String::new(),
      optional: false,
    }
  }
}

#[derive(Debug, Clone)]
pub struct RequireTracer {
  pub(crate) result: *mut RequireTraceResult,
  pub(crate) file_resolver: *mut FileResolver,
  pub(crate) current_module_name: ModuleName,
  pub(crate) locals: DenseHashMap<*mut AstLocal, *mut AstExpr>,
  pub(crate) work: Vec<*mut AstNode>,
  pub(crate) require_calls: Vec<*mut AstExprCall>,
}

impl AstVisitor for RequireTracer {
  fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_expr_type_assertion(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_expr_call(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let expr = node as *mut AstExprCall;
      if expr.is_null() {
        return true;
      }

      let global = (*(*expr).func).base.as_item_mut::<AstExprGlobal>();
      if !global.is_null() {
        let name_ptr = (*global).name.value;
        if !name_ptr.is_null() {
          let c_str = CStr::from_ptr(name_ptr).to_string_lossy();
          if c_str == "require" && (*expr).args.size >= 1 {
            self.require_calls.push(expr);
          }
        }
      }
      true
    }
  }

  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let stat = node as *mut AstStatLocal;
      if stat.is_null() {
        return true;
      }

      for i in 0..(*stat).vars.size.min((*stat).values.size) {
        let local = *(*stat).vars.data.add(i);
        let expr = *(*stat).values.data.add(i);
        *self.locals.get_or_insert(local) = expr;
      }
    }
    true
  }

  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let stat = node as *mut AstStatAssign;
      if stat.is_null() {
        return true;
      }

      for &v in (*stat).vars.as_slice() {
        let expr_local = (*v).base.as_item_mut::<AstExprLocal>();
        if !expr_local.is_null() {
          let local = (*expr_local).local;
          *self.locals.get_or_insert(local) = null_mut();
        }
      }
    }
    true
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    true
  }
}
