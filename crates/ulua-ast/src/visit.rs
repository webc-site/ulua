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
/// by that node's `visit` method item. `&self` is enough — `visit` never mutates
/// the node (it only feeds `self` to the visitor and recurses).
pub trait AstVisitable {
  fn visit(&self, visitor: &mut dyn AstVisitor);
}

/// `expr->visit(visitor)` where `expr` is a base `*mut AstExpr` — dispatch to the
/// concrete override by RTTI class index.
///
/// # Safety
/// `expr` must be null or point to a live `AstExpr`-prefixed node.
pub unsafe fn ast_expr_visit(expr: *mut AstExpr, visitor: &mut dyn AstVisitor) {
  unsafe {
    dispatch_node(expr as *mut AstNode, visitor);
  }
}

/// `stat->visit(visitor)` for a base `*mut AstStat`.
///
/// # Safety
/// `stat` must be null or point to a live `AstStat`-prefixed node.
pub unsafe fn ast_stat_visit(stat: *mut AstStat, visitor: &mut dyn AstVisitor) {
  unsafe {
    dispatch_node(stat as *mut AstNode, visitor);
  }
}

/// `ty->visit(visitor)` for a base `*mut AstType`.
///
/// # Safety
/// `ty` must be null or point to a live `AstType`-prefixed node.
pub unsafe fn ast_type_visit(ty: *mut AstType, visitor: &mut dyn AstVisitor) {
  unsafe {
    dispatch_node(ty as *mut AstNode, visitor);
  }
}

/// `pack->visit(visitor)` for a base `*mut AstTypePack`.
///
/// # Safety
/// `pack` must be null or point to a live `AstTypePack`-prefixed node.
pub(crate) unsafe fn ast_type_pack_visit(pack: *mut AstTypePack, visitor: &mut dyn AstVisitor) {
  unsafe {
    dispatch_node(pack as *mut AstNode, visitor);
  }
}

/// `node->visit(visitor)` for any base `*mut AstNode`.
///
/// # Safety
/// `node` must be null or point to a live AST node.
pub unsafe fn ast_node_visit(node: *mut AstNode, visitor: &mut dyn AstVisitor) {
  unsafe {
    dispatch_node(node, visitor);
  }
}

