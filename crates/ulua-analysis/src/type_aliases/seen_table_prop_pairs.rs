use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::type_id_pair_hash::TypeIdPairHash, type_aliases::type_id::TypeId};
pub type SeenTablePropPairs = DenseHashMap<(TypeId, TypeId), bool, TypeIdPairHash>;
