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
      // Safety: 变体载荷由 parser/重水合从 arena 写入（见 `AstTypeOrPack` 的构造
      // 契约），指向存活节点。独占性**不是**由 `&'static AstType` 这个共享引用供给
      // ——`Node::from_ref(..).get_mut()` 在此只是把同一 arena 地址交回写穿门面，
      // 真正的独占前提仍是「dispatch 单线程独占整棵 arena」这条模块级纪律（见
      // `node_handle` 模块头）。与改写前的 `NonNull::from(ty).as_ptr()` + 裸指针门面
      // 逐位等价，仅少了就地 `unsafe {}` 外壳。
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
