use alloc::{collections::BTreeMap, string::String};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::def_id_def::DefId;
pub type Props = DenseHashMap<DefId, BTreeMap<String, DefId>>;
