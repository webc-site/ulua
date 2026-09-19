//! Source: `Analysis/src/TypePath.cpp:324-391` (hand-ported)
use alloc::string::String;

use crate::{
  functions::{
    get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_type_or_pack::get_type_or_pack_mut as get_type_or_pack,
    get_type_or_pack_alt_r::get_type_or_pack as get_type_or_pack_ty,
    lookup_extern_type_prop::lookup_extern_type_prop,
  },
  records::{
    extern_type::ExternType, metatable_type::MetatableType,
    property_type::Property as PropertyType, property_type_path::Property, table_type::TableType,
    traversal_state::TraversalState,
  },
  type_aliases::type_id::TypeId,
};
impl TraversalState {
  pub fn traverse_type_path_property(&mut self, property: &Property) -> bool {
    let current_type = get_type_or_pack::<TypeId>(&self.current);
    if current_type.is_null() {
      return false;
    }

    if self.check_invariants() {
      return false;
    }

    let mut prop: Option<&PropertyType> = None;

    let t = get_type_or_pack_ty::<TableType>(&self.current);
    if !t.is_null() {
      if let Some(found) = unsafe { (*t).props.get(&property.name) } {
        prop = Some(found);
      }
    } else {
      let c = get_type_or_pack_ty::<ExternType>(&self.current);
      if !c.is_null() {
        prop = lookup_extern_type_prop(unsafe { &*c }, &property.name);
      } else {
        // For a metatable type, the table takes priority; check that
        // before falling through to the metatable entry below.
        let m = get_type_or_pack_ty::<MetatableType>(&self.current);
        if !m.is_null() {
          let pinned = self.current.clone();
          self.update_current_type_id(unsafe { (*m).table });

          if self.traverse_type_path_property(property) {
            return true;
          }

          // Restore the old current type if we didn't traverse the
          // metatable successfully; we'll use the next branch to
          // address this.
          self.current = pinned;
        }
      }
    }

    if prop.is_none() {
      // `current_type` was captured before any `update_current`; re-read
      // the current type to mirror the C++ `*currentType` dereference.
      let cur_ty = get_type_or_pack::<TypeId>(&self.current);
      if !cur_ty.is_null()
        && let Some(m) = get_metatable_type_id_not_null_builtin_types(unsafe { *cur_ty }, unsafe {
          &*self.builtin_types
        })
      {
        // Weird: rather than use findMetatableEntry, which requires a
        // lot of stuff that we don't have and don't want to pull in,
        // we use the path traversal logic to grab __index and then
        // re-enter the lookup logic there.
        self.update_current_type_id(m);

        // C++ `TypePath::Property::read("__index")` — a read-typed
        // path component for the `__index` property.
        let index_prop = Property {
          name: String::from("__index"),
          is_read: true,
        };
        if !self.traverse_type_path_property(&index_prop) {
          return false;
        }

        return self.traverse_type_path_property(property);
      }
    }

    if let Some(prop) = prop {
      let maybe_type = if property.is_read {
        prop.read_ty
      } else {
        prop.write_ty
      };

      if let Some(ty) = maybe_type {
        self.update_current_type_id(ty);
        return true;
      }
    }

    false
  }
}
