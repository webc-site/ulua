use alloc::string::String;

use ulua_ast::records::ast_local::AstLocal;

use crate::records::arena_handle::alias_opt;

/// `local` 为 parse-arena `AstLocal` 句柄（cpp `AstLocal*` 原样指针身份，可空）；
/// 解引用收口 `alias_opt`，null 与无名折叠为 "?"，业务侧 safe 调用。
/// 对应 C++ `static std::string getLocalName(AstLocal* local)`
/// (`cpp/Analysis/src/DumpCFG.cpp:16`)。
pub fn get_local_name(local: *mut AstLocal) -> String {
  if let Some(local) = alias_opt(local)
    && !local.name.is_null()
  {
    return local.name.as_str_or_empty().to_string();
  }
  String::from("?")
}
