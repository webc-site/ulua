use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::{type_function_type_id::TypeFunctionTypeId, type_id::TypeId};

pub type SeenTypes = DenseHashMap<TypeId, TypeFunctionTypeId>;
