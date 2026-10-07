//! `mapped_generic_environment` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, variant::Variant3},
};

use crate::{
  functions::{follow_type_pack, get_type_pack},
  methods::subtyping_bind_generic::dense_hash_map_find_no_default,
  records::{
    generic_type_pack::GenericTypePack, mapped_generic_environment::MappedGenericEnvironment,
    mapped_generic_frame::MappedGenericFrame, not_bindable::NotBindable, unmapped::Unmapped,
  },
  type_aliases::{lookup_result::LookupResult, type_pack_id::TypePackId},
};

impl MappedGenericEnvironment {
  pub fn bind_generic(&mut self, generic_tp: TypePackId, bindee_tp: TypePackId) -> bool {
    // We shouldn't bind generic type packs to themselves
    if generic_tp == bindee_tp {
      return true;
    }

    if get_type_pack::get::<GenericTypePack>(generic_tp).is_none() {
      LUAU_ASSERT!(false);
      return false;
    }

    let lookup_result = self.lookup_generic_pack(generic_tp);
    if let Variant3::V1(unmapped) = lookup_result {
      *self.frames[unmapped.scope_index]
        .mappings
        .get_or_insert(generic_tp) = Some(bindee_tp);
      true
    } else {
      false
    }
  }
}

impl MappedGenericEnvironment {
  pub(crate) fn lookup_generic_pack(&self, generic_tp: TypePackId) -> LookupResult {
    let generic_tp = follow_type_pack::follow(generic_tp);

    let mut current_frame_index = self.current_scope_index;

    while let Some(index) = current_frame_index {
      let current_frame = &self.frames[index];
      if let Some(mapped_pack) =
        dense_hash_map_find_no_default(&current_frame.mappings, &generic_tp)
      {
        if let Some(tp) = *mapped_pack {
          return Variant3::V0(tp);
        } else {
          return Variant3::V1(Unmapped { scope_index: index });
        }
      }
      current_frame_index = current_frame.parent_scope_index;
    }

    if let Some(base_index) = self.current_scope_index {
      let base_frame = &self.frames[base_index];
      let mut to_check: Vec<usize> = base_frame.children.iter().copied().collect();

      while let Some(curr_index) = to_check.pop() {
        let current_frame = &self.frames[curr_index];
        if let Some(mapped_pack) =
          dense_hash_map_find_no_default(&current_frame.mappings, &generic_tp)
        {
          if let Some(tp) = *mapped_pack {
            return Variant3::V0(tp);
          } else {
            return Variant3::V1(Unmapped {
              scope_index: curr_index,
            });
          }
        }
        to_check.extend(current_frame.children.iter().copied());
      }
    }

    Variant3::V2(NotBindable)
  }
}

impl MappedGenericEnvironment {
  pub fn pop_frame(&mut self) {
    LUAU_ASSERT!(self.current_scope_index.is_some());
    if let Some(current_scope_index) = self.current_scope_index {
      let new_frame_index = self.frames[current_scope_index].parent_scope_index;
      self.current_scope_index = new_frame_index.unwrap_or(0_usize).into();
    }
  }
}

impl MappedGenericEnvironment {
  pub fn push_frame(&mut self, generic_tps: &[TypePackId]) {
    let mut mappings: DenseHashMap<TypePackId, Option<TypePackId>> = DenseHashMap::default();
    for &tp in generic_tps.iter() {
      *mappings.get_or_insert(tp) = None;
    }
    let parent_scope_index = self.current_scope_index;
    let frame = MappedGenericFrame {
      mappings,
      parent_scope_index,
      children: DenseHashSet::default(),
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
