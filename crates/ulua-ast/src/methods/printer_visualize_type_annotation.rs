//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。CST 侧经 `lookup_cst_node` 返回 `Option<&T>`，
//! 判空与字段读取全部走安全代码。
//!
//! 节点子指针（`o.r#type`、`idx.index_type` 等）不再在调用点解引用：直接传
//! 裸指针给 `visualize_type_annotation`（`IntoNodePtr` 归一），unsafe 收口在
//! 被调函数入口一次 `&mut *into_node_ptr()`；`AstArray<*mut T>` 遍历统一走
//! `iter_nodes`（安全迭代器，见 `records/ast_array.rs`）。

use core::mem::swap;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    ast_array::AstArray,
    ast_node::AstNode,
    ast_type::AstType,
    ast_type_error::AstTypeError,
    ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection,
    ast_type_optional::AstTypeOptional,
    ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString,
    ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof,
    ast_type_union::AstTypeUnion,
    comma_separator_inserter::CommaSeparatorInserter,
    cst_expr_table::CstExprTableSeparator::{Comma, Missing},
    cst_generic_type_pack::CstGenericTypePack,
    cst_type_function::CstTypeFunction,
    cst_type_group::CstTypeGroup,
    cst_type_intersection::CstTypeIntersection,
    cst_type_reference::CstTypeReference,
    cst_type_singleton_string::CstTypeSingletonString,
    cst_type_table::{
      CstTypeTable,
      CstTypeTableItemKind::{Indexer, StringProperty},
    },
    cst_type_typeof::CstTypeTypeof,
    cst_type_union::CstTypeUnion,
    position::{EMPTY_POSITIONS, Position},
    printer::{IntoNodePtr, Printer},
    writer::Writer,
  },
  rtti::{ast_node_is, ast_node_try_as},
};

