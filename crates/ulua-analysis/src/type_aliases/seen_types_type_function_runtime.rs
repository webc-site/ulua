use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;

pub type SeenTypes = DenseHashMap<TypeFunctionTypeId, TypeFunctionTypeId>;
