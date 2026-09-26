//! 测试夹具读取全局 scope 导出类型绑定的统一门面：经 `Arc<Scope>` 共享引用
//! 解引用走安全路径（(a) 类裸指针读取绕道的收口点）；写入绑定仍须
//! `raw_handle`（(b) 类句柄，见 `register_*_extern_type_fixture_types`）。

use ulua_analysis::{records::global_types::GlobalTypes, type_aliases::type_id::TypeId};

/// 取全局 scope `exported_type_bindings` 中 `name` 的类型 id；缺失即夹具
/// 契约违例，panic 报出具体名字。
pub fn exported_type(globals: &GlobalTypes, name: &str) -> TypeId {
  globals
    .global_scope()
    .exported_type_bindings
    .get(name)
    .unwrap_or_else(|| panic!("missing exported type binding: {name}"))
    .r#type()
}
