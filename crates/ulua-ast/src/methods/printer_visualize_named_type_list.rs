use crate::{
  records::{
    arg_name_inserter::ArgNameInserter, ast_type_group::AstTypeGroup, ast_type_list::AstTypeList,
    comma_separator_inserter::CommaSeparatorInserter, position::Position, printer::Printer,
    writer::Writer,
  },
  rtti::ast_node_try_as,
  type_aliases::ast_argument_name::AstArgumentName,
};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_named_type_list(
    &mut self,
    list: &AstTypeList,
    unconditionally_parenthesize: bool,
    open_parentheses_position: Position,
    close_parentheses_position: Position,
    comma_positions: &[Position],
    arg_names: &[Option<AstArgumentName>],
    arg_names_colon_positions: &[Position],
  ) {
    let type_count = list.types.len() + usize::from(!list.tail_type.is_null());

    if type_count == 0 {
      self.maybe_advance_and_write(
        &open_parentheses_position,
        "(",
        unconditionally_parenthesize,
      );
      self.maybe_advance_and_write(
        &close_parentheses_position,
        ")",
        unconditionally_parenthesize,
      );
    } else if type_count == 1 {
      // SAFETY: 首元素指针指向 arena 存活的 AstType 节点；class_index 匹配后
      // 经安全 try_as 下转。首元素仅在 types 非空时解引用（cpp `data[0]`
      // 同样以 size==0 短路保护）。
      let first_type = list.types.as_slice().first().map(|&p| unsafe { &*p });

      let should_parenthesize = unconditionally_parenthesize
        && first_type.is_none_or(|t| ast_node_try_as::<AstTypeGroup>(&t.base).is_none());

      self.maybe_advance_and_write(&open_parentheses_position, "(", should_parenthesize);

      // 块内 drop inserter：其持有 &mut writer，须在 visualize_* 借用前释放。
      {
        let mut arg_name_inserter =
          ArgNameInserter::new(self.writer, arg_names, arg_names_colon_positions);
        arg_name_inserter.operator_call();
      }

      match first_type {
        Some(t) => self.visualize_type_annotation(t),
        None => {
          // types 空：仅 variadic tail（cpp 同款）
          // SAFETY: tail_type 指向 arena 存活的 AstTypePack（非空已判）
          self.visualize_type_pack_annotation(list.tail_type, false, true, false);
        }
      }

      self.maybe_advance_and_write(&close_parentheses_position, ")", should_parenthesize);
    } else {
      self.maybe_advance_and_write(
        &open_parentheses_position,
        "(",
        unconditionally_parenthesize,
      );

      // 逗号游标跨调用持有（cpp 同款；不持有 writer，无借用冲突）。
      let mut comma = CommaSeparatorInserter::new(comma_positions);
      let mut arg_name_idx = 0;

      for t in list.types.iter_nodes() {
        comma.operator_call(self.writer);
        arg_name_idx = {
          let mut arg_name =
            ArgNameInserter::new(self.writer, arg_names, arg_names_colon_positions);
          arg_name.idx = arg_name_idx;
          arg_name.operator_call();
          arg_name.idx
        };
        self.visualize_type_annotation(t);
      }

      if !list.tail_type.is_null() {
        comma.operator_call(self.writer);
        // SAFETY: tail_type 指向 arena 存活的 AstTypePack（非空已判）
        self.visualize_type_pack_annotation(list.tail_type, false, true, false);
      }

      self.maybe_advance_and_write(
        &close_parentheses_position,
        ")",
        unconditionally_parenthesize,
      );
    }
  }
}
