use crate::{records::type_pack_var::TypePackVar, type_aliases::type_pack_id::TypePackId};

pub fn as_mutable_type_pack_id(tp: TypePackId) -> *mut TypePackVar {
  tp as *mut TypePackVar
}

pub fn as_mutable(tp: TypePackId) -> *mut TypePackVar {
  as_mutable_type_pack_id(tp)
}

pub use as_mutable_type_pack_id as as_mutable_type_pack;
