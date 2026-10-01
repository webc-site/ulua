use crate::{
  records::{ast_stat_type_alias::AstStatTypeAlias, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_node_visit, ast_type_visit_ref},
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

  // type_ptr 槽已句柄化：get_mut 沿 &mut self 交出独占子节点引用（parser 保证
  // 非空），引用门面沿子指针只读遍历，不构造指向节点的 `&mut`。
  ast_type_visit_ref(this.type_ptr.get_mut(), visitor);
});
