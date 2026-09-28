//! 类型安全的 `AstVisitor`——cpp `Ast/include/Luau/Ast.h:1479` 的
//! `class AstVisitor`（virtual `visit(X*)` 重载族）的 Rust 形态。
//!
//! cpp 的每个重载都带具体节点类型参数，默认实现向下转发到父类重载
//! （`visit(AstExprConstantNil*) → visit(AstExpr*) → visit(AstNode*)`，
//! type/type-pack 家族例外地终止于 `false`）。本 trait 逐链保持该转发
//! 语义：hook 名、链拓扑与终值均与 cpp 一致，只是把 `X*` 参数换成
//! Rust 的 `&mut X`——由 [`AstVisitor::visit_any`] 从 [`AstNodeRefMut`]
//! 判别枚举解出，全程无裸指针、无 `c_void`。
//!
//! 节点全集与转发链只有一处事实来源：[`crate::visit::ast_node_table`] 的
//! (变体, 类型, hook, 父 hook) 表。本文件仅提供一个表回调
//! [`ast_visitor_hooks`]，一次生成 `visit_any` 的完整 match 与全部逐级
//! 默认 hook，杜绝与枚举/分发表在漂移。基类 hook（`visit_node` /
//! `visit_expr` / `visit_stat` / `visit_type` / `visit_type_pack`）是链的
//! 两端与两个 `false` 终点，手写保留可读性。
//!
//! 新 visitor 若不想逐类型覆盖，可直接覆盖 `visit_any` 一次拿到判别枚举；
//! 两条路径共用同一张表，行为一致。

/// `$ty` 在表宏（visit.rs）定义文本中给出，而 macro_rules 的路径不具卫生性，
/// 故展开点（本文件）须将全部节点类型导入作用域。
use crate::{
  records::{
    ast_attr::AstAttr, ast_expr::AstExpr, ast_expr_binary::AstExprBinary,
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_integer::AstExprConstantInteger, ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass, ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue, ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile, ast_type::AstType,
    ast_type_error::AstTypeError, ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
    ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
  },
  visit::AstNodeRefMut,
};

/// `ast_node_table` 的回调：由 (变体, 类型, hook, 父 hook) 行生成
/// `visit_any` 与各类型化默认 hook。
///
/// `#[repr(C)]` 首字段即父类子对象，`&mut node.base` 等价 cpp 的
/// `static_cast<Parent*>(node)`。`#[inline]`：V 单态化时整段 match 随变体
/// 折叠为直调，转发链在覆写点短路，零运行时开销。
macro_rules! ast_visitor_hooks {
  ($(($variant:ident, $ty:ty, $hook:ident, $parent:ident)),+ $(,)?) => {
    /// 类型化分发入口：`dispatch_node` 按 RTTI 构造 [`AstNodeRefMut`] 后调用。
    ///
    /// 默认实现把枚举解包转交给对应类型化 hook，转发链与 cpp 的
    /// `visit(X*)` 重载族逐级一致；覆盖本方法可跳过逐类型 hook，
    /// 一次拿到全部节点的判别视图。
    #[inline]
    fn visit_any(&mut self, node: AstNodeRefMut<'_>) -> bool {
      match node {
        $(AstNodeRefMut::$variant(n) => self.$hook(n)),+
      }
    }

    $(
      #[inline]
      fn $hook(&mut self, node: &mut $ty) -> bool {
        self.$parent(&mut node.base)
      }
    )+
  };
}

pub trait AstVisitor {
  /// cpp `visit(AstNode*)`：所有非 type/type-pack 链的终点，默认继续遍历。
  #[inline]
  fn visit_node(&mut self, _node: &mut AstNode) -> bool {
    true
  }

  ast_node_table!(ast_visitor_hooks);

  /// cpp `visit(AstExpr*)`。
  #[inline]
  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    self.visit_node(&mut node.base)
  }

  /// cpp `visit(AstStat*)`。
  #[inline]
  fn visit_stat(&mut self, node: &mut AstStat) -> bool {
    self.visit_node(&mut node.base)
  }

  /// cpp `visit(AstType*)`：默认 `false`，即类型注解子树不进入遍历，
  /// 除非下游显式覆写本 hook 或对应类型化 hook。
  #[inline]
  fn visit_type(&mut self, _node: &mut AstType) -> bool {
    false
  }

  /// cpp `visit(AstTypePack*)`：同 [`Self::visit_type`]，默认 `false`。
  #[inline]
  fn visit_type_pack(&mut self, _node: &mut AstTypePack) -> bool {
    false
  }
}