/// 数组表 indexer 的键类型名（解析器对 `{T}` 数组表固定生成 number 键）。
const NUMBER_KEY: &str = "number";

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_type_annotation<T: IntoNodePtr<AstType>>(&mut self, type_annotation: T) {
    // SAFETY: type_annotation 指向 arena 中存活的 AstType 派生节点
    let type_annotation = unsafe { &*type_annotation.into_node_ptr() };
    self.advance(type_annotation.base.location.begin);

    if let Some(a) = ast_node_try_as::<AstTypeReference>(&type_annotation.base) {
      let cst_node = self.lookup_cst_node::<CstTypeReference>(&a.base.base);

      if let Some(prefix) = a.prefix {
        self.writer.write(prefix.as_bytes());
        if let Some(cst) = cst_node {
          self.advance(cst.prefix_point_position);
        }
        self.writer.symbol(".");
      }

      self.advance(a.name_location.begin);
      self.writer.write(a.name.as_bytes());

      if a.parameters.size > 0 || a.has_parameter_list {
        let mut comma = CommaSeparatorInserter::new(cst_node.map_or(EMPTY_POSITIONS, |cst| {
          cst.parameters_comma_positions.as_slice()
        }));

        if let Some(cst) = cst_node {
          self.advance(cst.open_parameters_position);
        }
        self.writer.symbol("<");

        for o in AstArray::iter(&a.parameters) {
          comma.operator_call(self.writer);

          if !o.r#type.is_null() {
            self.visualize_type_annotation(o.r#type);
          } else {
            self.visualize_type_pack_annotation(o.type_pack, false, true, false);
          }
        }

        self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_parameters_position), ">");
      }
    } else if let Some(a) =
      ast_node_try_as::<AstTypeFunction>(&type_annotation.base)
    {
      let cst_node = self.lookup_cst_node::<CstTypeFunction>(&a.base.base);

      if a.generics.size > 0 || a.generic_packs.size > 0 {
        let mut comma = CommaSeparatorInserter::new(cst_node.map_or(EMPTY_POSITIONS, |cst| {
          cst.generics_comma_positions.as_slice()
        }));

        if let Some(cst) = cst_node {
          self.advance(cst.open_generics_position);
        }
        self.writer.symbol("<");

        for o in a.generics.iter_nodes() {
          comma.operator_call(self.writer);
          self.writer.advance(&o.base.location.begin);
          self.writer.identifier(o.name.as_bytes());
        }

        for o in a.generic_packs.iter_nodes() {
          comma.operator_call(self.writer);
          self.writer.advance(&o.base.location.begin);
          self.writer.identifier(o.name.as_bytes());

          if let Some(cst) = self.lookup_cst_node::<CstGenericTypePack>(&o.base) {
            self.advance(cst.ellipsis_position);
          }
          self.writer.symbol("...");
        }

        self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_generics_position), ">");
      }

      let (open_args, close_args) = match cst_node {
        Some(cst) => (cst.open_args_position, cst.close_args_position),
        None => (Position::missing(), Position::missing()),
      };
      let comma_pos = cst_node.map_or(EMPTY_POSITIONS, |cst| {
        cst.arguments_comma_positions.as_slice()
      });
      let colon_pos = cst_node.map_or(EMPTY_POSITIONS, |cst| {
        cst.argument_name_colon_positions.as_slice()
      });

      self.visualize_named_type_list(
        &a.arg_types,
        cst_node.is_none(),
        open_args,
        close_args,
        comma_pos,
        a.arg_names.as_slice(),
        colon_pos,
      );

      if let Some(cst) = cst_node {
        self.advance(cst.return_arrow_position);
      }
      self.writer.symbol("->");

      self.visualize_type_pack_annotation(a.return_types, false, cst_node.is_none(), false);
    } else if let Some(a) =
      ast_node_try_as::<AstTypeTable>(&type_annotation.base)
    {
      // SAFETY: indexer 指向 arena 存活的 AstTableIndexer（解析器保证非空时才解引用）
      let indexer = (!a.indexer.is_null()).then(|| unsafe { &*a.indexer });
      // SAFETY: index_type 指向 arena 存活的 AstType 节点；class_index 匹配后
      // 经安全 try_as 下转
      let index_type = indexer
        .and_then(|idx| unsafe { idx.index_type.as_ref() })
        .and_then(|t| ast_node_try_as::<AstTypeReference>(&t.base));

      self.writer.symbol("{");

      let cst_node = self.lookup_cst_node::<CstTypeTable>(&a.base.base);
      match cst_node {
        Some(cst) if cst.is_array => {
          LUAU_ASSERT!(a.props.size == 0 && index_type.is_some_and(|t| t.name == NUMBER_KEY));
          if let Some(idx) = indexer {
            self.write_table_access(idx.access, idx.access_location);
            self.visualize_type_annotation(idx.result_type);
          }
        }
        Some(cst) => {
          // 非 Indexer 项逐一对应 props 元素（解析器构造保证；耗尽仅解析器
          // bug，跳过——cpp 侧裸指针推进同样依赖该不变式）
          let mut props = a.props.as_slice().iter();

          for item in cst.items.as_slice() {
            if item.kind == Indexer {
              LUAU_ASSERT!(indexer.is_some());
              if let Some(idx) = indexer {
                self.write_table_access(idx.access, idx.access_location);
                self.advance(item.indexer_open_position);
                self.writer.symbol("[");
                self.visualize_type_annotation(idx.index_type);
                self.maybe_advance_and_write(&item.indexer_close_position, "]", false);
                self.maybe_advance_and_write(&item.colon_position, ":", false);
                self.visualize_type_annotation(idx.result_type);
              }
            } else {
              let Some(prop) = props.next() else { continue };
              self.write_table_access(prop.access, prop.access_location);

              if item.kind == StringProperty {
                if item.indexer_open_position.has_value() {
                  self.maybe_advance_and_write(&item.indexer_open_position, "[", false);
                }
                self.advance(item.string_position);
                // SAFETY: StringProperty 项必带 string_info（解析器保证非空）
                let info = unsafe { &*item.string_info };
                self.writer.source_string(
                  // 源文本直切片（引号内原文，非 fixup 产物）。
                  info.source_string.as_bytes(),
                  info.quote_style,
                  info.block_depth,
                );
                if item.indexer_close_position.has_value() {
                  self.maybe_advance_and_write(&item.indexer_close_position, "]", false);
                }
              } else {
                self.advance(prop.location.begin);
                self.writer.identifier(prop.name.as_bytes());
              }

              self.maybe_advance_and_write(&item.colon_position, ":", false);
              self.visualize_type_annotation(prop.r#type);
            }

            if item.separator != Missing {
              LUAU_ASSERT!(item.separator_position.has_value());
              self.maybe_advance_and_write(
                &item.separator_position,
                if item.separator == Comma { "," } else { ";" },
                true,
              );
            }
          }
        }
        None => {
          if a.props.size == 0 && index_type.is_some_and(|t| t.name == NUMBER_KEY) {
            // is_array 单元素表必有 indexer（解析器保证），index_type 为 Some
            // 蕴含 indexer 为 Some
            if let Some(idx) = indexer {
              self.visualize_type_annotation(idx.result_type);
            }
          } else {
            let mut comma = CommaSeparatorInserter::new(EMPTY_POSITIONS);

            for prop in AstArray::iter(&a.props) {
              comma.operator_call(self.writer);

              self.advance(prop.location.begin);
              self.writer.identifier(prop.name.as_bytes());
              if !prop.r#type.is_null() {
                self.writer.symbol(":");
                self.visualize_type_annotation(prop.r#type);
              }
            }
            if let Some(idx) = indexer {
              comma.operator_call(self.writer);

              self.writer.symbol("[");
              self.visualize_type_annotation(idx.index_type);
              self.writer.symbol("]");
              self.writer.symbol(":");
              self.visualize_type_annotation(idx.result_type);
            }
          }
        }
      }

      let mut end_pos = type_annotation.base.location.end;
      if end_pos.column > 0 {
        end_pos.column -= 1;
      }
      self.advance(end_pos);
      self.writer.symbol("}");
    } else if let Some(a) =
      ast_node_try_as::<AstTypeTypeof>(&type_annotation.base)
    {
      self.writer.keyword("typeof");
      let cst_node = self.lookup_cst_node::<CstTypeTypeof>(&a.base.base);
      match cst_node {
        Some(cst) => {
          self.maybe_advance_and_write(&cst.open_position, "(", false);
          self.visualize_ast_expr(a.expr);
          self.maybe_advance_and_write(&cst.close_position, ")", false);
        }
        None => {
          self.writer.symbol("(");
          self.visualize_ast_expr(a.expr);
          self.writer.symbol(")");
        }
      }
    } else if let Some(a) =
      ast_node_try_as::<AstTypeUnion>(&type_annotation.base)
    {
      let cst_node = self.lookup_cst_node::<CstTypeUnion>(&a.base.base);

      if cst_node.is_none()
        && let [l_ptr, r_ptr] = a.types.as_slice()
      {
        let mut l = *l_ptr;
        let mut r = *r_ptr;

        // SAFETY: l/r 均指向 arena 存活的 AstType 节点；class_index 匹配后
        // 经安全 try_as 下转，名字比较走 PartialEq<&str>
        let l_is_nil = ast_node_try_as::<AstTypeReference>(&unsafe { &*l }.base)
          .is_some_and(|t| t.name == "nil");
        let r_is_optional = ast_node_is::<AstTypeOptional>(r.cast::<AstNode>());
        if l_is_nil && !r_is_optional {
          swap(&mut l, &mut r);
        }

        // it's still possible that we had a (T | U) or (T | nil) and not (nil | T)
        // SAFETY: swap 后的 r 指向 arena 存活节点（指针值交换，节点内存不变）
        let r_is_nil = ast_node_try_as::<AstTypeReference>(&unsafe { &*r }.base)
          .is_some_and(|t| t.name == "nil");
        if r_is_nil {
          let wrap = ast_node_is::<AstTypeIntersection>(l.cast::<AstNode>())
            || ast_node_is::<AstTypeFunction>(l.cast::<AstNode>());
          if wrap {
            self.writer.symbol("(");
          }
          self.visualize_type_annotation(l);
          if wrap {
            self.writer.symbol(")");
          }
          self.writer.symbol("?");
          return;
        }
      }

      if let Some(cst) = cst_node {
        self.maybe_advance_and_write(&cst.leading_position, "|", false);
      }

      // separator_positions 长度为 types 数减一（cpp 同款；首个分隔符存
      // leadingPosition）。迭代器顺序消耗替代手动索引（optional 项 continue
      // 前不消耗，与 cpp `separatorIndex` 递增点一致）。
      let mut seps = cst_node.map(|cst| cst.separator_positions.as_slice().iter());

      for (i, t) in a.types.iter_nodes().enumerate() {
        if let Some(optional) = ast_node_try_as::<AstTypeOptional>(&t.base) {
          self.advance(optional.base.base.location.begin);
          self.writer.symbol("?");
          continue;
        }

        if i > 0 {
          match seps.as_mut().and_then(Iterator::next) {
            Some(pos) => self.advance(*pos),
            None => self.writer.maybe_space(&t.base.location.begin, 2),
          }
          self.writer.symbol("|");
        }

        let wrap = cst_node.is_none()
          && (ast_node_is::<AstTypeIntersection>(&t.base)
            || ast_node_is::<AstTypeFunction>(&t.base));
        if wrap {
          self.writer.symbol("(");
        }
        self.visualize_type_annotation(t);
        if wrap {
          self.writer.symbol(")");
        }
      }
    } else if let Some(a) =
      ast_node_try_as::<AstTypeIntersection>(&type_annotation.base)
    {
      let cst_node = self.lookup_cst_node::<CstTypeIntersection>(&a.base.base);

      if let Some(cst) = cst_node {
        self.maybe_advance_and_write(&cst.leading_position, "&", false);
      }

      // separator_positions 长度为 types 数减一（解析器保证，C++ 同样无越界
      // 检查）；i-1 索引即顺序消耗，迭代器 next 等价。
      let mut seps = cst_node.map(|cst| cst.separator_positions.as_slice().iter());

      for (i, t) in a.types.iter_nodes().enumerate() {
        if i > 0 {
          match seps.as_mut().and_then(Iterator::next) {
            Some(pos) => self.advance(*pos),
            None => self.writer.maybe_space(&t.base.location.begin, 2),
          }
          self.writer.symbol("&");
        }

        let wrap = cst_node.is_none()
          && (ast_node_is::<AstTypeUnion>(&t.base) || ast_node_is::<AstTypeFunction>(&t.base));
        if wrap {
          self.writer.symbol("(");
        }
        self.visualize_type_annotation(t);
        if wrap {
          self.writer.symbol(")");
        }
      }
    } else if let Some(a) =
      ast_node_try_as::<AstTypeGroup>(&type_annotation.base)
    {
      self.writer.symbol("(");
      self.visualize_type_annotation(a.type_);

      // cpp 无 LuauCstTypeGroup flag：无条件查 CstTypeGroup
      let cst_node = self.lookup_cst_node::<CstTypeGroup>(&a.base.base);
      match cst_node {
        Some(cst) => self.maybe_advance_and_write(&cst.close_position, ")", false),
        None => {
          self.advance_before(type_annotation.base.location.end, 1);
          self.writer.symbol(")");
        }
      }
    } else if let Some(a) =
      ast_node_try_as::<AstTypeSingletonBool>(&type_annotation.base)
    {
      self.writer.keyword(if a.value { "true" } else { "false" });
    } else if let Some(a) =
      ast_node_try_as::<AstTypeSingletonString>(&type_annotation.base)
    {
      match self.lookup_cst_node::<CstTypeSingletonString>(&a.base.base) {
        Some(cst) => {
          self.writer.source_string(
            // 源文本直切片（引号内原文，非 fixup 产物）。
            cst.source_string.as_bytes(),
            cst.quote_style,
            cst.block_depth,
          );
        }
        None => {
          // value 经 lexer_fixup_quoted_bytes 可含任意字节，走字节通道。
          self.writer.write(a.value.as_bytes());
        }
      }
    } else if ast_node_is::<AstTypeError>(&type_annotation.base) {
      self.writer.symbol("%error-type%");
    } else {
      LUAU_ASSERT!(false);
    }
  }
}
