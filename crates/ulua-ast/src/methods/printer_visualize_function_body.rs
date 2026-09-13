//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。

use core::ptr::null;

use crate::records::{
  ast_array::AstArray, ast_expr_function::AstExprFunction,
  comma_separator_inserter::CommaSeparatorInserter, cst_expr_function::CstExprFunction,
  cst_generic_type_pack::CstGenericTypePack, printer::Printer,
};

pub trait IntoAstExprFunctionMut {
  fn into_ast_expr_function_mut(self) -> *mut AstExprFunction;
}

impl IntoAstExprFunctionMut for *mut AstExprFunction {
  fn into_ast_expr_function_mut(self) -> *mut AstExprFunction {
    self
  }
}

impl IntoAstExprFunctionMut for &mut AstExprFunction {
  fn into_ast_expr_function_mut(self) -> *mut AstExprFunction {
    self
  }
}

impl<'a> Printer<'a> {
  pub fn visualize_function_body<F: IntoAstExprFunctionMut>(&mut self, func: F) {
    let func = unsafe { &mut *func.into_ast_expr_function_mut() };
    let cst_node = self.lookup_cst_node::<CstExprFunction>(func as *mut AstExprFunction as *mut _);

    if func.generics.size > 0 || func.generic_packs.size > 0 {
      let comma_position = if cst_node.is_null() {
        null()
      } else {
        unsafe { (*cst_node).generics_comma_positions.data }
      };

      let mut comma = CommaSeparatorInserter::new(self.writer, comma_position);

      if !cst_node.is_null() {
        let open_pos = unsafe { &(*cst_node).open_generics_position };
        self.maybe_advance_and_write(open_pos, "<", false);
      } else {
        self.writer.symbol("<");
      }

      for &generic_ty in AstArray::iter(&func.generics) {
        comma.operator_call(self.writer);

        unsafe {
          self.writer.advance(&(*generic_ty).base.location.begin);
          self.writer.identifier((*generic_ty).name.as_str_or_empty());
        }
      }

      for &pack in AstArray::iter(&func.generic_packs) {
        comma.operator_call(self.writer);

        unsafe {
          self.writer.advance(&(*pack).base.location.begin);
          self.writer.identifier((*pack).name.as_str_or_empty());

          let generic_type_pack_cst_node =
            self.lookup_cst_node::<CstGenericTypePack>(pack as *mut _);
          if !generic_type_pack_cst_node.is_null() {
            let ellipsis_pos = &(*generic_type_pack_cst_node).ellipsis_position;
            self.advance(ellipsis_pos);
          }

          self.writer.symbol("...");
        }
      }

      if !cst_node.is_null() {
        let close_pos = unsafe { &(*cst_node).close_generics_position };
        self.maybe_advance_and_write(close_pos, ">", false);
      } else {
        self.writer.symbol(">");
      }
    }

    if let Some(arg_location) = func.arg_location.as_ref() {
      self.advance(arg_location.begin);
    }
    self.writer.symbol("(");

    let args_comma_pos = if cst_node.is_null() {
      null()
    } else {
      unsafe { (*cst_node).args_comma_positions.data }
    };

    let mut comma = CommaSeparatorInserter::new(self.writer, args_comma_pos);

    for (i, &local) in AstArray::iter(&func.args).enumerate() {
      comma.operator_call(self.writer);

      unsafe {
        self.advance((*local).location.begin);
        self.writer.identifier((*local).name.as_str_or_empty());

        if self.write_types && !(*local).annotation.is_null() {
          if !cst_node.is_null() {
            let colon_pos = (*cst_node).args_annotation_colon_positions.data.add(i);
            self.maybe_advance_and_write(&*colon_pos, ":", false);
          } else {
            self.writer.symbol(":");
          }

          self.visualize_type_annotation(&mut *(*local).annotation);
        }
      }
    }

    if func.vararg {
      comma.operator_call(self.writer);

      self.advance(func.vararg_location.begin);
      self.writer.symbol("...");

      if self.write_types && !func.vararg_annotation.is_null() {
        if !cst_node.is_null() {
          unsafe {
            self.maybe_advance_and_write(&(*cst_node).vararg_annotation_colon_position, ":", false);
          }
        } else {
          self.writer.symbol(":");
        }

        unsafe {
          self.visualize_type_pack_annotation(&mut *func.vararg_annotation, true, false, false);
        }
      }
    }

    if let Some(arg_location) = func.arg_location.as_ref() {
      self.advance_before(arg_location.end, 1);
    }
    self.writer.symbol(")");

    if self.write_types && !func.return_annotation.is_null() {
      if !cst_node.is_null() {
        unsafe {
          self.maybe_advance_and_write(&(*cst_node).return_specifier_position, ":", false);
        }
      } else {
        self.writer.symbol(":");
      }

      if cst_node.is_null() {
        self.writer.space();
      }

      unsafe {
        self.visualize_type_pack_annotation(&mut *func.return_annotation, false, false, true);
      }
    }

    unsafe {
      self.visualize_block_ast_stat_block(&mut *func.body);
      self.advance((*func.body).base.base.location.end);
    }
    self.writer.keyword("end");
  }
}
