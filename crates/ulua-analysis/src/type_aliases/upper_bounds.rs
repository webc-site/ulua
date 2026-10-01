use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::type_id::TypeId;

pub type UpperBounds = DenseHashMap<TypeId, Vec<(Location, TypeId)>>;
