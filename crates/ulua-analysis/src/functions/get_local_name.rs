extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_local::AstLocal;

/// # Safety
/// `local` 须指向本次 dump 期间存活的 parse-arena `AstLocal`（非空、对齐、地址稳定），全程只读
/// 取名，单线程。对应 C++ `static std::string getLocalName(AstLocal* local)`
/// (`cpp/Analysis/src/DumpCFG.cpp:16`)。
pub unsafe fn get_local_name(local: *mut AstLocal) -> String {
  // Safety: `(*local)` 解引用由 `&&` 短路确保仅在 `!local.is_null()` 成立时发生；`(*local).name`
  // 亦经 `!(*local).name.is_null()` 确认非空后再 `as_str_or_empty()`。`local` 为空时短路跳过整个
  // if、落入返回 "?" 的安全分支。单线程只读，无别名冲突。
  unsafe {
    if !local.is_null() && !(*local).name.is_null() {
      return (*local).name.as_str_or_empty().to_string();
    }
  }
  String::from("?")
}
