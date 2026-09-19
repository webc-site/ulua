use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::{
  type_function_type_pack_id::TypeFunctionTypePackId, type_pack_id::TypePackId,
};

pub type SeenTypePacks = DenseHashMap<TypeFunctionTypePackId, TypePackId>;
