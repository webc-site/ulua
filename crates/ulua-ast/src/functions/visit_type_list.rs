//! `visit_type_list` (`Ast/src/Ast.cpp`) — recurse a visitor over an `AstTypeList`
//! (its element types and the optional tail pack).

use crate::{
  records::{ast_type_list::AstTypeList, ast_visitor::AstVisitor},
  visit::{ast_type_pack_visit, ast_type_visit},
};

pub fn visit_type_list<V: AstVisitor + ?Sized>(visitor: &mut V, list: &AstTypeList) {
  for &ty in list.types.iter() {
    // Safety: 元素指针由 parser 写入 arena 的 AstArray，为 null 或存活 AstType 节点；
    // dispatch 对 null 内部短路（等价旧守卫），arena 地址稳定，单线程独占遍历。
    unsafe {
      ast_type_visit(ty, visitor);
    }
  }

  // Safety: tail_type 为 arena 存活 AstTypePack 节点或 null（可选子指针语义）；
  // dispatch 对 null 内部短路（等价旧守卫），借用与写穿前提同上。
  unsafe {
    ast_type_pack_visit(list.tail_type, visitor);
  }
}
