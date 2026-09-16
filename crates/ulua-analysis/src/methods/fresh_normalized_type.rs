//! C++ `NormalizedType{builtinTypes}` 构造 (`Analysis/src/Normalize.cpp:1783,3320`
//! `std::make_unique<NormalizedType>(NormalizedType{builtinTypes})`) 的展开字面量,
//! 原先在 normalizer_union_normal_with_ty / normalizer_intersect_normals /
//! normalizer_intersect_normal_with_ty 各复制一份, 收敛于此。

use alloc::collections::BTreeMap;

use crate::records::{
  builtin_types::BuiltinTypes, normalized_extern_type::NormalizedExternType,
  normalized_function_type::NormalizedFunctionType, normalized_string_type::NormalizedStringType,
  normalized_type::NormalizedType, type_ids::TypeIds,
};

pub(crate) fn fresh_normalized_type(builtin_types: *mut BuiltinTypes) -> NormalizedType {
  // SAFETY: builtin_types 指向调用方保证有效的 BuiltinTypes（C++ sharedState 生命周期）
  let never_type = unsafe { (*builtin_types).never_type };
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
