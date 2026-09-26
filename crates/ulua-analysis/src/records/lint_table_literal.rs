use alloc::{collections::BTreeMap, string::String, vec::Vec};
use core::ptr::from_mut;

use ulua_ast::{
  enums::{ast_expr_ref::AstExprRef, ast_table_access::AstTableAccess},
  records::{
    ast_expr_table::{AstExprTable, ItemKind},
    ast_name::AstName,
    ast_type::AstType,
    ast_type_pack::AstTypePack,
    ast_type_table::AstTypeTable,
    ast_visitor::AstVisitor,
    location::Location,
  },
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintTableLiteral<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintTableLiteral<'ctx> {
  fn visit_expr_table(&mut self, node: &mut AstExprTable) -> bool {
    self.visit_ast_expr_table(from_mut(node))
  }

  fn visit_type(&mut self, _node: &mut AstType) -> bool {
    self.visit_ast_type()
  }

  fn visit_type_pack(&mut self, _node: &mut AstTypePack) -> bool {
    self.visit_ast_type_pack()
  }

  fn visit_type_table(&mut self, node: &mut AstTypeTable) -> bool {
    self.visit_ast_type_table(from_mut(node))
  }
}

// —— 原 methods/lint_table_literal_process.rs ——
impl<'ctx> LintTableLiteral<'ctx> {
  lint_stat_process!(LintTableLiteral);
}

