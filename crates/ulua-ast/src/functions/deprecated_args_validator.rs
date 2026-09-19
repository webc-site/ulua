use alloc::{string::String, vec::Vec};

use ulua_common::functions::format::format;

use crate::{
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{AstExprTable, ItemKind},
    location::Location,
  },
  rtti::{ast_node_is, ast_node_try_as},
};

/// `@deprecated` 参数非字符串常量时的共用报错文案。
const UNKNOWN_ARG_TYPE: &str = "Unknown argument type for @deprecated";

pub fn deprecated_args_validator(
  attr_loc: Location,
  args: AstArray<*mut AstExpr>,
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

  // 只读遍历：元素裸指针在此一处收口为引用，其余逻辑全部走 &。
  let node = |p: *mut AstExpr| -> &AstExpr {
    // SAFETY: 参数由 parser 刚构造，指向 arena 存活节点，本函数不写。
    unsafe { &*p }
  };
  // SAFETY: 上方早退分支已保证 len == 1。
  let first_arg = node(unsafe { *args.as_slice().get_unchecked(0) });

  let Some(table) = ast_node_try_as::<AstExprTable>(&first_arg.base) else {
    return vec![(first_arg.base.location, String::from(UNKNOWN_ARG_TYPE))];
  };
  let mut errors = Vec::new();

  for item in table.items.iter() {
    if item.kind == ItemKind::Record {
      let key_expr = node(item.key);
      match ast_node_try_as::<AstExprConstantString>(&key_expr.base) {
        Some(key_str) => {
          // 零分配字节比较；原先无条件 `String::from_iter` 拼键再比，现在只在
          // 报错分支为消息各拼一次。
          let key_bytes = key_str.value.as_bytes();
          if key_bytes != b"use" && key_bytes != b"reason" {
            let key = String::from_iter(key_str.value.iter().map(|&c| c as u8 as char));
            errors.push((
              key_expr.base.location,
              format(format_args!(
                "Unknown argument '{}' for @deprecated. Only string constants for 'use' and 'reason' are allowed",
                key
              )),
            ));
          } else {
            let value_expr = node(item.value);
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
        node(item.value).base.location,
        String::from(
          "Only constants keys 'use' and 'reason' are allowed for @deprecated attribute",
        ),
      ));
    }
  }

  errors
}
