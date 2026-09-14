//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/TypeArena.h:15:type_arena`
//! Source: `Analysis/include/Luau/TypeArena.h` (TypeArena.h:15-27, hand-ported)

use alloc::string::String;
use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  module::Module, r#type::Type, type_pack_var::TypePackVar, typed_allocator::TypedAllocator,
};
#[derive(Debug)]
pub struct TypeArena {
  pub types: TypedAllocator<Type>,
  pub type_packs: TypedAllocator<TypePackVar>,

  /// Owning module, if any
  pub owning_module: *mut Module,

  pub collect_singleton_stats: bool,
  pub bool_singletons_minted: usize,
  pub str_singletons_minted: usize,
  pub unique_str_singletons_minted: DenseHashSet<Option<String>>,
}

impl Default for TypeArena {
  fn default() -> Self {
    Self {
      types: TypedAllocator::default(),
      type_packs: TypedAllocator::default(),
      owning_module: null_mut(),
      collect_singleton_stats: false,
      bool_singletons_minted: 0,
      str_singletons_minted: 0,
      unique_str_singletons_minted: DenseHashSet::new(None),
    }
  }
}
