use crate::{
  records::{
    builtin_types::BuiltinTypes, normalized_extern_type::NormalizedExternType,
    normalized_function_type::NormalizedFunctionType, normalized_string_type::NormalizedStringType,
    type_ids::TypeIds,
  },
  type_aliases::{normalized_tyvars::NormalizedTyvars, type_id::TypeId},
};

#[derive(Debug, Clone)]
pub struct NormalizedType {
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub(crate) tops: TypeId,
  pub(crate) booleans: TypeId,
  pub(crate) extern_types: NormalizedExternType,
  pub(crate) errors: TypeId,
  pub(crate) nils: TypeId,
  pub(crate) numbers: TypeId,
  pub(crate) integers: TypeId,
  pub(crate) strings: NormalizedStringType,
  pub(crate) threads: TypeId,
  pub(crate) buffers: TypeId,
  pub(crate) tables: TypeIds,
  pub(crate) functions: NormalizedFunctionType,
  pub(crate) tyvars: NormalizedTyvars,
  pub(crate) is_cacheable: bool,
}

unsafe impl Send for NormalizedType {}
unsafe impl Sync for NormalizedType {}
