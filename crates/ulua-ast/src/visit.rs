//! AST visitor dispatch — the faithful Rust form of Luau's virtual
//! `AstNode::visit(AstVisitor*)` traversal (`Ast/src/Ast.cpp`).
//!
//! In C++ each concrete node overrides `visit(AstVisitor*)`: it calls the typed
//! `visitor->visit(this)` (compile-time overload on `this`'s static type) and,
//! if that returns `true`, recurses into its children with `child->visit(v)` —
//! a *virtual* call dispatched on the child's dynamic type.
//!
//! Rust has no vtable here (nodes are thin `*mut AstExpr` etc. in the arena), so
//! the per-node override becomes `impl AstVisitable for X`, and the virtual
//! recursion becomes a `class_index` match in the `*_visit` dispatch functions
//! below — the central analog of the C++ vtable. A node's `visit` body calls
//! `crate::visit::ast_expr_visit(self.child, v)` for each child pointer (and
//! loops over `AstArray` children), never `child.visit(v)` directly, because the
//! static type of `self.child` is only the base.

// `block->visit(visitor)` where the static type is already `AstStatBlock` —
// C++ calls the override directly (no virtual dispatch needed), so route to
// the node's own `visit` impl rather than the class-index dispatcher.
pub use crate::methods::ast_stat_block_visit::ast_stat_block_visit;
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
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion, ast_visitor::AstVisitor,
  },
  rtti::AstNodeClass,
};

/// C++ `AstX::visit(AstVisitor*)` override. Implemented once per concrete node
/// by that node's `visit` method item.
///
/// 参数是 `&mut self` 而非 `&self`：cpp 的 `visit(AstVisitor*)` 拿到的是非 const
/// `this`，observer（如 Analysis 的 TypeAttacher）会按 cpp 语义在 dispatch 期间
/// 写穿节点本身。取 `&self` 再 `as *const Self as *mut c_void` 交出去等于从共享
/// 借用造可变指针（`&self` 在 IR 里带 readonly/noalias，优化器可见的别名 UB），
/// 因此沿 arena 的可变性一路保持 `&mut`：调用方（`dispatch_node`）从 arena 裸指
/// 针取得独占借用，节点再把自己的 `*mut Self` 交给 visitor。
///
/// The visitor travels as generic `V: AstVisitor + ?Sized`: concrete visitors
/// monomorphize the whole recursion into static calls (no per-node vtable
/// round-trip), while `V = dyn AstVisitor` still compiles for callers that must
/// stay dynamically dispatched.
pub trait AstVisitable {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V);
}

/// `expr->visit(visitor)` where `expr` is a base `*mut AstExpr` — dispatch to the
/// concrete override by RTTI class index.
///
/// # Safety
/// `expr` must be null or point to a live `AstExpr`-prefixed node.
///
/// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
/// 的非 const 语义写穿节点，dispatch 内会取 `&mut`）。
pub unsafe fn ast_expr_visit<V: AstVisitor + ?Sized>(expr: *mut AstExpr, visitor: &mut V) {
  unsafe {
    dispatch_node(expr as *mut AstNode, visitor);
  }
}

/// `stat->visit(visitor)` for a base `*mut AstStat`.
///
/// # Safety
/// `stat` must be null or point to a live `AstStat`-prefixed node.
///
/// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
/// 的非 const 语义写穿节点，dispatch 内会取 `&mut`）。
pub unsafe fn ast_stat_visit<V: AstVisitor + ?Sized>(stat: *mut AstStat, visitor: &mut V) {
  unsafe {
    dispatch_node(stat as *mut AstNode, visitor);
  }
}

/// `ty->visit(visitor)` for a base `*mut AstType`.
///
/// # Safety
/// `ty` must be null or point to a live `AstType`-prefixed node.
///
/// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
/// 的非 const 语义写穿节点，dispatch 内会取 `&mut`）。
pub unsafe fn ast_type_visit<V: AstVisitor + ?Sized>(ty: *mut AstType, visitor: &mut V) {
  unsafe {
    dispatch_node(ty as *mut AstNode, visitor);
  }
}

/// `pack->visit(visitor)` for a base `*mut AstTypePack`.
///
/// # Safety
/// `pack` must be null or point to a live `AstTypePack`-prefixed node.
///
/// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
/// 的非 const 语义写穿节点，dispatch 内会取 `&mut`）。
pub(crate) unsafe fn ast_type_pack_visit<V: AstVisitor + ?Sized>(
  pack: *mut AstTypePack,
  visitor: &mut V,
) {
  unsafe {
    dispatch_node(pack as *mut AstNode, visitor);
  }
}

