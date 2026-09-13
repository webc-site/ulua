//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。

use core::{
  mem::swap,
  ptr::{null, null_mut},
  slice::from_raw_parts,
  str::from_utf8_unchecked,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::ast_table_access::AstTableAccess,
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
    position::Position,
    printer::Printer,
  },
  rtti::{ast_node_as, ast_node_is, ast_node_try_as_mut},
};

pub trait IntoAstTypeMut {
  fn into_ast_type_mut(self) -> *mut AstType;
}

impl IntoAstTypeMut for *mut AstType {
  fn into_ast_type_mut(self) -> *mut AstType {
    self
  }
}

impl IntoAstTypeMut for &*mut AstType {
  fn into_ast_type_mut(self) -> *mut AstType {
    *self
  }
}

impl IntoAstTypeMut for &mut AstType {
  fn into_ast_type_mut(self) -> *mut AstType {
    self
  }
}

impl<'a> Printer<'a> {
  pub fn visualize_type_annotation<T: IntoAstTypeMut>(&mut self, type_annotation: T) {
    let type_annotation = unsafe { &mut *type_annotation.into_ast_type_mut() };
    self.advance(type_annotation.base.location.begin);

    if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstTypeReference>(type_annotation as *mut AstType as *mut AstNode)
    } {
      let cst_node =
        self.lookup_cst_node::<CstTypeReference>(a as *mut AstTypeReference as *mut AstNode);

      if let Some(prefix) = a.prefix {
        self.writer.write(prefix.as_str_or_empty());
        if !cst_node.is_null() {
          self.advance(unsafe { (*cst_node).prefix_point_position });
        }
        self.writer.symbol(".");
      }

      self.advance(a.name_location.begin);
      self.writer.write(a.name.as_str_or_empty());

      if a.parameters.size > 0 || a.has_parameter_list {
        let comma_pos_ptr = if !cst_node.is_null() {
          unsafe { (*cst_node).parameters_comma_positions.begin() }
        } else {
          null()
        };

        if !cst_node.is_null() {
          self.advance(unsafe { (*cst_node).open_parameters_position });
        }
        self.writer.symbol("<");

        let mut comma = CommaSeparatorInserter::new(self.writer, comma_pos_ptr);
        for o in AstArray::iter(&a.parameters) {
          comma.operator_call(self.writer);

          if !o.r#type.is_null() {
            self.visualize_type_annotation(unsafe { &mut *o.r#type });
          } else {
            self.visualize_type_pack_annotation(unsafe { &mut *o.type_pack }, false, false, false);
          }
        }

        if !cst_node.is_null() {
          let pos = unsafe { (*cst_node).close_parameters_position };
          self.maybe_advance_and_write(&pos, ">", false);
        } else {
          self.writer.symbol(">");
        }
      }
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstTypeFunction>(type_annotation as *mut AstType as *mut AstNode)
    } {
      let cst_node =
        self.lookup_cst_node::<CstTypeFunction>(a as *mut AstTypeFunction as *mut AstNode);

      if a.generics.size > 0 || a.generic_packs.size > 0 {
        let comma_pos_ptr = if !cst_node.is_null() {
          unsafe { (*cst_node).generics_comma_positions.begin() }
        } else {
          null()
        };

        if !cst_node.is_null() {
          self.advance(unsafe { (*cst_node).open_generics_position });
        }
        self.writer.symbol("<");

        let mut comma = CommaSeparatorInserter::new(self.writer, comma_pos_ptr);
        for &o in AstArray::iter(&a.generics) {
          comma.operator_call(self.writer);

          let o = unsafe { &mut *o };
          self.writer.advance(&o.base.location.begin);
          self.writer.identifier(o.name.as_str_or_empty());
        }

        for &o in AstArray::iter(&a.generic_packs) {
          comma.operator_call(self.writer);

          let o = unsafe { &mut *o };
          self.writer.advance(&o.base.location.begin);
          self.writer.identifier(o.name.as_str_or_empty());

          let generic_type_pack_cst_node =
            self.lookup_cst_node::<CstGenericTypePack>(o as *mut _ as *mut AstNode);
          if !generic_type_pack_cst_node.is_null() {
            self.advance(unsafe { (*generic_type_pack_cst_node).ellipsis_position });
          }
          self.writer.symbol("...");
        }

        if !cst_node.is_null() {
          let pos = unsafe { (*cst_node).close_generics_position };
          self.maybe_advance_and_write(&pos, ">", false);
        } else {
          self.writer.symbol(">");
        }
      }

      let open_args = if !cst_node.is_null() {
        unsafe { (*cst_node).open_args_position }
      } else {
        Position::missing()
      };
      let close_args = if !cst_node.is_null() {
        unsafe { (*cst_node).close_args_position }
      } else {
        Position::missing()
      };
      let comma_pos = if !cst_node.is_null() {
        unsafe { &(*cst_node).arguments_comma_positions }
      } else {
        &AstArray::default()
      };
      let colon_pos = if !cst_node.is_null() {
        unsafe { &(*cst_node).argument_name_colon_positions }
      } else {
        &AstArray::default()
      };

      self.visualize_named_type_list(
        &a.arg_types,
        cst_node.is_null(),
        open_args,
        close_args,
        comma_pos,
        &a.arg_names,
        colon_pos,
      );

      if !cst_node.is_null() {
        self.advance(unsafe { (*cst_node).return_arrow_position });
      }
      self.writer.symbol("->");

      self.visualize_type_pack_annotation(
        unsafe { &mut *a.return_types },
        false,
        cst_node.is_null(),
        false,
      );
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstTypeTable>(type_annotation as *mut AstType as *mut AstNode)
    } {
      let index_type = if !a.indexer.is_null() {
        unsafe { ast_node_as::<AstTypeReference>((*a.indexer).index_type as *mut AstNode) }
      } else {
        null_mut()
      };

      self.writer.symbol("{");

      let cst_node = self.lookup_cst_node::<CstTypeTable>(a as *mut AstTypeTable as *mut AstNode);
      if !cst_node.is_null() {
        if unsafe { (*cst_node).is_array } {
          LUAU_ASSERT!(
            a.props.size == 0 && !index_type.is_null() && unsafe { (*index_type).name == "number" }
          );
          unsafe {
            if let Some(loc) = (*a.indexer).access_location {
              self.advance(loc.begin);
              self
                .writer
                .keyword(if (*a.indexer).access == AstTableAccess::Read {
                  "read"
                } else {
                  "write"
                });
            }
            self.visualize_type_annotation(&mut *(*a.indexer).result_type);
          }
        } else {
          let mut prop_idx = 0;
          let items = unsafe { (*cst_node).items.as_slice() };

          for item in items {
            if item.kind == Indexer {
              LUAU_ASSERT!(!a.indexer.is_null());
              unsafe {
                if let Some(loc) = (*a.indexer).access_location {
                  self.advance(loc.begin);
                  self
                    .writer
                    .keyword(if (*a.indexer).access == AstTableAccess::Read {
                      "read"
                    } else {
                      "write"
                    });
                }
                self.advance(item.indexer_open_position);
                self.writer.symbol("[");
                self.visualize_type_annotation(&mut *(*a.indexer).index_type);
                self.maybe_advance_and_write(&item.indexer_close_position, "]", false);
                self.maybe_advance_and_write(&item.colon_position, ":", false);
                self.visualize_type_annotation(&mut *(*a.indexer).result_type);
              }
            } else {
              let prop = unsafe { &mut *a.props.data.add(prop_idx) };
              if let Some(loc) = prop.access_location {
                self.advance(loc.begin);
                self.writer.keyword(if prop.access == AstTableAccess::Read {
                  "read"
                } else {
                  "write"
                });
              }

              if item.kind == StringProperty {
                if item.indexer_open_position.has_value() {
                  self.maybe_advance_and_write(&item.indexer_open_position, "[", false);
                }
                self.advance(item.string_position);
                unsafe {
                  let s_ptr = (*item.string_info).source_string.data as *const u8;
                  let s_len = (*item.string_info).source_string.size;
                  let s_str = from_utf8_unchecked(from_raw_parts(s_ptr, s_len));
                  self.writer.source_string(
                    s_str,
                    (*item.string_info).quote_style,
                    (*item.string_info).block_depth,
                  );
                }
                if item.indexer_close_position.has_value() {
                  self.maybe_advance_and_write(&item.indexer_close_position, "]", false);
                }
              } else {
                self.advance(prop.location.begin);
                self.writer.identifier(prop.name.as_str_or_empty());
              }

              self.maybe_advance_and_write(&item.colon_position, ":", false);
              self.visualize_type_annotation(unsafe { &mut *prop.r#type });
              prop_idx += 1;
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
      } else {
        if a.props.size == 0 && !index_type.is_null() && unsafe { (*index_type).name == "number" } {
          self.visualize_type_annotation(unsafe { &mut *(*a.indexer).result_type });
        } else {
          let mut comma = CommaSeparatorInserter::new(self.writer, null());

          for prop in AstArray::iter(&a.props) {
            comma.operator_call(self.writer);

            self.advance(prop.location.begin);
            self.writer.identifier(prop.name.as_str_or_empty());
            if !prop.r#type.is_null() {
              self.writer.symbol(":");
              self.visualize_type_annotation(unsafe { &mut *prop.r#type });
            }
          }
          if !a.indexer.is_null() {
            comma.operator_call(self.writer);

            self.writer.symbol("[");
            self.visualize_type_annotation(unsafe { &mut *(*a.indexer).index_type });
            self.writer.symbol("]");
            self.writer.symbol(":");
            self.visualize_type_annotation(unsafe { &mut *(*a.indexer).result_type });
          }
        }
      }

      let mut end_pos = type_annotation.base.location.end;
      if end_pos.column > 0 {
        end_pos.column -= 1;
      }
      self.advance(end_pos);
      self.writer.symbol("}");
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstTypeTypeof>(type_annotation as *mut AstType as *mut AstNode)
    } {
      self.writer.keyword("typeof");
      let cst_node = self.lookup_cst_node::<CstTypeTypeof>(a as *mut AstTypeTypeof as *mut AstNode);
      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).open_position }, "(", false);
        self.visualize_ast_expr(unsafe { &mut *a.expr });
        self.maybe_advance_and_write(unsafe { &(*cst_node).close_position }, ")", false);
      } else {
        self.writer.symbol("(");
        self.visualize_ast_expr(unsafe { &mut *a.expr });
        self.writer.symbol(")");
      }
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstTypeUnion>(type_annotation as *mut AstType as *mut AstNode)
    } {
      let cst_node = self.lookup_cst_node::<CstTypeUnion>(a as *mut AstTypeUnion as *mut AstNode);

      if cst_node.is_null() && a.types.size == 2 {
        let mut l = unsafe { *a.types.data.add(0) };
        let mut r = unsafe { *a.types.data.add(1) };

        let lta = unsafe { ast_node_as::<AstTypeReference>(l as *mut AstNode) };
        if !lta.is_null()
          && unsafe { (*lta).name == "nil" }
          && !ast_node_is::<AstTypeOptional>(unsafe { &*(r as *mut AstNode) })
        {
          swap(&mut l, &mut r);
        }

        let rta = unsafe { ast_node_as::<AstTypeReference>(r as *mut AstNode) };
        if !rta.is_null() && unsafe { (*rta).name == "nil" } {
          let wrap = ast_node_is::<AstTypeIntersection>(unsafe { &*(l as *mut AstNode) })
            || ast_node_is::<AstTypeFunction>(unsafe { &*(l as *mut AstNode) });
          if wrap {
            self.writer.symbol("(");
          }
          self.visualize_type_annotation(unsafe { &mut *l });
          if wrap {
            self.writer.symbol(")");
          }
          self.writer.symbol("?");
          return;
        }
      }

      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).leading_position }, "|", false);
      }