/// The central class-index dispatcher — the analog of the C++ vtable. One arm
/// per concrete node type; each downcast is sound for the same reason as
/// `ast_node_as` (standard-layout, base at offset 0).
///
/// # Safety
/// `node` must be null or point to a live AST node.
pub unsafe fn dispatch_node(node: *mut AstNode, visitor: &mut dyn AstVisitor) {
  unsafe {
    if node.is_null() {
      return;
    }
    match (*node).class_index {
      x if x == AstAttr::CLASS_INDEX => (&*(node as *const AstAttr)).visit(visitor),
      x if x == AstExprBinary::CLASS_INDEX => (&*(node as *const AstExprBinary)).visit(visitor),
      x if x == AstExprCall::CLASS_INDEX => (&*(node as *const AstExprCall)).visit(visitor),
      x if x == AstExprConstantBool::CLASS_INDEX => {
        (&*(node as *const AstExprConstantBool)).visit(visitor)
      }
      x if x == AstExprConstantInteger::CLASS_INDEX => {
        (&*(node as *const AstExprConstantInteger)).visit(visitor)
      }
      x if x == AstExprConstantNil::CLASS_INDEX => {
        (&*(node as *const AstExprConstantNil)).visit(visitor)
      }
      x if x == AstExprConstantNumber::CLASS_INDEX => {
        (&*(node as *const AstExprConstantNumber)).visit(visitor)
      }
      x if x == AstExprConstantString::CLASS_INDEX => {
        (&*(node as *const AstExprConstantString)).visit(visitor)
      }
      x if x == AstExprError::CLASS_INDEX => (&*(node as *const AstExprError)).visit(visitor),
      x if x == AstExprFunction::CLASS_INDEX => (&*(node as *const AstExprFunction)).visit(visitor),
      x if x == AstExprGlobal::CLASS_INDEX => (&*(node as *const AstExprGlobal)).visit(visitor),
      x if x == AstExprGroup::CLASS_INDEX => (&*(node as *const AstExprGroup)).visit(visitor),
      x if x == AstExprIfElse::CLASS_INDEX => (&*(node as *const AstExprIfElse)).visit(visitor),
      x if x == AstExprIndexExpr::CLASS_INDEX => {
        (&*(node as *const AstExprIndexExpr)).visit(visitor)
      }
      x if x == AstExprIndexName::CLASS_INDEX => {
        (&*(node as *const AstExprIndexName)).visit(visitor)
      }
      x if x == AstExprInstantiate::CLASS_INDEX => {
        (&*(node as *const AstExprInstantiate)).visit(visitor)
      }
      x if x == AstExprInterpString::CLASS_INDEX => {
        (&*(node as *const AstExprInterpString)).visit(visitor)
      }
      x if x == AstExprLocal::CLASS_INDEX => (&*(node as *const AstExprLocal)).visit(visitor),
      x if x == AstExprTable::CLASS_INDEX => (&*(node as *const AstExprTable)).visit(visitor),
      x if x == AstExprTypeAssertion::CLASS_INDEX => {
        (&*(node as *const AstExprTypeAssertion)).visit(visitor)
      }
      x if x == AstExprUnary::CLASS_INDEX => (&*(node as *const AstExprUnary)).visit(visitor),
      x if x == AstExprVarargs::CLASS_INDEX => (&*(node as *const AstExprVarargs)).visit(visitor),
      x if x == AstGenericType::CLASS_INDEX => (&*(node as *const AstGenericType)).visit(visitor),
      x if x == AstGenericTypePack::CLASS_INDEX => {
        (&*(node as *const AstGenericTypePack)).visit(visitor)
      }
      x if x == AstStatAssign::CLASS_INDEX => (&*(node as *const AstStatAssign)).visit(visitor),
      x if x == AstStatBlock::CLASS_INDEX => (&*(node as *const AstStatBlock)).visit(visitor),
      x if x == AstStatBreak::CLASS_INDEX => (&*(node as *const AstStatBreak)).visit(visitor),
      x if x == AstStatClass::CLASS_INDEX => (&*(node as *const AstStatClass)).visit(visitor),
      x if x == AstStatCompoundAssign::CLASS_INDEX => {
        (&*(node as *const AstStatCompoundAssign)).visit(visitor)
      }
      x if x == AstStatContinue::CLASS_INDEX => (&*(node as *const AstStatContinue)).visit(visitor),
      x if x == AstStatDeclareExternType::CLASS_INDEX => {
        (&*(node as *const AstStatDeclareExternType)).visit(visitor)
      }
      x if x == AstStatDeclareFunction::CLASS_INDEX => {
        (&*(node as *const AstStatDeclareFunction)).visit(visitor)
      }
      x if x == AstStatDeclareGlobal::CLASS_INDEX => {
        (&*(node as *const AstStatDeclareGlobal)).visit(visitor)
      }
      x if x == AstStatError::CLASS_INDEX => (&*(node as *const AstStatError)).visit(visitor),
      x if x == AstStatExpr::CLASS_INDEX => (&*(node as *const AstStatExpr)).visit(visitor),
      x if x == AstStatFor::CLASS_INDEX => (&*(node as *const AstStatFor)).visit(visitor),
      x if x == AstStatForIn::CLASS_INDEX => (&*(node as *const AstStatForIn)).visit(visitor),
      x if x == AstStatFunction::CLASS_INDEX => (&*(node as *const AstStatFunction)).visit(visitor),
      x if x == AstStatIf::CLASS_INDEX => (&*(node as *const AstStatIf)).visit(visitor),
      x if x == AstStatLocal::CLASS_INDEX => (&*(node as *const AstStatLocal)).visit(visitor),
      x if x == AstStatLocalFunction::CLASS_INDEX => {
        (&*(node as *const AstStatLocalFunction)).visit(visitor)
      }
      x if x == AstStatRepeat::CLASS_INDEX => (&*(node as *const AstStatRepeat)).visit(visitor),
      x if x == AstStatReturn::CLASS_INDEX => (&*(node as *const AstStatReturn)).visit(visitor),
      x if x == AstStatTypeAlias::CLASS_INDEX => {
        (&*(node as *const AstStatTypeAlias)).visit(visitor)
      }
      x if x == AstStatTypeFunction::CLASS_INDEX => {
        (&*(node as *const AstStatTypeFunction)).visit(visitor)
      }
      x if x == AstStatWhile::CLASS_INDEX => (&*(node as *const AstStatWhile)).visit(visitor),
      x if x == AstTypeError::CLASS_INDEX => (&*(node as *const AstTypeError)).visit(visitor),
      x if x == AstTypeFunction::CLASS_INDEX => (&*(node as *const AstTypeFunction)).visit(visitor),
      x if x == AstTypeGroup::CLASS_INDEX => (&*(node as *const AstTypeGroup)).visit(visitor),
      x if x == AstTypeIntersection::CLASS_INDEX => {
        (&*(node as *const AstTypeIntersection)).visit(visitor)
      }
      x if x == AstTypeOptional::CLASS_INDEX => (&*(node as *const AstTypeOptional)).visit(visitor),
      x if x == AstTypePackExplicit::CLASS_INDEX => {
        (&*(node as *const AstTypePackExplicit)).visit(visitor)
      }
      x if x == AstTypePackGeneric::CLASS_INDEX => {
        (&*(node as *const AstTypePackGeneric)).visit(visitor)
      }
      x if x == AstTypePackVariadic::CLASS_INDEX => {
        (&*(node as *const AstTypePackVariadic)).visit(visitor)
      }
      x if x == AstTypeReference::CLASS_INDEX => {
        (&*(node as *const AstTypeReference)).visit(visitor)
      }
      x if x == AstTypeSingletonBool::CLASS_INDEX => {
        (&*(node as *const AstTypeSingletonBool)).visit(visitor)
      }
      x if x == AstTypeSingletonString::CLASS_INDEX => {
        (&*(node as *const AstTypeSingletonString)).visit(visitor)
      }
      x if x == AstTypeTable::CLASS_INDEX => (&*(node as *const AstTypeTable)).visit(visitor),
      x if x == AstTypeTypeof::CLASS_INDEX => (&*(node as *const AstTypeTypeof)).visit(visitor),
      x if x == AstTypeUnion::CLASS_INDEX => (&*(node as *const AstTypeUnion)).visit(visitor),
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
