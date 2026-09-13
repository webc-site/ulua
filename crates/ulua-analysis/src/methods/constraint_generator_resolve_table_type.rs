use alloc::{collections::BTreeMap, string::String};
use core::ffi::CStr;

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::{ast_type::AstType, ast_type_table::AstTypeTable, location::Location},
};
use ulua_common::FFlag;

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{get_mutable_type::get_mutable_type_id, polarity_of_access::polarity_of_access},
  records::{
    constraint_generator::ConstraintGenerator, generic_error::GenericError,
    property_type::Property, scope::Scope, table_indexer::TableIndexer, table_type::TableType,
  },
  type_aliases::{name_type::Name, type_error_data::TypeErrorData, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn resolve_table_type(
    &mut self,
    scope: *mut Scope,
    _ty: *mut AstType,
    tab: *mut AstTypeTable,
    in_type_arguments: bool,
    _replace_error_with_fresh: bool,
  ) -> TypeId {
    let mut props: BTreeMap<Name, Property> = BTreeMap::new();
    let mut indexer: Option<TableIndexer> = None;

    let p = self.polarity;

    // SAFETY: tab is a valid pointer to AstTypeTable
    let tab_ref = unsafe { &*tab };
    let props_array = &tab_ref.props;

    for prop_ref in props_array.as_slice() {
      let name: String = unsafe {
        CStr::from_ptr(prop_ref.name.value)
          .to_string_lossy()
          .into_owned()
      };
      let prop_access = prop_ref.access;

      // Set the polarity for the inner type
      self.polarity = polarity_of_access(prop_access, p);
      let cur_polarity = self.polarity;

      let prop_ty = self.resolve_type(
        scope,
        prop_ref.r#type,
        in_type_arguments,
        false,
        cur_polarity,
      );

      let prop_ref_mut = props.entry(name).or_default();

      prop_ref_mut.type_location = Some(prop_ref.location);

      match prop_access {
        AstTableAccess::ReadWrite => {
          prop_ref_mut.read_ty = Some(prop_ty);
          prop_ref_mut.write_ty = Some(prop_ty);
        }
        AstTableAccess::Read => {
          prop_ref_mut.read_ty = Some(prop_ty);
        }
        AstTableAccess::Write => {
          prop_ref_mut.write_ty = Some(prop_ty);
        }
      }
    }

    if !tab_ref.indexer.is_null() {
      let ast_indexer = unsafe { &*tab_ref.indexer };
      let indexer_access = ast_indexer.access;

      if indexer_access == AstTableAccess::Read {
        if !FFlag::LuauReadOnlyIndexers.get() {
          self.report_error(
            ast_indexer.access_location.unwrap_or(Location::new(
              ast_indexer.location.begin,
              ast_indexer.location.begin,
            )),
            TypeErrorData::GenericError(GenericError::new(
              "read keyword is illegal here".to_string(),
            )),
          );
        } else {
          self.polarity = p;
          let cur_polarity = self.polarity;
          let index_ty = self.resolve_type(
            scope,
            ast_indexer.index_type,
            in_type_arguments,
            false,
            cur_polarity,
          );
          let cur_polarity2 = self.polarity;
          let result_ty = self.resolve_type(
            scope,
            ast_indexer.result_type,
            in_type_arguments,
            false,
            cur_polarity2,
          );
          indexer = Some(TableIndexer {
            index_type: index_ty,
            index_result_type: result_ty,
            is_read_only: true,
          });
        }
      } else if indexer_access == AstTableAccess::Write {
        self.report_error(
          ast_indexer.access_location.unwrap_or(Location::new(
            ast_indexer.location.begin,
            ast_indexer.location.begin,
          )),
          TypeErrorData::GenericError(GenericError::new(
            "write keyword is illegal here".to_string(),
          )),
        );
      } else if indexer_access == AstTableAccess::ReadWrite {
        self.polarity = Polarity::Mixed;
        let cur_polarity = self.polarity;
        let index_ty = self.resolve_type(
          scope,
          ast_indexer.index_type,
          in_type_arguments,
          false,
          cur_polarity,
        );
        let cur_polarity2 = self.polarity;
        let result_ty = self.resolve_type(
          scope,
          ast_indexer.result_type,
          in_type_arguments,
          false,
          cur_polarity2,
        );
        indexer = Some(TableIndexer {
          index_type: index_ty,
          index_result_type: result_ty,
          is_read_only: false,
        });
      }
      // else: Unexpected property access - handled by C++ ice
    }

    self.polarity = p;

    let table_ty = unsafe {
      let tt = TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
        &props,
        indexer,
        (*scope).level,
        scope,
        TableState::Sealed,
      );

      (*self.arena).add_type(tt)
    };

    // table_ty 刚由 add_type(TableType) 分配，必命中；对照 C++:4673-4676
    // `TableType* ttv = getMutable<TableType>(tableTy); ttv->definition...`
    let ttv = get_mutable_type_id::<TableType>(table_ty).unwrap();
    // SAFETY: tab 为 AST 节点指针，生命周期与模块 AST 同寿。
    unsafe {
      ttv.definition_module_name = self.module.as_ref().unwrap().name.clone();
      ttv.definition_location = (*tab).base.base.location;
    }

    table_ty
  }
}
