extern crate alloc;

use alloc::string::String;

use crate::{
  functions::to_dot_to_dot::to_dot_type_id_to_dot_options, records::to_dot_options::ToDotOptions,
  type_aliases::type_id::TypeId,
};

pub fn to_dot(ty: TypeId) -> String {
  to_dot_type_id_to_dot_options(ty, &ToDotOptions::default())
}
