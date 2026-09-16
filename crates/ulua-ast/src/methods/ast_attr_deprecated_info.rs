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
    ast_node::AstNode,
    deprecated_info::DeprecatedInfo,
  },
  rtti::{ast_node_as, ast_node_is},
};

impl AstAttr {
  pub fn deprecated_info(&self) -> DeprecatedInfo {
    let deprecated = self.r#type == AstAttrType::Deprecated;
    let mut info = DeprecatedInfo {
      deprecated,
      ..Default::default()
    };

    if info.deprecated && !self.args.is_empty() {
      let arg0 = self.args.as_slice()[0];
      if !arg0.is_null() && ast_node_is::<AstExprTable>(unsafe { &*(arg0 as *mut AstNode) }) {
        let table = unsafe { &*ast_node_as::<AstExprTable>(arg0 as *mut AstNode) };
        info.use_ = string_field(table, "use");
        info.reason = string_field(table, "reason");
      }
    }

    info
  }
}

/// Looks up `key` in the table and, when its value is a string literal, returns
/// it as a `String` (rebuilt from the byte-valued `char` array).
fn string_field(table: &AstExprTable, key: &str) -> Option<String> {
  let value = table.get_record_str(key)?;
  if value.is_null() || !ast_node_is::<AstExprConstantString>(unsafe { &*(value as *mut AstNode) })
  {
    return None;
  }
  let s = unsafe { &*ast_node_as::<AstExprConstantString>(value as *mut AstNode) };
  Some(
    s.value
      .as_slice()
      .iter()
      .map(|&c| c as u8 as char)
      .collect(),
  )
}
