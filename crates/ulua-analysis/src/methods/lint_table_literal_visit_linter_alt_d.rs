use alloc::collections::BTreeMap;
use core::ffi::CStr;

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::{ast_name::AstName, ast_type_table::AstTypeTable, location::Location},
};
use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_table_literal::LintTableLiteral};
impl LintTableLiteral {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_table(&mut self, node: *mut AstTypeTable) -> bool {
    let node = unsafe { &*node };
    let checked_in_new_solver = unsafe {
      !(*self.context).module.is_null() && (*(*self.context).module).checked_in_new_solver
    };

    if checked_in_new_solver {
      #[derive(Clone, Copy)]
      struct Rec {
        access: AstTableAccess,
        location: Location,
      }

      let mut names: BTreeMap<AstName, Rec> = BTreeMap::new();

      for item in node.props.iter() {
        let name = unsafe { CStr::from_ptr(item.name.value).to_string_lossy() };

        if let Some(rec) = names.get_mut(&item.name) {
          if (rec.access as u8 & item.access as u8) != 0 {
            if rec.access == item.access {
              emit_warning(
                unsafe { &mut *self.context },
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
                unsafe { &mut *self.context },
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
                unsafe { &mut *self.context },
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
                unsafe { &mut *self.context },
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
            rec.access = match rec.access as u8 | item.access as u8 {
              1 => AstTableAccess::Read,
              2 => AstTableAccess::Write,
              _ => AstTableAccess::ReadWrite,
            };
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
      let name = unsafe { CStr::from_ptr(item.name.value).to_string_lossy() };

      if let Some(line) = names.get(&item.name).copied() {
        emit_warning(
          unsafe { &mut *self.context },
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
