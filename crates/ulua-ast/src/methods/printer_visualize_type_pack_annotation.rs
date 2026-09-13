//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。

use crate::{
  records::{
    ast_array::AstArray, ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
    cst_type_pack_explicit::CstTypePackExplicit, cst_type_pack_generic::CstTypePackGeneric,
    position::Position, printer::Printer,
  },
  rtti::ast_node_as,
};

impl<'a> Printer<'a> {
  pub fn visualize_type_pack_annotation(
    &mut self,
    annotation: &mut AstTypePack,
    for_var_arg: bool,
    unconditionally_parenthesize: bool,
    for_function_return: bool,
  ) {
    self.advance(annotation.base.location.begin);

    let variadic_tp = unsafe { ast_node_as::<AstTypePackVariadic>(&mut annotation.base) };
    if let Some(variadic_tp) = unsafe { variadic_tp.as_mut() } {
      if !for_var_arg {
        self.writer.symbol("...");
      }
      unsafe {
        self.visualize_type_annotation(&mut *variadic_tp.variadic_type);
      }
      return;
    }

    let generic_tp = unsafe { ast_node_as::<AstTypePackGeneric>(&mut annotation.base) };
    if let Some(generic_tp) = unsafe { generic_tp.as_mut() } {
      self
        .writer
        .symbol(generic_tp.generic_name.as_str_or_empty());

      if let Some(cst_node) = unsafe {
        self
          .lookup_cst_node::<CstTypePackGeneric>(&mut annotation.base as *mut _)
          .as_mut()
      } {
        self.advance(cst_node.ellipsis_position);
      }

      self.writer.symbol("...");
      return;
    }

    let explicit_tp = unsafe { ast_node_as::<AstTypePackExplicit>(&mut annotation.base) };
    if let Some(explicit_tp) = unsafe { explicit_tp.as_mut() } {
      ulua_common::macros::luau_assert::LUAU_ASSERT!(!for_var_arg);

      let cst_node = self.lookup_cst_node::<CstTypePackExplicit>(&mut annotation.base as *mut _);

      if let Some(cst_node) = unsafe { cst_node.as_mut() } {
        self.visualize_type_list(
          &explicit_tp.type_list,
          false,
          cst_node.open_parentheses_position,
          cst_node.close_parentheses_position,
          cst_node.comma_positions,
        );
        return;
      }

      if for_function_return {
        let pack_size = explicit_tp.type_list.types.size
          + if !explicit_tp.type_list.tail_type.is_null() {
            1
          } else {
            0
          };

        self.visualize_type_list(
          &explicit_tp.type_list,
          pack_size != 1,
          Position::missing(),
          Position::missing(),
          AstArray::default(),
        );
        return;
      }

      self.visualize_type_list(
        &explicit_tp.type_list,
        unconditionally_parenthesize,
        Position::missing(),
        Position::missing(),
        AstArray::default(),
      );
      return;
    }

    ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
  }
}
