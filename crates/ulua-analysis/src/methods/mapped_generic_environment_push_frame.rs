use core::ptr::null;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    mapped_generic_environment::MappedGenericEnvironment, mapped_generic_frame::MappedGenericFrame,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl MappedGenericEnvironment {
  pub fn push_frame(&mut self, generic_tps: &[TypePackId]) {
    let mut mappings: DenseHashMap<TypePackId, Option<TypePackId>> = DenseHashMap::new(null());
    for &tp in generic_tps.iter() {
      *mappings.get_or_insert(tp) = None;
    }
    let parent_scope_index = self.current_scope_index;
    let frame = MappedGenericFrame {
      mappings,
      parent_scope_index,
      children: DenseHashSet::new(0),
    };
    self.frames.push(frame);
    let new_frame_index = self.frames.len() - 1;
    if let Some(current_scope_index) = self.current_scope_index {
      self.frames[current_scope_index]
        .children
        .insert(new_frame_index);
    }
    self.current_scope_index = Some(new_frame_index);
  }
}
