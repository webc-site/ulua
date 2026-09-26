//! `AstAttr::deprecatedInfo` (`Ast/src/Ast.cpp`).
//!
//! When the attribute is `@deprecated` and its first arg is a table literal,
//! pulls the `use`/`reason` string fields out of that table. `value` is a byte
//! array stored as `AstArray<char>`, so each string is rebuilt from the scalars'
//! low bytes.

use alloc::string::String;

use crate::{
  records::{
    ast_attr::{AstAttr, AstAttrType},
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::AstExprTable,
    deprecated_info::DeprecatedInfo,
  },
  rtti::{ast_node_try_as, ast_node_try_as_ptr},
};

impl AstAttr {
  pub fn deprecated_info(&self) -> DeprecatedInfo {
    let deprecated = self.r#type == AstAttrType::Deprecated;
    let mut info = DeprecatedInfo {
      deprecated,
      ..Default::default()
    };

    if info.deprecated && !self.args.is_empty() {
      // arg0 为 args 句柄数组首元素，parser 构造的属性实参已句柄化恒非空；
      // .get() 安全借用后走 RTTI 门面判型（#[repr(C)] 基址重合）。
      let arg0 = self.args.at(0).get();
      if let Some(table) = ast_node_try_as::<AstExprTable>(&arg0.base) {
        info.use_ = string_field(table, "use");
        info.reason = string_field(table, "reason");
      }
    }

    info
  }
}

/// Looks up `key` in the table and, when its value is a string literal, returns
/// it as a `String` (rebuilt from the byte-valued array; 批 2 后 value 为
/// `AstArray<u8>`，cpp `AstArray<char>` 同字节域).
fn string_field(table: &AstExprTable, key: &str) -> Option<String> {
  let s = unsafe {
    // Safety: get_record_str 返回的 value 是表项写入 arena 的存活节点或 null；try_as_ptr 判空+class index 命中后下转类型正确，仅共享读常量字符串字段。
    ast_node_try_as_ptr::<AstExprConstantString>(table.get_record_str(key)?)
  }?;
  Some(s.value.as_slice().iter().copied().map(char::from).collect())
}
