use alloc::{collections::BTreeMap, string::String};
use core::ptr::from_ref;

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::{ast_type::AstType, ast_type_table::AstTypeTable, location::Location},
};
use ulua_common::fflag;

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{get_mutable_type, polarity_of_access::polarity_of_access},
  records::{
    arena_handle::alias_ref, constraint_generator::ConstraintGenerator,
    generic_error::GenericError, property_type::Property, table_indexer::TableIndexer,
    table_type::TableType,
  },
  type_aliases::{
    name_type::Name, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl ConstraintGenerator {
  /// C++ `resolveTableType(NotNull<Scope*>, AstTypeTable*, bool, bool)`
  /// （由 resolve_type_inner 的 RTTI 命中分支传入存活的 `tab` 共享引用）。
  /// 形参链已引用化：`scope` 为调用方 `ScopePtr` 共享借用，`tab` 全程只读。
  pub fn resolve_table_type(
    &mut self,
    scope: &ScopePtr,
    _ty: &AstType,
    tab: &AstTypeTable,
    in_type_arguments: bool,
    _replace_error_with_fresh: bool,
  ) -> TypeId {
    let mut props: BTreeMap<Name, Property> = BTreeMap::new();
    let mut indexer: Option<TableIndexer> = None;

    let p = self.polarity;

    for prop_ref in tab.props.as_slice() {
      let name: String = prop_ref.name.as_str_or_empty().to_string();
      let prop_access = prop_ref.access;

      // Set the polarity for the inner type
      self.polarity = polarity_of_access(prop_access, p);
      let cur_polarity = self.polarity;

      let prop_ty = self.resolve_type(
        scope,
        alias_ref(prop_ref.r#type),
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

    if let Some(ast_indexer) = tab.indexer.get() {
      let indexer_access = ast_indexer.access;

      if indexer_access == AstTableAccess::Read {
        if !fflag::LuauReadOnlyIndexers.get() {
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
            ast_indexer.index_type.get(),
            in_type_arguments,
            false,
            cur_polarity,
          );
          let cur_polarity2 = self.polarity;
          let result_ty = self.resolve_type(
            scope,
            ast_indexer.result_type.get(),
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
          ast_indexer.index_type.get(),
          in_type_arguments,
          false,
          cur_polarity,
        );
        let cur_polarity2 = self.polarity;
        let result_ty = self.resolve_type(
          scope,
          ast_indexer.result_type.get(),
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

    // `scope` 为调用方 `ScopePtr` 共享借用的 Arc<Scope> 堆分配（非空、地址稳定），
    // level 为 Copy 只读；TableType 记录的 scope 字段保持裸指针布局（记录布局不改），
    // 仅在此处一次性还原地址。arena 句柄为构造期接线的会话存活 arena，add_type
    // 只在 bump 块尾追加新节点，tt 全部由上方本地 props/indexer 构造。
    let tt = TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
      &props,
      indexer,
      scope.level,
      from_ref(&**scope).cast_mut(),
      TableState::Sealed,
    );

    let table_ty = self.arena.get_mut().add_type(tt);

    // table_ty 刚由 add_type(TableType) 分配，必命中；对照 C++:4673-4676
    // `TableType* ttv = getMutable<TableType>(tableTy); ttv->definition...`
    let ttv = get_mutable_type::get_mutable::<TableType>(table_ty)
      .expect("table_ty 刚由 add_type(TableType) 分配，必命中（cpp:4673）");
    // ttv 取自上方刚 add_type 的 TableType——RTTI 命中且 expect 保证非空，节点驻留
    // 会话 arena、尚无其它活动借用，两处字段写为唯一写窗口；tab 为 RTTI 确认的
    // parse arena 存活节点（此处仅读 base 链上的 location）。
    ttv.definition_module_name = self
      .module
      .as_ref()
      .expect("ConstraintGenerator 构造期以 ModulePtr 接线并 LUAU_ASSERT(is_some)，恒为 Some")
      .name
      .clone();
    ttv.definition_location = tab.base.base.location;

    table_ty
  }
}
