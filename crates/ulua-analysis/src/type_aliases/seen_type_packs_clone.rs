use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::type_pack_id::TypePackId;
pub type SeenTypePacks = DenseHashMap<TypePackId, TypePackId>;
