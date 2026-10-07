//! Source: `Ast/src/Parser.cpp:934`
//!
//! 校验 `@attr` 是否属于已知属性表（`kAttributeEntries` 加上由各自 FFlag
//! 门控的 `kDebugAttributeEntries`）：解析 `AstAttr::Type`、拒绝重复属性，
//! 并执行按属性的参数校验器（仅 `@deprecated` 有）。C++ 的静态表在此内联
//! 为 `match`，而非哨兵结尾数组。

use ulua_common::fflag::DebugLuauNoInline;

use crate::{
  functions::deprecated_args_validator::deprecated_args_validator,
  records::{
    ast_attr::{AstAttr, AstAttrType},
    ast_expr::AstExpr,
    location::Location,
    node_handle::{Node, Nodes},
    parser::Parser,
    temp_vector::TempVector,
  },
};

impl Parser {
  pub(crate) fn validate_attribute(
    &mut self,
    loc: Location,
    attribute_name: &str,
    attributes: &TempVector<'_, Node<AstAttr>>,
    args: &Nodes<AstExpr>,
  ) -> Option<AstAttrType> {
    // kAttributeEntries (Parser.cpp): name -> (type, optional args validator).
    // Only "deprecated" carries a validator (deprecatedArgsValidator).
    let mut r#type: Option<AstAttrType> = None;
    let mut has_deprecated_validator = false;

    match attribute_name {
      "checked" => r#type = Some(AstAttrType::Checked),
      "native" => r#type = Some(AstAttrType::Native),
      "deprecated" => {
        r#type = Some(AstAttrType::Deprecated);
        has_deprecated_validator = true;
      }
      _ => {}
    }

    // kDebugAttributeEntries: FFlag-gated debug-only attributes.
    if r#type.is_none() && attribute_name == "debugnoinline" && DebugLuauNoInline.get() {
      r#type = Some(AstAttrType::DebugNoinline);
    }

    if let Some(attr_type) = r#type {
      // check that attribute is not duplicated
      for attr in attributes.iter() {
        // scratch_attr 窗口的元素是 arena 句柄（类型层恒非空），判重经 Deref 只读
        // `AstAttr::type`，无需解引用门面。
        if attr.r#type == attr_type {
          self.report(
            loc,
            format_args!("Cannot duplicate attribute '@{}'", attribute_name),
          );
        }
      }

      if has_deprecated_validator {
        let errors_to_report = deprecated_args_validator(loc, args);
        for (error_loc, msg) in errors_to_report {
          self.report(error_loc, format_args!("{}", msg));
        }
      }
    } else if attribute_name.is_empty() {
      self.report(loc, format_args!("Attribute name is missing"));
    } else {
      self.report(loc, format_args!("Invalid attribute '@{}'", attribute_name));
    }

    r#type
  }
}

// Free-function node surface delegating to the method.
