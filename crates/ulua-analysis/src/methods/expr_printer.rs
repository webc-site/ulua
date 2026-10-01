//! `expr_printer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;
use core::fmt::Write as _;

use ulua_ast::{
  enums::{ast_expr_ref::AstExprRef, constant_number_parse_result::ConstantNumberParseResult},
  functions::to_string_ast::{to_str as unary_op_to_string, to_str_binary as binary_op_to_string},
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_local::AstExprLocal,
    ast_expr_unary::AstExprUnary,
  },
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{dump_def::dump_def, get_local_name::get_local_name},
  records::expr_printer::ExprPrinter,
  type_aliases::def_id_control_flow_graph::DefId,
};

impl ExprPrinter {
  pub fn new(use_defs: DenseHashMap<*mut AstExpr, DefId>) -> Self {
    Self {
      use_defs,
      result: String::new(),
    }
  }
}

// ExprPrinter 的表达式访问器，对齐 C++ `DumpCFG.cpp` 的 `ExprPrinter : AstVisitor`。

extern crate alloc;
impl ExprPrinter {
  pub fn visit_ast_expr(&mut self, node: &AstExpr) -> bool {
    match node.as_expr_ref() {
      AstExprRef::Local(local) => self.visit_ast_expr_local(local),
      AstExprRef::ConstantNumber(num) => self.visit_ast_expr_constant_number(num),
      AstExprRef::ConstantString(str_lit) => self.visit_ast_expr_constant_string(str_lit),
      AstExprRef::ConstantBool(b) => self.visit_ast_expr_constant_bool(b),
      AstExprRef::ConstantNil(_) => self.visit_ast_expr_constant_nil(),
      AstExprRef::Binary(bin) => self.visit_ast_expr_binary(bin),
      AstExprRef::Unary(un) => self.visit_ast_expr_unary(un),
      _ => {
        self.result.push_str("<expr>");
        false
      }
    }
  }

  fn visit_ast_expr_local(&mut self, node: &AstExprLocal) -> bool {
    let expr_ptr = (node as *const AstExprLocal).cast_mut().cast::<AstExpr>();
    if let Some(&def) = self.use_defs.get(&expr_ptr) {
      self.result.push_str(&dump_def(def));
    } else {
      self
        .result
        .push_str(&get_local_name(Some(node.local.get())));
      self.result.push('?');
    }
    false
  }

  fn visit_ast_expr_constant_number(&mut self, node: &AstExprConstantNumber) -> bool {
    // 对齐 C++：整数且精确解析时按 i64 输出，否则按 %f 六位小数
    if node.parse_result == ConstantNumberParseResult::Ok && node.value == node.value as i64 as f64
    {
      let _ = write!(self.result, "{}", node.value as i64);
    } else {
      let _ = write!(self.result, "{:.6}", node.value);
    }
    false
  }

  fn visit_ast_expr_constant_string(&mut self, node: &AstExprConstantString) -> bool {
    self.result.push('"');
    self
      .result
      .push_str(&String::from_utf8_lossy(node.value.as_bytes()));
    self.result.push('"');
    false
  }

  fn visit_ast_expr_constant_bool(&mut self, node: &AstExprConstantBool) -> bool {
    self
      .result
      .push_str(if node.value { "true" } else { "false" });
    false
  }

  fn visit_ast_expr_constant_nil(&mut self) -> bool {
    self.result.push_str("nil");
    false
  }

  fn visit_ast_expr_binary(&mut self, node: &AstExprBinary) -> bool {
    self.visit_ast_expr(node.left.get());
    self.result.push(' ');
    self.result.push_str(binary_op_to_string(node.op));
    self.result.push(' ');
    self.visit_ast_expr(node.right.get());
    false
  }

  fn visit_ast_expr_unary(&mut self, node: &AstExprUnary) -> bool {
    self.result.push_str(unary_op_to_string(node.op));
    self.visit_ast_expr(node.expr.get());
    false
  }
}
