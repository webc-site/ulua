//! Node: `cxx:Method:Luau.Ast:Ast/src/Parser.cpp:934:Parser::validateAttribute`
//!
//! 校验 `@attr` 是否属于已知属性表（`kAttributeEntries` 加上由各自 FFlag
//! 门控的 `kDebugAttributeEntries`）：解析 `AstAttr::Type`、拒绝重复属性，
//! 并执行按属性的参数校验器（仅 `@deprecated` 有）。C++ 的静态表在此内联
//! 为 `match`，而非哨兵结尾数组。

use ulua_common::FFlag::DebugLuauNoInline;

use crate::{
  functions::deprecated_args_validator::deprecated_args_validator,
  records::{
    ast_array::AstArray,
    ast_attr::{AstAttr, AstAttrType},
    ast_expr::AstExpr,
    location::Location,
    parser::Parser,
    temp_vector::TempVector,
  },
};

impl Parser {
  pub(crate) fn validate_attribute(
    &mut self,
    loc: Location,
    attribute_name: &str,
    attributes: &TempVector<'_, *mut AstAttr>,
    args: &AstArray<*mut AstExpr>,
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
      for &attr_ptr in attributes.iter() {
        // SAFETY: attr_ptr 指向 arena 中存活的 AstAttr 节点。
        unsafe {
          if (*attr_ptr).r#type == attr_type {
            self.report(
              loc,
              format_args!("Cannot duplicate attribute '@{}'", attribute_name),
            );
          }
        }
      }

      if has_deprecated_validator {
        let errors_to_report = deprecated_args_validator(loc, *args);
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
pub fn parser_validate_attribute(
  parser: &mut Parser,
  loc: Location,
  attribute_name: &str,
  attributes: &TempVector<'_, *mut AstAttr>,
  args: &AstArray<*mut AstExpr>,
) -> Option<AstAttrType> {
  parser.validate_attribute(loc, attribute_name, attributes, args)
}
