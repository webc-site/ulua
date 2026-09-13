use core::ffi::c_void;

use ulua_ast::{
  records::{ast_node::AstNode, ast_stat::AstStat, ast_stat_if::AstStatIf},
  rtti::ast_node_is,
  visit::ast_stat_visit,
};

use crate::{
  functions::{is_constant_false::is_constant_false, is_constant_true::is_constant_true},
  records::{cost::Cost, cost_visitor::CostVisitor},
};

impl CostVisitor {
  pub fn visit_ast_stat_if(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatIf);

      // C++ uses isConstantFalse/isConstantTrue (full truthiness over the constant
      // map), not getNumber — a folded boolean/string/nil condition must also prune
      // the dead branch. getNumber only recognised Number and so over-modelled
      // const-conditional bodies (e.g. `if a == 1 then ...`).
      if is_constant_false(&*self.constants, node.condition) {
        if !node.elsebody.is_null() {
          ast_stat_visit(node.elsebody, self);
        }
        return false;
      }

      if is_constant_true(&*self.constants, node.condition) {
        if !node.thenbody.is_null() {
          ast_stat_visit(node.thenbody as *mut AstStat, self);
        }
        return false;
      }

      // unconditional 'else' may require a jump after the 'if' body
      // note: this ignores cases when 'then' always terminates and also assumes comparison requires an extra instruction which may be false
      let mut else_is_if = false;
      if !node.elsebody.is_null() {
        else_is_if = ast_node_is::<AstStatIf>(&*(node.elsebody as *mut AstNode));
      }

      let discount = if !node.elsebody.is_null() && !else_is_if {
        1
      } else {
        0
      };
      // C++ `result += 1 + (elsebody && !elsebody->is<AstStatIf>())` via operator+=.
      self.result.add_assign(&Cost::new(1 + discount, 0));

      true
    }
  }
}
