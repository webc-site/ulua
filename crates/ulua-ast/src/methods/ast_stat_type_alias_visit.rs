use crate::{
  records::{ast_stat_type_alias::AstStatTypeAlias, ast_visitor::AstVisitor, node_handle::OptNode},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit_ref},
};

impl_visitable!(AstStatTypeAlias, StatTypeAlias, |this, visitor| {
  // generics/generic_packs 元素是 parser 写入的存活 arena 节点（非空、地址稳定）；
  // 静态类型已知，经 `OptNode` 句柄边界解引用后直接走 `AstVisitable::visit`
  // （cpp 虚分发的同一目标），调用点无 unsafe。
  for &el in this.generics.iter() {
    if let Some(generic) = OptNode::from_ptr(el).get_mut() {
      generic.visit(visitor);
    }
  }

  for &el in this.generic_packs.iter() {
    if let Some(pack) = OptNode::from_ptr(el).get_mut() {
      pack.visit(visitor);
    }
  }

  // type_ptr 槽已句柄化：get_mut 沿 &mut self 交出独占子节点引用（parser 保证
  // 非空），引用门面沿子指针只读遍历，不构造指向节点的 `&mut`。
  ast_type_visit_ref(this.type_ptr.get_mut(), visitor);
});
