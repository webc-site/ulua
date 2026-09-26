//! Source: `Analysis/src/AstJsonEncoder.cpp:1174-1552` (hand-ported)
// C++ encoder's 55 `bool visit(class AstX*)` overrides. 旧端口以 55 个
// `visit_xxx(*mut ())` trait hook 直译，每个只做一次指针→具体类型
// 指针的转接；现在直接覆盖 `visit_any`，消费 `AstNodeRefMut` 的类型化
// `&mut 具体节点`，逐变体单态化为对 `visit_ast_xxx` 的直调：
// 零擦除、零转发函数，一次 match 完成分发。
use ulua_ast::{records::ast_visitor::AstVisitor, visit::AstNodeRefMut};

use crate::records::ast_json_encoder::AstJsonEncoder;

/// 展开整个 `visit_any` 的 match：
/// - `$variant => $method`：覆盖变体 → 类型化共享引用 → 固有 `visit_ast_xxx` 直调；
/// - `fallback { … }`：未覆盖变体（合并为 or-pattern）→ `$fb`。
///
/// 变体清单保持穷尽，枚举若新增变体此处即编译失败，不会静默漏分发。
macro_rules! encoder_visit_any {
  (
    $recv:tt, $node:tt,
    fallback { $($uncovered:ident,)* } => $fb:expr,
    $($variant:ident => $method:ident,)*
  ) => {
    match $node {
      $(AstNodeRefMut::$variant(n) => $recv.$method(&*n),)*
      $(AstNodeRefMut::$uncovered(_))|* => $fb,
    }
  };
}

impl AstVisitor for AstJsonEncoder {
  fn visit_any(&mut self, node: AstNodeRefMut<'_>) -> bool {
    // 固有 visit_ast_xxx 全部消费类型化共享引用（编码只读），无裸指针契约。
    encoder_visit_any! {
      self, node,
    // cpp AstJsonEncoder 未 override 的类（AstAttr / AstExprInstantiate /
    // AstGenericType / AstGenericTypePack / AstStatClass / AstStatTypeFunction）
    // 走 cpp AstVisitor 基类回退链（visit_xxx → … → visit_node → true），
    // 返回 true 即由 dispatch 继续下钻子节点。
    fallback {
      Attr,
      ExprInstantiate,
      GenericType,
      GenericTypePack,
      StatClass,
      StatTypeFunction,
    } => true,
    TypeGroup => visit_ast_type_group,
      TypeSingletonBool => visit_ast_type_singleton_bool,
      TypeSingletonString => visit_ast_type_singleton_string,
      ExprGroup => visit_ast_expr_group,
      ExprConstantNil => visit_ast_expr_constant_nil,
      ExprConstantBool => visit_ast_expr_constant_bool,
      ExprConstantNumber => visit_ast_expr_constant_number,
      ExprConstantInteger => visit_ast_expr_constant_integer,
      ExprConstantString => visit_ast_expr_constant_string,
      ExprIfElse => visit_ast_expr_if_else,
      ExprInterpString => visit_ast_expr_interp_string,
      ExprLocal => visit_ast_expr_local,
      ExprGlobal => visit_ast_expr_global,
      ExprVarargs => visit_ast_expr_varargs,
      ExprCall => visit_ast_expr_call,
      ExprIndexName => visit_ast_expr_index_name,
      ExprIndexExpr => visit_ast_expr_index_expr,
      ExprFunction => visit_ast_expr_function,
      ExprTable => visit_ast_expr_table,
      ExprUnary => visit_ast_expr_unary,
      ExprBinary => visit_ast_expr_binary,
      ExprTypeAssertion => visit_ast_expr_type_assertion,
      ExprError => visit_ast_expr_error,
      StatBlock => visit_ast_stat_block,
      StatIf => visit_ast_stat_if,
      StatWhile => visit_ast_stat_while,
      StatRepeat => visit_ast_stat_repeat,
      StatBreak => visit_ast_stat_break,
      StatContinue => visit_ast_stat_continue,
      StatReturn => visit_ast_stat_return,
      StatExpr => visit_ast_stat_expr,
      StatLocal => visit_ast_stat_local,
      StatFor => visit_ast_stat_for,
      StatForIn => visit_ast_stat_for_in,
      StatAssign => visit_ast_stat_assign,
      StatCompoundAssign => visit_ast_stat_compound_assign,
      StatFunction => visit_ast_stat_function,
      StatLocalFunction => visit_ast_stat_local_function,
      StatTypeAlias => visit_ast_stat_type_alias,
      StatDeclareFunction => visit_ast_stat_declare_function,
      StatDeclareGlobal => visit_ast_stat_declare_global,
      StatDeclareExternType => visit_ast_stat_declare_extern_type,
      StatError => visit_ast_stat_error,
      TypeReference => visit_ast_type_reference,
      TypeTable => visit_ast_type_table,
      TypeFunction => visit_ast_type_function,
      TypeTypeof => visit_ast_type_typeof,
      TypeOptional => visit_ast_type_optional,
      TypeUnion => visit_ast_type_union,
      TypeIntersection => visit_ast_type_intersection,
      TypeError => visit_ast_type_error,
      TypePackExplicit => visit_ast_type_pack_explicit,
      TypePackVariadic => visit_ast_type_pack_variadic,
      TypePackGeneric => visit_ast_type_pack_generic,
    }
  }
}
