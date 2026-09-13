use core::ffi::c_void;

use ulua_ast::records::{ast_stat_block::AstStatBlock, ast_visitor::AstVisitor};
#[derive(Debug, Clone)]
pub struct NearestLikelyBlockFinder {
  pub stmt_block_recent_ast: *mut AstStatBlock,
  pub found: Option<*mut AstStatBlock>,
}

impl NearestLikelyBlockFinder {
  pub fn new(stmt_block_recent_ast: *mut AstStatBlock) -> Self {
    Self {
      stmt_block_recent_ast,
      found: None,
    }
  }
}

// C++ `struct NearestLikelyBlockFinder : public AstVisitor` with a single
// `bool visit(AstStatBlock* block)` override (FragmentAutocomplete.cpp). Driven
// via `stale->visit(&lsf)` so the full AST is traversed and every nested block
// is considered; all other node visits fall through to the default (recurse).
impl AstVisitor for NearestLikelyBlockFinder {
  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    let block = node as *mut AstStatBlock;
    unsafe {
      let block_location = (*block).base.base.location;
      let recent_location = (*self.stmt_block_recent_ast).base.base.location;
      if block_location.begin <= recent_location.begin {
        if let Some(found) = self.found {
          let found_location = (*found).base.base.location;
          if found_location.begin < block_location.begin {
            self.found = Some(block);
          }
        } else {
          self.found = Some(block);
        }
      }
    }
    true
  }
}
