use alloc::vec::Vec;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  methods::fresh_normalized_type::fresh_normalized_type,
  records::{normalized_type::NormalizedType, normalizer::Normalizer},
  type_aliases::{seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId},
};
impl Normalizer {
  pub fn normalize_intersections(
    &mut self,
    intersections: &Vec<TypeId>,
    out_type: &mut NormalizedType,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    if self.arena.is_none() {
      // C++: sharedState->iceHandler->ice("Normalizing types outside a module")
      // 契约：本分支仅在 arena 哨兵为 None（模块外归一化）时触发；`self.shared_state`
      // 为构造/接线期注入的 Option<Handle<UnifierSharedState>>（C++ `sharedState`
      // 成员直译，存活整次归一化，shared_state_ref 断言接线），其 `ice_handler`
      // 字段由 UnifierSharedState 构造期写入 Frontend 持有的 InternalErrorReporter，
      // 同样非空存活；`ice_string` 只读上报，无别名冲突。
      let ice = self.shared_state_ref().ice_handler;
      unsafe {
        (*ice).ice_string("Normalizing types outside a module");
      }
    }

    self.consume_fuel();

    // NormalizedType norm{builtinTypes}; norm.tops = builtinTypes->unknownType;
    let mut norm = fresh_normalized_type(self.builtin_types);
    // Safety: `self.builtin_types.as_ptr()` 是构造期接线的非空 BuiltinTypes 会话单例（C++ NotNull），
    // 此处只读拷贝 unknown_type 句柄。
    norm.tops = self.builtin_types.get().unknown_type;

    for &ty in intersections {
      let res = self.intersect_normal_with_ty(&mut norm, ty, seen_table_prop_pairs, seen_set);
      if res != NormalizationResult::True {
        return res;
      }
    }

    let res = self.union_normals(out_type, &norm, -1);
    if res != NormalizationResult::True {
      return res;
    }

    NormalizationResult::True
  }
}
