//! Source: `Analysis/src/TypePath.cpp:469-549` (hand-ported)
use crate::{
  enums::type_field::TypeField,
  functions::{
    follow_type::follow_type_id, get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_type_alt_j::get_type_id, get_type_or_pack as get_type_or_pack_crate,
    get_type_or_pack_alt_r::get_type_or_pack as get_type_or_pack_ty,
    get_type_or_pack_alt_s::get_type_or_pack_mut_2,
  },
  records::{
    extern_type::ExternType, free_type::FreeType, metatable_type::MetatableType,
    negation_type::NegationType, table_indexer::TableIndexer, table_type::TableType,
    traversal_state::TraversalState, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_id::TypeId,
};
impl TraversalState {
  pub fn traverse_type_path_type_field(&mut self, field: TypeField) -> bool {
    if self.check_invariants() {
      return false;
    }

    match field {
      TypeField::Table => {
        let mt = get_type_or_pack_ty::<MetatableType>(&self.current);
        if !mt.is_null() {
          self.update_current_type_id(unsafe { (*mt).table });
          return true;
        }
        false
      }
      TypeField::Metatable => {
        let current_type = get_type_or_pack_crate::get_type_or_pack_mut::<TypeId>(&self.current);
        if !current_type.is_null()
          && let Some(mt) =
            get_metatable_type_id_not_null_builtin_types(unsafe { *current_type }, unsafe {
              &*self.builtin_types
            })
        {
          self.update_current_type_id(mt);
          return true;
        }
        false
      }
      TypeField::LowerBound | TypeField::UpperBound => {
        let ft = get_type_or_pack_ty::<FreeType>(&self.current);
        if !ft.is_null() {
          let target = if field == TypeField::LowerBound {
            unsafe { (*ft).lower_bound }
          } else {
            unsafe { (*ft).upper_bound }
          };
          self.update_current_type_id(target);
          return true;
        }
        false
      }
      TypeField::IndexLookup | TypeField::IndexResult => {
        let mut indexer: Option<&TableIndexer> = None;

        let tt = get_type_or_pack_ty::<TableType>(&self.current);
        if !tt.is_null() && unsafe { (*tt).indexer.is_some() } {
          indexer = unsafe { (*tt).indexer.as_ref() };
        } else {
          let mt = get_type_or_pack_ty::<MetatableType>(&self.current);
          if !mt.is_null() {
            let mt_tab = unsafe { get_type_id::<TableType>(follow_type_id((*mt).table)) };
            if let Some(tab) = mt_tab
              && tab.indexer.is_some()
            {
              indexer = tab.indexer.as_ref();
            } else {
              let mt_mt = unsafe { get_type_id::<TableType>(follow_type_id((*mt).metatable)) };
              if let Some(mt_tab) = mt_mt
                && mt_tab.indexer.is_some()
              {
                indexer = mt_tab.indexer.as_ref();
              }
            }
          } else {
            // Note: we don't appear to walk the class hierarchy for
            // indexers
            let ct = get_type_or_pack_ty::<ExternType>(&self.current);
            if !ct.is_null() && unsafe { (*ct).indexer.is_some() } {
              indexer = unsafe { (*ct).indexer.as_ref() };
            }
          }
        }

        if let Some(indexer) = indexer {
          let target = if field == TypeField::IndexLookup {
            indexer.index_type
          } else {
            indexer.index_result_type
          };
          self.update_current_type_id(target);
          return true;
        }
        false
      }
      TypeField::Negated => {
        let nt = get_type_or_pack_ty::<NegationType>(&self.current);
        if !nt.is_null() {
          self.update_current_type_id(unsafe { (*nt).ty });
          return true;
        }
        false
      }
      TypeField::Variadic => {
        let vtp = get_type_or_pack_mut_2::<VariadicTypePack>(&self.current);
        if !vtp.is_null() {
          self.update_current_type_id(unsafe { (*vtp).ty });
          return true;
        }
        false
      }
    }
  }
}
