use alloc::vec::Vec;

use ulua_common::records::variant::Variant3;

use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  methods::subtyping_bind_generic::dense_hash_map_find_no_default,
  records::{
    mapped_generic_environment::MappedGenericEnvironment, not_bindable::NotBindable,
    unmapped::Unmapped,
  },
  type_aliases::{lookup_result::LookupResult, type_pack_id::TypePackId},
};
impl MappedGenericEnvironment {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn lookup_generic_pack(&self, generic_tp: TypePackId) -> LookupResult {
    let generic_tp = unsafe { follow_type_pack_id(generic_tp) };

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

    Variant3::V2(NotBindable { _unused: None })
  }
}