/// `node->visit(visitor)` for any base `*mut AstNode`.
///
/// # Safety
/// `node` must be null or point to a live AST node.
///
/// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
/// 的非 const 语义写穿节点，dispatch 内会取 `&mut`）。
pub unsafe fn ast_node_visit<V: AstVisitor + ?Sized>(node: *mut AstNode, visitor: &mut V) {
  unsafe {
    dispatch_node(node, visitor);
  }
}

/// The central class-index dispatcher — the analog of the C++ vtable. One arm
/// per concrete node type; each downcast is sound for the same reason as
/// `ast_node_as` (standard-layout, base at offset 0).
///
/// 关联常量 `T::CLASS_INDEX` 直接作 match 模式（i32 结构匹配合法）：编译器
/// 可将其优化为对 60 个分支的比较树/跳转表，而 guard 写法 `x if x == ..`
/// 强制逐臂线性求值，visit 每节点一次，热路径差距显著。
///
/// # Safety
/// `node` must be null or point to a live AST node.
///
/// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
/// 的非 const 语义写穿节点，dispatch 内会取 `&mut`）。
pub unsafe fn dispatch_node<V: AstVisitor + ?Sized>(node: *mut AstNode, visitor: &mut V) {
  unsafe {
    if node.is_null() {
      return;
    }
    match (*node).class_index {
      AstAttr::CLASS_INDEX => (&mut *(node as *mut AstAttr)).visit(visitor),
      AstExprBinary::CLASS_INDEX => (&mut *(node as *mut AstExprBinary)).visit(visitor),
      AstExprCall::CLASS_INDEX => (&mut *(node as *mut AstExprCall)).visit(visitor),
      AstExprConstantBool::CLASS_INDEX => (&mut *(node as *mut AstExprConstantBool)).visit(visitor),
      AstExprConstantInteger::CLASS_INDEX => {
        (&mut *(node as *mut AstExprConstantInteger)).visit(visitor)
      }
      AstExprConstantNil::CLASS_INDEX => (&mut *(node as *mut AstExprConstantNil)).visit(visitor),
      AstExprConstantNumber::CLASS_INDEX => {
        (&mut *(node as *mut AstExprConstantNumber)).visit(visitor)
      }
      AstExprConstantString::CLASS_INDEX => {
        (&mut *(node as *mut AstExprConstantString)).visit(visitor)
      }
      AstExprError::CLASS_INDEX => (&mut *(node as *mut AstExprError)).visit(visitor),
      AstExprFunction::CLASS_INDEX => (&mut *(node as *mut AstExprFunction)).visit(visitor),
      AstExprGlobal::CLASS_INDEX => (&mut *(node as *mut AstExprGlobal)).visit(visitor),
      AstExprGroup::CLASS_INDEX => (&mut *(node as *mut AstExprGroup)).visit(visitor),
      AstExprIfElse::CLASS_INDEX => (&mut *(node as *mut AstExprIfElse)).visit(visitor),
      AstExprIndexExpr::CLASS_INDEX => (&mut *(node as *mut AstExprIndexExpr)).visit(visitor),
      AstExprIndexName::CLASS_INDEX => (&mut *(node as *mut AstExprIndexName)).visit(visitor),
      AstExprInstantiate::CLASS_INDEX => (&mut *(node as *mut AstExprInstantiate)).visit(visitor),
      AstExprInterpString::CLASS_INDEX => (&mut *(node as *mut AstExprInterpString)).visit(visitor),
      AstExprLocal::CLASS_INDEX => (&mut *(node as *mut AstExprLocal)).visit(visitor),
      AstExprTable::CLASS_INDEX => (&mut *(node as *mut AstExprTable)).visit(visitor),
      AstExprTypeAssertion::CLASS_INDEX => {
        (&mut *(node as *mut AstExprTypeAssertion)).visit(visitor)
      }
      AstExprUnary::CLASS_INDEX => (&mut *(node as *mut AstExprUnary)).visit(visitor),
      AstExprVarargs::CLASS_INDEX => (&mut *(node as *mut AstExprVarargs)).visit(visitor),
      AstGenericType::CLASS_INDEX => (&mut *(node as *mut AstGenericType)).visit(visitor),
      AstGenericTypePack::CLASS_INDEX => (&mut *(node as *mut AstGenericTypePack)).visit(visitor),
      AstStatAssign::CLASS_INDEX => (&mut *(node as *mut AstStatAssign)).visit(visitor),
      AstStatBlock::CLASS_INDEX => (&mut *(node as *mut AstStatBlock)).visit(visitor),
      AstStatBreak::CLASS_INDEX => (&mut *(node as *mut AstStatBreak)).visit(visitor),
      AstStatClass::CLASS_INDEX => (&mut *(node as *mut AstStatClass)).visit(visitor),
      AstStatCompoundAssign::CLASS_INDEX => {
        (&mut *(node as *mut AstStatCompoundAssign)).visit(visitor)
      }
      AstStatContinue::CLASS_INDEX => (&mut *(node as *mut AstStatContinue)).visit(visitor),
      AstStatDeclareExternType::CLASS_INDEX => {
        (&mut *(node as *mut AstStatDeclareExternType)).visit(visitor)
      }
      AstStatDeclareFunction::CLASS_INDEX => {
        (&mut *(node as *mut AstStatDeclareFunction)).visit(visitor)
      }
      AstStatDeclareGlobal::CLASS_INDEX => {
        (&mut *(node as *mut AstStatDeclareGlobal)).visit(visitor)
      }
      AstStatError::CLASS_INDEX => (&mut *(node as *mut AstStatError)).visit(visitor),
      AstStatExpr::CLASS_INDEX => (&mut *(node as *mut AstStatExpr)).visit(visitor),
      AstStatFor::CLASS_INDEX => (&mut *(node as *mut AstStatFor)).visit(visitor),
      AstStatForIn::CLASS_INDEX => (&mut *(node as *mut AstStatForIn)).visit(visitor),
      AstStatFunction::CLASS_INDEX => (&mut *(node as *mut AstStatFunction)).visit(visitor),
      AstStatIf::CLASS_INDEX => (&mut *(node as *mut AstStatIf)).visit(visitor),
      AstStatLocal::CLASS_INDEX => (&mut *(node as *mut AstStatLocal)).visit(visitor),
      AstStatLocalFunction::CLASS_INDEX => {
        (&mut *(node as *mut AstStatLocalFunction)).visit(visitor)
      }
      AstStatRepeat::CLASS_INDEX => (&mut *(node as *mut AstStatRepeat)).visit(visitor),
      AstStatReturn::CLASS_INDEX => (&mut *(node as *mut AstStatReturn)).visit(visitor),
      AstStatTypeAlias::CLASS_INDEX => (&mut *(node as *mut AstStatTypeAlias)).visit(visitor),
      AstStatTypeFunction::CLASS_INDEX => (&mut *(node as *mut AstStatTypeFunction)).visit(visitor),
      AstStatWhile::CLASS_INDEX => (&mut *(node as *mut AstStatWhile)).visit(visitor),
      AstTypeError::CLASS_INDEX => (&mut *(node as *mut AstTypeError)).visit(visitor),
      AstTypeFunction::CLASS_INDEX => (&mut *(node as *mut AstTypeFunction)).visit(visitor),
      AstTypeGroup::CLASS_INDEX => (&mut *(node as *mut AstTypeGroup)).visit(visitor),
      AstTypeIntersection::CLASS_INDEX => (&mut *(node as *mut AstTypeIntersection)).visit(visitor),
      AstTypeOptional::CLASS_INDEX => (&mut *(node as *mut AstTypeOptional)).visit(visitor),
      AstTypePackExplicit::CLASS_INDEX => (&mut *(node as *mut AstTypePackExplicit)).visit(visitor),
      AstTypePackGeneric::CLASS_INDEX => (&mut *(node as *mut AstTypePackGeneric)).visit(visitor),
      AstTypePackVariadic::CLASS_INDEX => (&mut *(node as *mut AstTypePackVariadic)).visit(visitor),
      AstTypeReference::CLASS_INDEX => (&mut *(node as *mut AstTypeReference)).visit(visitor),
      AstTypeSingletonBool::CLASS_INDEX => {
        (&mut *(node as *mut AstTypeSingletonBool)).visit(visitor)
      }
      AstTypeSingletonString::CLASS_INDEX => {
        (&mut *(node as *mut AstTypeSingletonString)).visit(visitor)
      }
      AstTypeTable::CLASS_INDEX => (&mut *(node as *mut AstTypeTable)).visit(visitor),
      AstTypeTypeof::CLASS_INDEX => (&mut *(node as *mut AstTypeTypeof)).visit(visitor),
      AstTypeUnion::CLASS_INDEX => (&mut *(node as *mut AstTypeUnion)).visit(visitor),
      _ => {
        // C++ cannot reach here: every concrete AstNode subclass overrides
        // visit. An unknown class index means arena corruption.
        panic!(
          "dispatch_node: unknown AST class index {}",
          (*node).class_index
        );
      }
    }
  }
}
