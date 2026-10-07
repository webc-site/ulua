use crate::{
  enums::ast_table_access::AstTableAccess,
  records::{ast_name::AstName, ast_type::AstType, location::Location},
};

/// cpp `AstDeclaredExternTypeProperty`（`Ast/include/Luau/Ast.h:1132`）：
/// `declare` 外部类型的一条属性/方法签名。
///
/// 头文件里 `AstType* ty = nullptr` 只是 C++ 的防御性默认初始化；四个实际构造点
/// （`Parser.cpp:1783` 报错产物、`1806` 方法类型、`1963`/`2019` 属性类型）都无条件写入
/// `parseType()` / `reportTypeError()` 的产物，槽位永不为空。原 `Default`（null 哨兵）
/// 全仓无调用点，已删除。
#[derive(Debug, Clone, Copy)]
pub struct AstDeclaredExternTypeProperty {
  pub name: AstName,
  pub name_location: Location,
  pub ty: *mut AstType,
  pub is_method: bool,
  pub location: Location,
  pub access: AstTableAccess,
}
