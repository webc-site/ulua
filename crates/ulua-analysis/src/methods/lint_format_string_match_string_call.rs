use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_string::AstExprConstantString, ast_name::AstName, ast_node::AstNode,
  },
  rtti,
};
use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_format_string::LintFormatString};

impl LintFormatString {
  pub fn match_string_call(
    &mut self,
    name: AstName,
    self_expr: *mut AstExpr,
    args: AstArray<*mut AstExpr>,
  ) {
    let args_slice = args.as_slice();
    let is_format = name == "format";
    let is_pack_packsize_unpack = name == "pack" || name == "packsize" || name == "unpack";
    let is_match_gmatch = name == "match" || name == "gmatch";
    let is_find = name == "find";
    let is_gsub = name == "gsub";

    if is_format {
      let fmt_node =
        unsafe { rtti::ast_node_as::<AstExprConstantString>(self_expr as *mut AstNode) };
      if !fmt_node.is_null() {
        let fmt = unsafe { &*fmt_node };
        if let Some(error) = self.check_string_format(fmt.value.as_bytes()) {
          emit_warning(
            unsafe { &mut *self.context },
            Code::FormatString,
            fmt.base.base.location,
            format_args!("Invalid format string: {}", error),
          );
        }
      }
    } else if is_pack_packsize_unpack {
      let fmt_node =
        unsafe { rtti::ast_node_as::<AstExprConstantString>(self_expr as *mut AstNode) };
      if !fmt_node.is_null() {
        let fmt = unsafe { &*fmt_node };
        let is_packsize = name == "packsize";
        if let Some(error) = self.check_string_pack(fmt.value.as_bytes(), is_packsize) {
          emit_warning(
            unsafe { &mut *self.context },
            Code::FormatString,
            fmt.base.base.location,
            format_args!("Invalid pack format: {}", error),
          );
        }
      }
    } else if is_match_gmatch && let Some(&first_arg) = args_slice.first() {
      let pat_node =
        unsafe { rtti::ast_node_as::<AstExprConstantString>(first_arg as *mut AstNode) };
      if !pat_node.is_null() {
        let pat = unsafe { &*pat_node };
        if let Err(error) = self.check_string_match(pat.value.as_bytes()) {
          emit_warning(
            unsafe { &mut *self.context },
            Code::FormatString,
            pat.base.base.location,
            format_args!("Invalid match pattern: {}", error),
          );
        }
      }
    } else if is_find && !args_slice.is_empty() && args_slice.len() <= 2 {
      let first_arg = args_slice[0];
      let pat_node =
        unsafe { rtti::ast_node_as::<AstExprConstantString>(first_arg as *mut AstNode) };
      if !pat_node.is_null() {
        let pat = unsafe { &*pat_node };
        if let Err(error) = self.check_string_match(pat.value.as_bytes()) {
          emit_warning(
            unsafe { &mut *self.context },
            Code::FormatString,
            pat.base.base.location,
            format_args!("Invalid match pattern: {}", error),
          );
        }
      }
    } else if is_find && args_slice.len() >= 3 {
      let third_arg = args_slice[2];
      let mode = unsafe { rtti::ast_node_as::<AstExprConstantBool>(third_arg as *mut AstNode) };
      if !mode.is_null() {
        let mode_val = unsafe { &*mode };
        if !mode_val.value {
          let first_arg = args_slice[0];
          let pat_node =
            unsafe { rtti::ast_node_as::<AstExprConstantString>(first_arg as *mut AstNode) };
          if !pat_node.is_null() {
            let pat = unsafe { &*pat_node };
            if let Err(error) = self.check_string_match(pat.value.as_bytes()) {
              emit_warning(
                unsafe { &mut *self.context },
                Code::FormatString,
                pat.base.base.location,
                format_args!("Invalid match pattern: {}", error),
              );
            }
          }
        }
      }
    } else if is_gsub && args_slice.len() > 1 {
      let mut captures = -1;

      let first_arg = args_slice[0];
      let pat_node =
        unsafe { rtti::ast_node_as::<AstExprConstantString>(first_arg as *mut AstNode) };
      if !pat_node.is_null() {
        let pat = unsafe { &*pat_node };
        match self.check_string_match(pat.value.as_bytes()) {
          Ok(c) => {
            captures = c;
          }
          Err(error) => {
            emit_warning(
              unsafe { &mut *self.context },
              Code::FormatString,
              pat.base.base.location,
              format_args!("Invalid match pattern: {}", error),
            );
          }
        }
      }

      let second_arg = args_slice[1];
      let rep_node =
        unsafe { rtti::ast_node_as::<AstExprConstantString>(second_arg as *mut AstNode) };
      if !rep_node.is_null() {
        let rep = unsafe { &*rep_node };
        if let Some(error) = self.check_string_replace(rep.value.as_bytes(), captures) {
          emit_warning(
            unsafe { &mut *self.context },
            Code::FormatString,
            rep.base.base.location,
            format_args!("Invalid match replacement: {}", error),
          );
        }
      }
    }
  }
}
