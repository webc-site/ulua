use crate::records::{ast_name::AstName, ast_type::AstType, location::Location};

/// cpp `AstClassProperty`（`Ast/include/Luau/Ast.h:1142`）：类体内的属性声明。
///
/// `ty` 可空是真实语义——`class C public x end` 里 `x` 无标注（cpp `Parser.cpp:1580`
/// `AstType* propType = nullptr`，仅在有 `:` 时才 `parseType()`），且 cpp 用
/// `LUAU_ASSERT((bool)propType == (bool)typeColonLocation)`（`Parser.cpp:1607`）声明
/// 「类型与冒号位置同生同灭」。头文件里的 `= nullptr` 只是默认初始化；原 `Default`
/// 实现（null 哨兵 + 零跨度）全仓无调用点，已删除。
#[derive(Debug, Clone, Copy)]
pub struct AstClassProperty {
  pub qualifier_location: Location,
  pub name: AstName,
  pub name_location: Location,
  pub type_colon_location: Option<Location>,
  pub ty: *mut AstType,
}
