use crate::{
  records::{
    ast_array::AstArray, ast_type_or_pack::AstTypeOrPack, ast_visitor::AstVisitor,
    node_handle::Node,
  },
  visit::{ast_type_pack_visit_ref, ast_type_visit_ref},
};

pub(crate) fn visit_type_or_pack_array<V: AstVisitor + ?Sized>(
  visitor: &mut V,
  array_of_type_or_pack: AstArray<AstTypeOrPack>,
) {
  for param in array_of_type_or_pack.as_slice() {
    match *param {
      // 变体载荷由 parser/重水合从 arena 写入（见 `AstTypeOrPack` 的构造契约），
      // 指向存活节点；经 [`Node`] 句柄边界取写穿视图后走引用门面，
      // 本调用点不再展开 `unsafe`。
      AstTypeOrPack::Type(ty) => {
        ast_type_visit_ref(Node::from_ref(ty).get_mut(), visitor);
      }
      AstTypeOrPack::Pack(pack) => {
        ast_type_pack_visit_ref(Node::from_ref(pack).get_mut(), visitor);
      }
      // cpp 在该形态下会对 null 的 `typePack` 解引用（`Ast.cpp:41`）；此处与迁移前的
      // 判空跳过同形：无节点可访问。
      AstTypeOrPack::Error => {}
    }
  }
}
