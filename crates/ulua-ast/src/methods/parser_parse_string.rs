use core::{ffi::c_char, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{
    quote_style_ast::QuoteStyle::{QuotedRaw, QuotedSimple},
    quote_style_cst::QuoteStyle::QuotedDouble,
  },
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    ast_node::AstNode, cst_expr_constant_string::CstExprConstantString, cst_node::CstNode,
    lexeme::Type, location::Location, parser::Parser,
  },
  rtti::{AstNodeClass, CstNodeClass},
};

impl Parser {
  pub fn parse_string(&mut self) -> *mut AstExpr {
    let location: Location = self.lexer.current().location;

    let style = match self.lexer.current().r#type {
      Type::QUOTED_STRING | Type::INTERP_STRING_SIMPLE => QuotedSimple,
      Type::RAW_STRING => QuotedRaw,
      _ => {
        LUAU_ASSERT!(false);
        QuotedSimple
      }
    };

    let mut full_style = QuotedDouble;
    let mut block_depth: u32 = 0;

    if self.options.store_cst_data {
      let (fs, bd) = self.extract_string_details();
      full_style = fs;
      block_depth = bd;
    }

    let mut original_string = AstArray::<c_char> {
      data: null_mut(),
      size: 0,
    };

    if let Some(value) = self.parse_char_array(if self.options.store_cst_data {
      Some(&mut original_string)
    } else {
      None
    }) {
      let node = unsafe {
        (*self.allocator).alloc(AstExprConstantString {
          base: AstExpr {
            base: AstNode {
              class_index: AstExprConstantString::CLASS_INDEX,
              location,
            },
          },
          value,
          quote_style: style,
        })
      };

      if self.options.store_cst_data {
        let cst_node = unsafe {
          (*self.allocator).alloc(CstExprConstantString {
            base: CstNode {
              class_index: CstExprConstantString::CLASS_INDEX,
            },
            source_string: original_string,
            quote_style: full_style,
            block_depth,
          })
        };
        self
          .cst_node_map
          .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
      }

      node as *mut AstExpr
    } else {
      self.report_expr_error(
        location,
        AstArray {
          data: null_mut(),
          size: 0,
        },
        format_args!("String literal contains malformed escape sequence"),
      ) as *mut AstExpr
    }
  }
}
