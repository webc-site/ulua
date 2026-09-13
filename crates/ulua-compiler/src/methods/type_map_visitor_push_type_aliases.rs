use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_type_alias::AstStatTypeAlias,
  },
  rtti::{ast_node_as, ast_node_is},
};

use crate::records::type_map_visitor::TypeMapVisitor;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn type_map_visitor_push_type_aliases<'a>(
  this: &mut TypeMapVisitor<'a>,
  block: *mut AstStatBlock,
) -> usize {
  let alias_stack_top = this.type_alias_stack.len();

  unsafe {
    let body = &(*block).body;

    for stat_ptr in body.as_slice() {
      let stat = &mut **stat_ptr;

      let stat_as_node = stat as *mut AstStat as *mut AstNode;

      if ast_node_is::<AstStatTypeAlias>(&*stat_as_node) {
        let alias_ptr = ast_node_as::<AstStatTypeAlias>(stat_as_node);
        if alias_ptr.is_null() {
          continue;
        }

        let alias_ref: &mut AstStatTypeAlias = &mut *alias_ptr;

        let prev_alias = if let Some(alias_ptr) = this.type_aliases.find(&alias_ref.name).copied() {
          alias_ptr
        } else {
          null_mut()
        };

        this.type_alias_stack.push((alias_ref.name, prev_alias));

        // C++ `prevAlias = alias` overwrites typeAliases[name]; try_insert was a
        // no-op when a same-named alias already existed (a nested redefinition),
        // leaving the outer alias in scope.
        *this.type_aliases.get_or_insert(alias_ref.name) = alias_ptr;
      }
    }
  }

  alias_stack_top
}
