use alloc::string::String;

use ulua_ast::records::ast_local::AstLocal;

/// `local` 为 parse-arena `AstLocal` 引用（可空，None 折叠为 "?"）。
/// 业务侧 safe 调用。对应 C++ `static std::string getLocalName(AstLocal* local)`
/// (`cpp/Analysis/src/DumpCFG.cpp:16`)。
pub fn get_local_name(local: Option<&AstLocal>) -> String {
  if let Some(local) = local
    && !local.name.is_null()
  {
    return local.name.as_str_or_empty().to_string();
  }
  String::from("?")
}
