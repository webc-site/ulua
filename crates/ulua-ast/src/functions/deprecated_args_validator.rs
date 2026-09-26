use alloc::{string::String, vec::Vec};

use ulua_common::functions::format::format;

use crate::{
  functions::optional_node::slot_ref,
  records::{
    ast_expr::AstExpr,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{AstExprTable, ItemKind},
    location::Location,
    node_handle::Nodes,
  },
  rtti::{ast_node_is, ast_node_try_as},
};

/// `@deprecated` 参数非字符串常量时的共用报错文案。
const UNKNOWN_ARG_TYPE: &str = "Unknown argument type for @deprecated";

pub fn deprecated_args_validator(
  attr_loc: Location,
  args: &Nodes<AstExpr>,
) -> Vec<(Location, String)> {
  if args.is_empty() {
    return Vec::new();
  }

  if args.len() > 1 {
    return vec![(
      attr_loc,
      String::from("@deprecated can be parametrized only by 1 argument"),
    )];
  }

  // 上方早退分支已保证 len == 1；args 已句柄化恒非空，.get() 安全借用。
  let first_arg = args.at(0).get();

  let Some(table) = ast_node_try_as::<AstExprTable>(first_arg) else {
    return vec![(first_arg.base.location, String::from(UNKNOWN_ARG_TYPE))];
  };
  let mut errors = Vec::new();

  for item in table.items.iter() {
    if item.kind == ItemKind::Record {
      let key_expr = slot_ref(item.key);
      match ast_node_try_as::<AstExprConstantString>(key_expr) {
        Some(key_str) => {
          // 零分配字节比较；原先无条件 `String::from_iter` 拼键再比，现在只在
          // 报错分支为消息各拼一次。
          let key_bytes = key_str.value.as_bytes();
          if key_bytes != b"use" && key_bytes != b"reason" {
            // 与下方分支同一 `from_utf8_lossy` 口径：cpp 侧键是 `std::string`
            // 原样字节，逐字节 `as u8 as char` 的 Latin-1 还原会把多字节序列
            // 拆成假字符，两条消息对同一个键的呈现因此不一致。
            errors.push((
              key_expr.base.location,
              format(format_args!(
                "Unknown argument '{}' for @deprecated. Only string constants for 'use' and 'reason' are allowed",
                String::from_utf8_lossy(key_bytes)
              )),
            ));
          } else {
            let value_expr = slot_ref(item.value);
            if !ast_node_is::<AstExprConstantString>(value_expr) {
              errors.push((
                value_expr.base.location,
                format(format_args!(
                  "Only constant string allowed as value for '{}'",
                  String::from_utf8_lossy(key_bytes)
                )),
              ));
            }
          }
        }
        // parser 构造的 Record 键恒为字符串常量；非该类型（cpp 侧为空指针
        // 解引用 UB）收敛为报错分支。
        None => errors.push((key_expr.base.location, String::from(UNKNOWN_ARG_TYPE))),
      }
    } else {
      errors.push((
        slot_ref(item.value).base.location,
        String::from(
          "Only constants keys 'use' and 'reason' are allowed for @deprecated attribute",
        ),
      ));
    }
  }

  errors
}
