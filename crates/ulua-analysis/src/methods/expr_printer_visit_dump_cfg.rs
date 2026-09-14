//! ExprPrinter 的表达式访问器，对齐 C++ `DumpCFG.cpp` 的 `ExprPrinter : AstVisitor`。

extern crate alloc;

use alloc::string::{String, ToString as _};
use core::slice::from_raw_parts;

use ulua_ast::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  functions::{
    to_string_ast::to_string as unary_op_to_string,
    to_string_ast_alt_b::to_string as binary_op_to_string,
  },
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_local::AstExprLocal,
    ast_expr_unary::AstExprUnary,
  },
  rtti::ast_node_is,
};

use crate::{
  functions::{dump_def::dump_def, get_local_name::get_local_name},
  records::expr_printer::ExprPrinter,
};

impl ExprPrinter {
  /// # Safety
  /// 调用方须保证 `node` 有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr(&mut self, node: *mut AstExpr) -> bool {
    unsafe {
      let base = &(*node).base;
      if ast_node_is::<AstExprLocal>(base) {
        self.visit_ast_expr_local(node as *mut AstExprLocal)
      } else if ast_node_is::<AstExprConstantNumber>(base) {
        self.visit_ast_expr_constant_number(node as *mut AstExprConstantNumber)
      } else if ast_node_is::<AstExprConstantString>(base) {
        self.visit_ast_expr_constant_string(node as *mut AstExprConstantString)
      } else if ast_node_is::<AstExprConstantBool>(base) {
        self.visit_ast_expr_constant_bool(node as *mut AstExprConstantBool)
      } else if ast_node_is::<AstExprConstantNil>(base) {
        self.visit_ast_expr_constant_nil(node as *mut AstExprConstantNil)
      } else if ast_node_is::<AstExprBinary>(base) {
        self.visit_ast_expr_binary(node as *mut AstExprBinary)
      } else if ast_node_is::<AstExprUnary>(base) {
        self.visit_ast_expr_unary(node as *mut AstExprUnary)
      } else {
        self.result.push_str("<expr>");
        false
      }
    }
  }

  unsafe fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    unsafe {
      if let Some(&def) = self.use_defs.get(&(node as *mut AstExpr)) {
        self.result.push_str(&dump_def(def));
      } else {
        self.result.push_str(&get_local_name((*node).local));
        self.result.push('?');
      }
    }
    false
  }

  unsafe fn visit_ast_expr_constant_number(&mut self, node: *mut AstExprConstantNumber) -> bool {
    unsafe {
      let node = &*node;
      // 对齐 C++：整数且精确解析时按 i64 输出，否则按 %f 六位小数
      if node.parse_result == ConstantNumberParseResult::Ok
        && node.value == node.value as i64 as f64
      {
        self.result.push_str(&(node.value as i64).to_string());
      } else {
        self.result.push_str(&format!("{:.6}", node.value));
      }
    }
    false
  }

  unsafe fn visit_ast_expr_constant_string(&mut self, node: *mut AstExprConstantString) -> bool {
    unsafe {
      let node = &*node;
      self.result.push('"');
      self
        .result
        .push_str(&String::from_utf8_lossy(from_raw_parts(
          node.value.data as *const u8,
          node.value.size,
        )));
      self.result.push('"');
    }
    false
  }

  unsafe fn visit_ast_expr_constant_bool(&mut self, node: *mut AstExprConstantBool) -> bool {
    self.result.push_str(if unsafe { (*node).value } {
      "true"
    } else {
      "false"
    });
    false
  }

  unsafe fn visit_ast_expr_constant_nil(&mut self, _: *mut AstExprConstantNil) -> bool {
    self.result.push_str("nil");
    false
  }

  unsafe fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
    unsafe {
      let node = &*node;
      self.visit_ast_expr(node.left);
      self.result.push(' ');
      self.result.push_str(&binary_op_to_string(node.op));
      self.result.push(' ');
      self.visit_ast_expr(node.right);
    }
    false
  }

  unsafe fn visit_ast_expr_unary(&mut self, node: *mut AstExprUnary) -> bool {
    unsafe {
      let node = &*node;
      self.result.push_str(&unary_op_to_string(node.op));
      self.visit_ast_expr(node.expr);
    }
    false
  }
}
