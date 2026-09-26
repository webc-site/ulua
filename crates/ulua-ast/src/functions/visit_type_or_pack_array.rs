use core::ptr::NonNull;

use crate::{
  records::{ast_array::AstArray, ast_type_or_pack::AstTypeOrPack, ast_visitor::AstVisitor},
  visit::{ast_type_pack_visit, ast_type_visit},
};

pub(crate) fn visit_type_or_pack_array<V: AstVisitor + ?Sized>(
  visitor: &mut V,
  array_of_type_or_pack: AstArray<AstTypeOrPack>,
) {
  for param in array_of_type_or_pack.as_slice() {
    match *param {
      // Safety: 变体载荷由 parser/重水合从 arena 写入（见 `AstTypeOrPack` 的构造契约），
      // 指向存活节点；数组元素来自类型实例化解析产物，arena 地址稳定，dispatch 单线程
      // 独占遍历。`as_ptr` 只把同一地址交回仍以裸指针为入参的 visit 门面，不改变借用关系。
      AstTypeOrPack::Type(ty) => unsafe {
        ast_type_visit(NonNull::from(ty).as_ptr(), visitor);
      },
      AstTypeOrPack::Pack(pack) => unsafe {
        ast_type_pack_visit(NonNull::from(pack).as_ptr(), visitor);
      },
      // cpp 在该形态下会对 null 的 `typePack` 解引用（`Ast.cpp:41`）；此处与迁移前的
      // 判空跳过同形：无节点可访问。
      AstTypeOrPack::Error => {}
    }
  }
}
