use crate::{
  records::{
    ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
  },
  rtti::define_ast_ref_enum,
};

define_ast_ref_enum! {
  /// 类型包节点的只读引用判别枚举。
  ///
  /// 封装基于 RTTI `class_index` 的下转逻辑，使消费方可以通过安全的
  /// `match` 模式匹配具体类型包，无需在每个分支手写 `unsafe { ast_node_as_unchecked }`。
  AstTypePackRef<'a> : AstTypePack ;
  try try_from_pack , from from_pack , expect "AstTypePack class_index 必须为合法的类型包节点类型" ;
  variants {
    Explicit(AstTypePackExplicit),
    Variadic(AstTypePackVariadic),
    Generic(AstTypePackGeneric),
  }
}
