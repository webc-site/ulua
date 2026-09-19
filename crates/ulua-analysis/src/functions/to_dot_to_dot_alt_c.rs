extern crate alloc;

use alloc::string::String;

use crate::{
  functions::to_dot_to_dot, records::to_dot_options::ToDotOptions, type_aliases::type_id::TypeId,
};

pub fn to_dot(ty: TypeId) -> String {
  to_dot_to_dot::to_dot(ty, &ToDotOptions::default())
}
