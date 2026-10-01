use alloc::string::String;
use core::mem::swap;

use ulua_ast::{
  records::{
    ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_string::AstExprConstantString, ast_expr_global::AstExprGlobal,
    ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  enums::type_kind::TypeKind,
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintUnknownType<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintUnknownType<'ctx> {
  fn visit_expr_binary(&mut self, node: &mut AstExprBinary) -> bool {
    self.visit(node)
  }
}

// —— 原 methods/lint_unknown_type_get_type_kind.rs ——
impl<'ctx> LintUnknownType<'ctx> {
  pub fn get_type_kind(&mut self, name: &str) -> TypeKind {
    match name {
      "nil" | "boolean" | "userdata" | "number" | "string" | "table" | "function" | "thread"
      | "buffer" | "vector" => TypeKind::Primitive,
      _ => {
        let context = self.context.get();
        if context.scope.lookup_type(name).is_some() {
          TypeKind::Userdata
        } else {
          TypeKind::Unknown
        }
      }
    }
  }
}

// —— 原 methods/lint_unknown_type_process.rs ——
impl<'ctx> LintUnknownType<'ctx> {
  lint_stat_process!(LintUnknownType);
}

// —— 原 methods/lint_unknown_type_validate_type.rs ——
impl<'ctx> LintUnknownType<'ctx> {
  /// cpp `validateType(AstExprConstantString*)`：`expr` 为遍历期存活的字面量节点借用，
  /// 本方法只读其 `value`/`location`，告警经 `self.context` 写句柄发出。
  pub fn validate_type(
    &mut self,
    expr: &AstExprConstantString,
    expected: &[TypeKind],
    expected_string: &str,
  ) {
    let name_bytes = expr.value.as_bytes();
    let name = String::from_utf8_lossy(name_bytes);
    let kind = self.get_type_kind(&name);
    if kind == TypeKind::Unknown {
      // 直传 `format_args!`（操作数为存活局部借用）：免中间 String 物化（2 alloc → 1）。
      emit_warning(
        self.context.get(),
        Code::UnknownType,
        expr.base.base.location,
        format_args!("Unknown type '{}'", name),
      );
      return;
    }
    for &ek in expected {
      if kind == ek {
        return;
      }
    }
    emit_warning(
      self.context.get(),
      Code::UnknownType,
      expr.base.base.location,
      format_args!("Unknown type '{}' (expected {})", name, expected_string),
    );
  }
}

// —— 原 methods/lint_unknown_type_visit.rs ——
impl<'ctx> LintUnknownType<'ctx> {
  /// cpp `LintUnknownType::visit(AstExprBinary*)`：`node` 为遍历期存活、由 arena 持有的
  /// 二元表达式共享借用（cpp 裸指针形参的 Rust 对应），本方法只读其 `op`/`left`/`right`。
  pub fn visit(&mut self, node: &AstExprBinary) -> bool {
    if !node.op.is_equality() {
      return true;
    }
    let mut lhs = node.left;
    let mut rhs = node.right;
    // Ensure rhs is the constant string argument
    // lhs/rhs 槽位已句柄化为 `Node<AstExpr>`：判型走句柄上生命周期正确的
    // 安全门面 `is`/`try_as`（类位不命中折叠为 false/None，即 cpp `->as<T>()`
    // 两段式），借用半径由各局部句柄供给，不再锻造假 'static。
    if !rhs.is::<AstExprConstantString>() {
      swap(&mut lhs, &mut rhs);
    }
    let Some(call) = lhs.try_as::<AstExprCall>() else {
      return true;
    };
    let Some(arg) = rhs.try_as::<AstExprConstantString>() else {
      return true;
    };
    // `call.func` 同为 parser 必建的非空子表达式槽位（仍是裸指针，经
    // `OptNode::from_ptr` 折叠），判型逻辑同上。
    let func_node = OptNode::from_ptr(call.func);
    let Some(g) = func_node.try_as::<AstExprGlobal>() else {
      return true;
    };
    if g.name.as_bytes() == b"type" {
      self.validate_type(arg, &[TypeKind::Primitive], "primitive type");
    } else if g.name.as_bytes() == b"typeof" {
      self.validate_type(
        arg,
        &[TypeKind::Primitive, TypeKind::Userdata],
        "primitive or userdata type",
      );
    }
    true
  }
}
