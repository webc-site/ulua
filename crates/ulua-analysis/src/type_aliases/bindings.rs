use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::symbol::Symbol, type_aliases::def_id_def::DefId};
pub type Bindings = DenseHashMap<Symbol, DefId>;