// —— 原 methods/lint_table_literal_visit_linter.rs ——
impl<'ctx> LintTableLiteral<'ctx> {
  /// # Safety
  /// - `node` 必须非空并指向存活的 `AstExprTable`：本 crate 内唯一调用点是
  ///   [`AstVisitor::visit_expr_table`]（records/lint_table_literal.rs），以
  ///   `from_mut` 取自 visitor 分发出的 `&mut` 借用；其 `items` 数组及各
  ///   `key` 由 parser 成对写入 arena（cpp `visit(AstExprTable* node)` 经
  ///   `root->visit(&pass)` 分发的同一前提），在整个 lint pass 内只读存活。
  pub(crate) fn visit_ast_expr_table(&mut self, node: *mut AstExprTable) -> bool {
    // Safety: `node` 按函数级契约为分派借出节点转回的裸指针，非空对齐且
    // 存活；parse 结束后 arena 节点无人改写，共享借用止于本函数、无可变别名。
    let node = unsafe { &*node };
    let mut handle = self.context;
    let context = handle.get();
    let mut count = 0;
    for item in node.items.iter() {
      if item.kind == ItemKind::List {
        count += 1;
      }
    }
    let mut names: BTreeMap<Vec<u8>, u32> = BTreeMap::new();
    let mut indices: BTreeMap<i32, u32> = BTreeMap::new();
    for item in node.items.iter() {
      let Some(key_ref) = (unsafe { item.key.as_ref() }) else {
        continue;
      };
      match key_ref.as_expr_ref() {
        AstExprRef::ConstantString(expr) => {
          let key = expr.value.as_bytes().to_vec();
          let field = String::from_utf8_lossy(&key);
          if let Some(line) = names.get(&key).copied() {
            emit_warning(
              context,
              Code::TableLiteral,
              expr.base.base.location,
              format_args!(
                "Table field '{}' is a duplicate; previously defined at line {}",
                field, line
              ),
            );
          } else {
            names.insert(key, expr.base.base.location.begin.line + 1);
          }
        }
        AstExprRef::ConstantNumber(expr) => {
          let value = expr.value;
          if value >= 1.0 && value <= f64::from(count) && f64::from(value as i32) == value {
            emit_warning(
              context,
              Code::TableLiteral,
              expr.base.base.location,
              format_args!(
                "Table index {} is a duplicate; previously defined as a list entry",
                value as i32
              ),
            );
          } else if value >= 0.0 && value <= f64::from(i32::MAX) && f64::from(value as i32) == value {
            let index = value as i32;
            if let Some(line) = indices.get(&index).copied() {
              emit_warning(
                context,
                Code::TableLiteral,
                expr.base.base.location,
                format_args!(
                  "Table index {} is a duplicate; previously defined at line {}",
                  index, line
                ),
              );
            } else {
              indices.insert(index, expr.base.base.location.begin.line + 1);
            }
          }
        }
        _ => {}
      }
    }
    true
  }
  pub fn visit_ast_type(&mut self) -> bool {
    true
  }
  pub fn visit_ast_type_pack(&mut self) -> bool {
    true
  }
  /// # Safety
  /// - `node` 必须非空并指向存活的 `AstTypeTable`：本 crate 内唯一调用点是
  ///   [`AstVisitor::visit_type_table`]（records/lint_table_literal.rs），
  ///   `from_mut` 自 visitor 分派借用；其 `props` 数组由 parser 成对写入
  ///   arena 并在 lint pass 内只读存活。
  /// - 宿主 context 的 `module` 字段须为 null 或指向整个 `lint` 运行期间
  ///   存活的 `Module`（由 functions/lint.rs 自 `&Module` 参数存入；cpp 侧
  ///   `context->module->checkedInNewSolver` 直接解引用，本移植多了判空短路）。
  pub(crate) fn visit_ast_type_table(&mut self, node: *mut AstTypeTable) -> bool {
    // Safety: `node` 按函数级契约是分派借出 `AstTypeTable` 转回的裸指针，
    // 非空对齐；只共享读 `props`，parse 后 arena 无人改写。
    let node = unsafe { &*node };
    let mut handle = self.context;
    let context = handle.get();
    // Safety: `module` 按函数级契约为 null 或 `lint` 全程存活的 `Module`，
    // `is_null` 短路后只读 `checked_in_new_solver` 这一个 bool 字段。
    let checked_in_new_solver =
      !context.module.is_null() && unsafe { (*context.module).checked_in_new_solver };
    if checked_in_new_solver {
      #[derive(Clone, Copy)]
      struct Rec {
        access: AstTableAccess,
        location: Location,
      }
      let mut names: BTreeMap<AstName, Rec> = BTreeMap::new();
      for item in node.props.iter() {
        let name = item.name.as_str_or_empty();
        if let Some(rec) = names.get_mut(&item.name) {
          if (rec.access as u8 & item.access as u8) != 0 {
            if rec.access == item.access {
              emit_warning(
                context,
                Code::TableLiteral,
                item.location,
                format_args!(
                  "Table type field '{}' is a duplicate; previously defined at line {}",
                  name,
                  rec.location.begin.line + 1
                ),
              );
            } else if rec.access == AstTableAccess::ReadWrite {
              emit_warning(
                context,
                Code::TableLiteral,
                item.location,
                format_args!(
                  "Table type field '{}' is already read-write; previously defined at line {}",
                  name,
                  rec.location.begin.line + 1
                ),
              );
            } else if rec.access == AstTableAccess::Read {
              emit_warning(
                context,
                Code::TableLiteral,
                rec.location,
                format_args!(
                  "Table type field '{}' already has a read type defined at line {}",
                  name,
                  rec.location.begin.line + 1
                ),
              );
            } else if rec.access == AstTableAccess::Write {
              emit_warning(
                context,
                Code::TableLiteral,
                rec.location,
                format_args!(
                  "Table type field '{}' already has a write type defined at line {}",
                  name,
                  rec.location.begin.line + 1
                ),
              );
            }
          } else {
            rec.access |= item.access;
          }
        } else {
          names.insert(
            item.name,
            Rec {
              access: item.access,
              location: item.location,
            },
          );
        }
      }
      return true;
    }
    let mut names: BTreeMap<AstName, u32> = BTreeMap::new();
    for item in node.props.iter() {
      let name = item.name.as_str_or_empty();
      if let Some(line) = names.get(&item.name).copied() {
        emit_warning(
          context,
          Code::TableLiteral,
          item.location,
          format_args!(
            "Table type field '{}' is a duplicate; previously defined at line {}",
            name, line
          ),
        );
      } else {
        names.insert(item.name, item.location.begin.line + 1);
      }
    }
    true
  }
}
