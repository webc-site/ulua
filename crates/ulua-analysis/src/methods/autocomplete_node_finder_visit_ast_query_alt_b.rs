use ulua_ast::records::{ast_node::AstNode, ast_stat::AstStat};

use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;

impl AutocompleteNodeFinder {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_stat(&mut self, stat: *mut AstStat) -> bool {
    let stat_ref = unsafe { &*stat };

    if stat_ref.base.location.begin < self.pos
      && if stat_ref.has_semicolon {
        self.pos < stat_ref.base.location.end
      } else {
        self.pos <= stat_ref.base.location.end
      }
    {
      self.ancestry.push(stat as *mut AstNode);
      return true;
    }

    false
  }
}
