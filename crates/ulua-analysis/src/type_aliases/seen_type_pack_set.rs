use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::type_pair_hash::TypePairHash, type_aliases::type_pack_id::TypePackId};

pub type SeenTypePackSet = DenseHashMap<(TypePackId, TypePackId), bool, TypePairHash>;
