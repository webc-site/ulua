use crate::{
  records::{ast_stat_type_alias::AstStatTypeAlias, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_node_visit, ast_type_visit},
};

impl_visitable!(AstStatTypeAlias, StatTypeAlias, |this, visitor| {
  for &el in this.generics.iter() {
    // Safety: `generics` 元素是 parser 写入的存活 arena `AstType` 指针（非空、地址稳定），
    // `.cast()` 借 repr(C) 基址重合改节点类型位；`ast_node_visit` 只读遍历，无 `&mut` 别名。
    unsafe {
      ast_node_visit(el.cast(), visitor);
    }
  }

  for &el in this.generic_packs.iter() {
    // Safety: 同上，`generic_packs` 元素为 parser 保证的存活 arena 类型指针，cast 后只读遍历。
    unsafe {
      ast_node_visit(el.cast(), visitor);
    }
  }

  // Safety: `type_ptr` 是 parser 保证非空的别名右侧类型指针（arena 存活、地址稳定），
  // `ast_type_visit` 沿子指针只读遍历，不构造指向节点的 `&mut`。
  unsafe {
    ast_type_visit(this.type_ptr, visitor);
  }
});
