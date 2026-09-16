use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  functions::get_mutable_type::get_mutable_type_id,
  records::{free_type::FreeType, replace_generics::ReplaceGenerics, table_type::TableType},
  type_aliases::type_id::TypeId,
};

impl ReplaceGenerics {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(self.is_dirty_type_id(ty));

    let log = self.base.base.log;
    let level = self.level;
    let scope = self.scope;

    if FFlag::LuauReplacerIsSolverAgnostic.get() {
      let ttv = unsafe { (*log).txn_log_get_mutable::<TableType, TypeId>(ty) };
      if !ttv.is_null() {
        let ttv = unsafe { &*ttv };
        let mut clone =
          TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &ttv.props,
            ttv.indexer,
            level,
            scope,
            TableState::Free,
          );
        clone.definition_module_name = ttv.definition_module_name.clone();
        clone.definition_location = ttv.definition_location;
        self.base.add_type(clone)
      } else {
        // arena->freshType(builtinTypes, scope, level)
        let builtins = unsafe { &*self.builtin_types };
        let free_type = FreeType {
          scope,
          level,
          lower_bound: builtins.never_type,
          upper_bound: builtins.unknown_type,
          ..FreeType::default()
        };
        self.base.add_type(free_type)
      }
    } else {
      let ttv = unsafe { (*log).txn_log_get_mutable::<TableType, TypeId>(ty) };
      if !ttv.is_null() {
        let ttv = unsafe { &*ttv };
        let mut clone =
          TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &ttv.props,
            ttv.indexer,
            level,
            scope,
            TableState::Free,
          );
        clone.definition_module_name = ttv.definition_module_name.clone();
        clone.definition_location = ttv.definition_location;
        self.base.add_type(clone)
      } else if FFlag::LuauSolverV2.get() {
        let builtins = unsafe { &*self.builtin_types };
        let free_type = FreeType {
          scope,
          lower_bound: builtins.never_type,
          upper_bound: builtins.unknown_type,
          ..FreeType::default()
        };
        let res = self.base.add_type(free_type);
        if let Some(ft) = get_mutable_type_id::<FreeType>(res) {
          ft.level = level;
        }
        res
      } else {
        // arena->freshType(builtinTypes, scope, level)
        let builtins = unsafe { &*self.builtin_types };
        let free_type = FreeType {
          scope,
          level,
          lower_bound: builtins.never_type,
          upper_bound: builtins.unknown_type,
          ..FreeType::default()
        };
        self.base.add_type(free_type)
      }
    }
  }
}
