use crate::{
  records::{
    ast_type::AstType, ast_type_error::AstTypeError, ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup, ast_type_intersection::AstTypeIntersection,
    ast_type_optional::AstTypeOptional, ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
  },
  rtti::define_ast_ref_enum,
};

define_ast_ref_enum! {
  /// 类型注解节点的只读引用判别枚举。
  ///
  /// 封装基于 RTTI `class_index` 的下转逻辑，使消费方可以通过安全的
  /// `match` 模式匹配具体类型注解，无需在每个分支手写 `unsafe { ast_node_as_unchecked }`。
  AstTypeRef<'a> : AstType ;
  try try_from_type , from from_type , expect "AstType class_index 必须为合法的类型注解节点类型" ;
  variants {
    Reference(AstTypeReference),
    Table(AstTypeTable),
    Function(AstTypeFunction),
    Typeof(AstTypeTypeof),
    Union(AstTypeUnion),
    Intersection(AstTypeIntersection),
    Group(AstTypeGroup),
    SingletonBool(AstTypeSingletonBool),
    SingletonString(AstTypeSingletonString),
    Optional(AstTypeOptional),
    Error(AstTypeError),
  }
}
