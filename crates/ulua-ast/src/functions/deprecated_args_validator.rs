use alloc::{string::String, vec::Vec};

use ulua_common::functions::format::format;

use crate::{
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_node::AstNode,
    location::Location,
  },
  rtti::{ast_node_as, ast_node_is},
};

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

  let first_arg = unsafe { *args.as_slice().get_unchecked(0) };

  if !ast_node_is::<AstExprTable>(unsafe { &*(first_arg as *mut AstNode) }) {
    return vec![(
      unsafe { (*first_arg).base.location },
      String::from("Unknown argument type for @deprecated"),
    )];
  }

  let table = unsafe { ast_node_as::<AstExprTable>(first_arg as *mut AstNode) };
  let mut errors = Vec::new();

  for item in unsafe { (*table).items.iter() } {
    if item.kind == ItemKind::Record {
      let key_expr = unsafe { ast_node_as::<AstExprConstantString>(item.key as *mut AstNode) };
      let key_string_array = unsafe { (*key_expr).value };
      let key = String::from_iter(key_string_array.iter().map(|&c| c as u8 as char));

      if key != "use" && key != "reason" {
        errors.push((
                    unsafe { (*item.key).base.location },
                    format(format_args!(
                        "Unknown argument '{}' for @deprecated. Only string constants for 'use' and 'reason' are allowed",
                        key
                    )),
                ));
      } else if !ast_node_is::<AstExprConstantString>(unsafe { &*(item.value as *mut AstNode) }) {
        errors.push((
          unsafe { (*item.value).base.location },
          format(format_args!(
            "Only constant string allowed as value for '{}'",
            key
          )),
        ));
      }
    } else {
      errors.push((
        unsafe { (*item.value).base.location },
        String::from(
          "Only constants keys 'use' and 'reason' are allowed for @deprecated attribute",
        ),
      ));
    }
  }

  errors
}
