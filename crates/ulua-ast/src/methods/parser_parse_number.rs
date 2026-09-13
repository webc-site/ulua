use core::{slice::from_raw_parts, str::from_utf8};

use ulua_common::FFlag;

use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  functions::{parse_double::parse_double, parse_integer_64::parse_integer_64},
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_number::AstExprConstantNumber, ast_node::AstNode,
    cst_expr_constant_integer::CstExprConstantInteger,
    cst_expr_constant_number::CstExprConstantNumber, cst_node::CstNode, parser::Parser,
  },
  rtti::{AstNodeClass, CstNodeClass},
};

impl Parser {
  pub fn parse_number(&mut self) -> *mut AstExpr {
    // scratchData.assign(lexer.current().data, lexer.current().getLength());
    let (start, data_ptr, lexeme_len) = {
      let current = self.lexer.current();
      (
        current.location,
        unsafe { current.data.data } as *const u8,
        current.get_length() as usize,
      )
    };
    self.scratch_data.clear();
    let bytes = unsafe { from_raw_parts(data_ptr, lexeme_len) };
    self.scratch_data.push_str(from_utf8(bytes).unwrap_or(""));

    let mut source_data = AstArray::default();
    if self.options.store_cst_data {
      let sd = self.scratch_data.clone();
      source_data = self.copy_string(&sd);
    }

    // Remove all internal _
    if self.scratch_data.contains('_') {
      self.scratch_data.retain(|c| c != '_');
    }

    if FFlag::LuauIntegerType2.get() && self.scratch_data.ends_with('i') {
      let mut value: i64 = 0;

      let result: ConstantNumberParseResult =
        if self.scratch_data.starts_with("0x") || self.scratch_data.starts_with("0X") {
          parse_integer_64(&mut value, &self.scratch_data, 16)
        } else if self.scratch_data.starts_with("0b") || self.scratch_data.starts_with("0B") {
          parse_integer_64(&mut value, &self.scratch_data[2..], 2)
        } else {
          parse_integer_64(&mut value, &self.scratch_data, 10)
        };

      self.next_lexeme();

      if result == ConstantNumberParseResult::Malformed {
        return self.report_expr_error(
          start,
          AstArray::default(),
          format_args!("Malformed integer"),
        ) as *mut AstExpr;
      }

      if result != ConstantNumberParseResult::Ok {
        return self.report_expr_error(start, AstArray::default(), format_args!("Integer overflow"))
          as *mut AstExpr;
      }

      let node = unsafe {
        (*self.allocator).alloc(AstExprConstantInteger {
          base: AstExpr {
            base: AstNode {
              class_index: AstExprConstantInteger::CLASS_INDEX,
              location: start,
            },
          },
          value,
          parse_result: result,
        })
      };

      if self.options.store_cst_data {
        let cst_node = unsafe {
          (*self.allocator).alloc(CstExprConstantInteger {
            base: CstNode {
              class_index: CstExprConstantInteger::CLASS_INDEX,
            },
            value: source_data,
          })
        };
        self
          .cst_node_map
          .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
      }

      node as *mut AstExpr
    } else {
      let mut value: f64 = 0.0;
      let result = parse_double(&mut value, &self.scratch_data);

      self.next_lexeme();

      if result == ConstantNumberParseResult::Malformed {
        return self.report_expr_error(start, AstArray::default(), format_args!("Malformed number"))
          as *mut AstExpr;
      }

      let node = unsafe {
        (*self.allocator).alloc(AstExprConstantNumber {
          base: AstExpr {
            base: AstNode {
              class_index: AstExprConstantNumber::CLASS_INDEX,
              location: start,
            },
          },
          value,
          parse_result: result,
        })
      };

      if self.options.store_cst_data {
        let cst_node = unsafe {
          (*self.allocator).alloc(CstExprConstantNumber {
            base: CstNode {
              class_index: CstExprConstantNumber::CLASS_INDEX,
            },
            value: source_data,
          })
        };
        self
          .cst_node_map
          .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
      }

      node as *mut AstExpr
    }
  }
}
