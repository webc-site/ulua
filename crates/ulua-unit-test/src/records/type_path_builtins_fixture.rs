use ulua_analysis::{records::type_arena::TypeArena, type_aliases::type_pack_id::TypePackId};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
};

pub struct TypePathBuiltinsFixture {
  pub base: BuiltinsFixture,
  pub sff1: ScopedFastFlag,
  pub arena: TypeArena,
  pub empty_map_deprecated: DenseHashMap<TypePackId, TypePackId>,
}
