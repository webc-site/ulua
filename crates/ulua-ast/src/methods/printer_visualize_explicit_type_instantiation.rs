use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  ast_array::AstArray, ast_type_or_pack::AstTypeOrPack,
  comma_separator_inserter::CommaSeparatorInserter, cst_type_instantiation::CstTypeInstantiation,
  position::EMPTY_POSITIONS, printer::Printer, writer::Writer,
};

impl<'a, W: Writer> Printer<'a, W> {
  pub(crate) fn visualize_explicit_type_instantiation(
    &mut self,
    type_arguments: AstArray<AstTypeOrPack>,
    cst_node: Option<&CstTypeInstantiation>,
  ) {
    // C<<T>>：两个连续 "<" 与 ">"，位置可各自被 CST 校准
    for (open, close) in [
      (cst_node.map(|cst| &cst.left_arrow_1_position), "<"),
      (cst_node.map(|cst| &cst.left_arrow_2_position), "<"),
    ] {
      match open {
        Some(pos) => self.maybe_advance_and_write(pos, close, false),
        None => self.writer.symbol(close),
      }
    }

    let mut comma = CommaSeparatorInserter::new(
      cst_node.map_or(EMPTY_POSITIONS, |cst| cst.comma_positions.as_slice()),
    );

    for type_or_pack in type_arguments.as_slice() {
      comma.operator_call(self.writer);

      if !type_or_pack.r#type.is_null() {
        self.visualize_type_annotation(unsafe { &mut *type_or_pack.r#type });
      } else {
        LUAU_ASSERT!(!type_or_pack.type_pack.is_null());
        self.visualize_type_pack_annotation(
          unsafe { &mut *type_or_pack.type_pack },
          false,
          true,
          false,
        );
      }
    }

    for (close, sym) in [
      (cst_node.map(|cst| &cst.right_arrow_1_position), ">"),
      (cst_node.map(|cst| &cst.right_arrow_2_position), ">"),
    ] {
      match close {
        Some(pos) => self.maybe_advance_and_write(pos, sym, false),
        None => self.writer.symbol(sym),
      }
    }
  }
}
