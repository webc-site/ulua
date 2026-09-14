//! Source: `Compiler/src/CostModel.cpp:103-419`

use core::ffi::c_void;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs, ast_local::AstLocal, ast_node::AstNode,
    ast_visitor::AstVisitor,
  },
  rtti::{ast_node_as, ast_node_is},
};
use ulua_common::{
  FFlag, enums::luau_builtin_function::LuauBuiltinFunction, macros::luau_assert::LUAU_ASSERT,
  records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::type_constant_folding::Type::Unknown,
  methods::cost_visitor_visit_cost_model_alt_b::visit_ast_stat_for,
  records::{constant::Constant, cost::Cost},
};

#[derive(Debug)]
pub struct CostVisitor {
  pub(crate) builtins: *const DenseHashMap<*mut AstExprCall, i32>,
  pub(crate) constants: *const DenseHashMap<*mut AstExpr, Constant>,
  pub(crate) vars: DenseHashMap<*mut AstLocal, u64>,
  pub(crate) result: Cost,
}

impl CostVisitor {
  pub fn model(&mut self, node: *mut AstExpr) -> Cost {
    if FFlag::LuauCompilePropagateTableProps2.get() && !FFlag::LuauCompileFoldOptimize.get() {
      if let Some(c) = unsafe { &*self.constants }.find(&node)
        && c.r#type != Unknown
      {
        return Cost::new(0, Cost::K_LITERAL);
      }
    } else if unsafe { &*self.constants }.find(&node).is_some() {
      return Cost::new(0, Cost::K_LITERAL);
    }

    let node_ptr = node as *mut AstNode;

    if let Some(expr) = unsafe { ast_node_as::<AstExprGroup>(node_ptr).as_mut() } {
      self.model(expr.expr)
    } else if ast_node_is::<AstExprConstantNil>(node_ptr)
      || ast_node_is::<AstExprConstantBool>(node_ptr)
      || ast_node_is::<AstExprConstantNumber>(node_ptr)
      || ast_node_is::<AstExprConstantString>(node_ptr)
      || ast_node_is::<AstExprConstantInteger>(node_ptr)
    {
      Cost::new(0, Cost::K_LITERAL)
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprLocal>(node_ptr).as_mut() } {
      let constant = self.vars.find(&expr.local).copied().unwrap_or(0);
      Cost::new(0, constant)
    } else if ast_node_is::<AstExprGlobal>(node_ptr) {
      Cost::new(1, 0)
    } else if ast_node_is::<AstExprVarargs>(node_ptr) {
      Cost::new(3, 0)
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprCall>(node_ptr).as_mut() } {
      let bfid = unsafe { &*self.builtins }.find(&(expr as *mut AstExprCall));
      let builtin = bfid.copied().unwrap_or(0) != LuauBuiltinFunction::LBF_NONE as i32;
      let builtin_short = builtin
        && expr.args.size
          <= if FFlag::LuauCompileFastcall3CostModel.get() {
            3
          } else {
            2
          };

      let mut cost = Cost::new(if builtin { 2 } else { 3 }, 0);

      if !builtin {
        cost = cost.add(&self.model(expr.func));
      }

      for arg in expr.args.iter() {
        let ac = self.model(*arg);
        let arg_cost = if ac.model == 0 && !builtin_short {
          Cost::new(1, 0)
        } else {
          ac
        };
        cost = cost.add(&arg_cost);
      }

      cost
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprIndexName>(node_ptr).as_mut() } {
      self.model(expr.expr).add(&Cost::new(1, 0))
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprIndexExpr>(node_ptr).as_mut() } {
      self
        .model(expr.expr)
        .add(&self.model(expr.index))
        .add(&Cost::new(1, 0))
    } else if ast_node_is::<AstExprFunction>(node_ptr) {
      Cost::new(10, 0)
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprTable>(node_ptr).as_mut() } {
      let mut cost = Cost::new(10, 0);

      for item in expr.items.iter() {
        if !item.key.is_null() {
          cost = cost.add(&self.model(item.key));
        }

        cost = cost.add(&self.model(item.value));
        cost = cost.add(&Cost::new(1, 0));
      }

      cost
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprUnary>(node_ptr).as_mut() } {
      Cost::fold(&self.model(expr.expr), &Cost::new(0, Cost::K_LITERAL))
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprBinary>(node_ptr).as_mut() } {
      Cost::fold(&self.model(expr.left), &self.model(expr.right))
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprTypeAssertion>(node_ptr).as_mut() } {
      self.model(expr.expr)
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprIfElse>(node_ptr).as_mut() } {
      self
        .model(expr.condition)
        .add(&self.model(expr.true_expr))
        .add(&self.model(expr.false_expr))
        .add(&Cost::new(2, 0))
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprInterpString>(node_ptr).as_mut() } {
      let mut cost = Cost::new(3, 0);
      for inner_expression in expr.expressions.iter() {
        cost = cost.add(&self.model(*inner_expression));
      }
      cost
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprInstantiate>(node_ptr).as_mut() } {
      self.model(expr.expr)
    } else {
      LUAU_ASSERT!(false, "Unknown expression type");
      Cost::default()
    }
  }
}

// The CostVisitor's accumulation lives in the inherent visit_ast_* methods (one
// per CostModel.cpp visit() override). The AstVisitor trait dispatch must delegate
// to them; returning false here (as the original stubs did) skipped ALL cost
// accumulation, so every function modelled to cost 0 and loops always unrolled.
// Statement kinds C++ doesn't override (return/expr/function/...) fall through to
// the trait defaults (descend), and their child expressions are charged via
// visit_expr -> visit_ast_expr -> model().
impl AstVisitor for CostVisitor {
  fn visit_expr(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr(node as *mut AstExpr)
  }

  fn visit_stat_for(&mut self, node: *mut c_void) -> bool {
    visit_ast_stat_for(self, node)
  }

  fn visit_stat_for_in(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_for_in(node)
  }

  fn visit_stat_while(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_while(node)
  }

  fn visit_stat_repeat(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_repeat(node)
  }

  fn visit_stat_if(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_if(node)
  }

  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local(node)
  }

  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_assign(node)
  }

  fn visit_stat_compound_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_compound_assign(node)
  }

  fn visit_stat_break(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_break(node)
  }

  fn visit_stat_continue(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_continue(node)
  }

  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_block(node)
  }
}
