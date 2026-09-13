use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::type_pair_hash::TypePairHash, type_aliases::type_id::TypeId};
pub type SeenSet = DenseHashMap<(TypeId, TypeId), bool, TypePairHash>;