      let seps = if !cst_node.is_null() {
        Some(unsafe { (*cst_node).separator_positions.as_slice() })
      } else {
        None
      };

      let mut separator_index = 0;
      for (i, &t_ptr) in AstArray::iter(&a.types).enumerate() {
        let t = unsafe { &mut *t_ptr };
        if let Some(optional) =
          unsafe { ast_node_try_as_mut::<AstTypeOptional>(t as *mut AstType as *mut AstNode) }
        {
          self.advance(optional.base.base.location.begin);
          self.writer.symbol("?");
          continue;
        }

        if i > 0 {
          if let Some(seps) = seps {
            // SAFETY: separator_positions 与 types 等长（解析器保证，C++ 同样无越界检查）
            self.advance(unsafe { *seps.get_unchecked(separator_index) });
            separator_index += 1;
          } else {
            self.writer.maybe_space(&t.base.location.begin, 2);
          }
          self.writer.symbol("|");
        }

        let wrap = cst_node.is_null()
          && (ast_node_is::<AstTypeIntersection>(unsafe { &*(t as *mut AstType as *mut AstNode) })
            || ast_node_is::<AstTypeFunction>(unsafe { &*(t as *mut AstType as *mut AstNode) }));
        if wrap {
          self.writer.symbol("(");
        }
        self.visualize_type_annotation(t);
        if wrap {
          self.writer.symbol(")");
        }
      }
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstTypeIntersection>(type_annotation as *mut AstType as *mut AstNode)
    } {
      let cst_node =
        self.lookup_cst_node::<CstTypeIntersection>(a as *mut AstTypeIntersection as *mut AstNode);

      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).leading_position }, "&", false);
      }

      let seps = if !cst_node.is_null() {
        Some(unsafe { (*cst_node).separator_positions.as_slice() })
      } else {
        None
      };

      for (i, &t_ptr) in AstArray::iter(&a.types).enumerate() {
        let t = unsafe { &mut *t_ptr };
        if i > 0 {
          if let Some(seps) = seps {
            // SAFETY: separator_positions 长度为 types 数减一（解析器保证，C++ 同样无越界检查）
            self.advance(unsafe { *seps.get_unchecked(i - 1) });
          } else {
            self.writer.maybe_space(&t.base.location.begin, 2);
          }
          self.writer.symbol("&");
        }

        let wrap = cst_node.is_null()
          && (ast_node_is::<AstTypeUnion>(unsafe { &*(t as *mut AstType as *mut AstNode) })
            || ast_node_is::<AstTypeFunction>(unsafe { &*(t as *mut AstType as *mut AstNode) }));
        if wrap {
          self.writer.symbol("(");
        }
        self.visualize_type_annotation(t);
        if wrap {
          self.writer.symbol(")");
        }
      }
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstTypeGroup>(type_annotation as *mut AstType as *mut AstNode)
    } {
      self.writer.symbol("(");
      self.visualize_type_annotation(unsafe { &mut *a.type_ });

      // cpp 无 LuauCstTypeGroup flag：无条件查 CstTypeGroup
      let cst_node = self.lookup_cst_node::<CstTypeGroup>(a as *mut AstTypeGroup as *mut AstNode);
      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).close_position }, ")", false);
      } else {
        self.advance_before(type_annotation.base.location.end, 1);
        self.writer.symbol(")");
      }
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstTypeSingletonBool>(type_annotation as *mut AstType as *mut AstNode)
    } {
      self.writer.keyword(if a.value { "true" } else { "false" });
    } else if let Some(a) = unsafe {
      ast_node_as::<AstTypeSingletonString>(type_annotation as *mut AstType as *mut AstNode)
        .as_mut()
    } {
      let cst_node = self.lookup_cst_node::<CstTypeSingletonString>(
        a as *mut AstTypeSingletonString as *mut AstNode,
      );
      if !cst_node.is_null() {
        unsafe {
          let s_ptr = (*cst_node).source_string.data as *const u8;
          let s_len = (*cst_node).source_string.size;
          let s_str = from_utf8_unchecked(from_raw_parts(s_ptr, s_len));
          self
            .writer
            .source_string(s_str, (*cst_node).quote_style, (*cst_node).block_depth);
        }
      } else {
        let s_ptr = a.value.data as *const u8;
        let s_len = a.value.size;
        let s_str = unsafe { from_utf8_unchecked(from_raw_parts(s_ptr, s_len)) };
        self.writer.write(s_str);
      }
    } else if ast_node_is::<AstTypeError>(unsafe {
      &*(type_annotation as *mut AstType as *mut AstNode)
    }) {
      self.writer.symbol("%error-type%");
    } else {
      LUAU_ASSERT!(false);
    }
  }
}
