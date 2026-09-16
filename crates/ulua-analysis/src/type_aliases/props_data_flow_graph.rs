use alloc::{collections::BTreeMap, string::String};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::def::Def;
pub type Props = DenseHashMap<*const Def, BTreeMap<String, *const Def>>;
