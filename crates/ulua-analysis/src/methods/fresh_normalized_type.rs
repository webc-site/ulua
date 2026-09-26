//! C++ `NormalizedType{builtinTypes}` 构造 (`Analysis/src/Normalize.cpp:1783,3320`
//! `std::make_unique<NormalizedType>(NormalizedType{builtinTypes})`) 的展开字面量,
//! 原先在 normalizer_union_normal_with_ty / normalizer_intersect_normals /
//! normalizer_intersect_normal_with_ty 各复制一份, 收敛于此。

use alloc::collections::BTreeMap;

use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes, normalized_extern_type::NormalizedExternType,
  normalized_function_type::NormalizedFunctionType, normalized_string_type::NormalizedStringType,
  normalized_type::NormalizedType, type_ids::TypeIds,
};

pub(crate) fn fresh_normalized_type(builtin_types: Handle<BuiltinTypes>) -> NormalizedType {
  // Safety: 句柄契约保证目标非空且调用期内有效（C++ sharedState 生命周期）；
  // get() 只读 Copy 的 never_type 句柄。
  let never_type = builtin_types.get().never_type;
  NormalizedType {
    builtin_types,
    tops: never_type,
    booleans: never_type,
    extern_types: NormalizedExternType {
      extern_types: BTreeMap::new(),
      shape_extensions: TypeIds::new(),
      ordering: Vec::new(),
    },
    errors: never_type,
    nils: never_type,
    numbers: never_type,
    integers: never_type,
    strings: NormalizedStringType::NEVER,
    threads: never_type,
    buffers: never_type,
    tables: TypeIds::new(),
    functions: NormalizedFunctionType {
      is_top: false,
      parts: TypeIds::new(),
    },
    tyvars: BTreeMap::new(),
    is_cacheable: true,
  }
}
