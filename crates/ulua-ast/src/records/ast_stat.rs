//! Source: `Ast/include/Luau/Ast.h`

// Ast.h:263 — class AstStat : public AstNode { bool hasSemicolon; }
// Base-class embedding convention: concrete nodes hold `pub base: AstStat`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStat {
  pub base: AstNode,
  pub has_semicolon: bool,
}
use crate::records::ast_node::AstNode;
