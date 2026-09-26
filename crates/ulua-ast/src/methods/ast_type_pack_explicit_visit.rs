use crate::{
  records::{ast_type_pack_explicit::AstTypePackExplicit, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_pack_visit, ast_type_visit},
};

impl_visitable!(AstTypePackExplicit, TypePackExplicit, |this, visitor| {
  for &type_ptr in this.type_list.types.iter() {
    // Safety: type_list.types 元素是 parser 写入 arena 的类型槽（null 由 dispatch 短路）；存活节点随 self 的 arena 在列。
    unsafe {
      ast_type_visit(type_ptr, visitor);
    }
  }

  // Safety: tail_type 为 arena 存活 AstTypePack 节点或 null；dispatch 对 null 内部短路
  // （等价旧守卫），单线程独占遍历满足写穿前提。
  unsafe {
    ast_type_pack_visit(this.type_list.tail_type, visitor);
  }
});
