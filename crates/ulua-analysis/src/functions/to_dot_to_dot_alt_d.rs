extern crate alloc;

use alloc::string::String;

use crate::{
  functions::to_dot_to_dot_alt_b, records::to_dot_options::ToDotOptions,
  type_aliases::type_pack_id::TypePackId,
};

pub fn to_dot(tp: TypePackId) -> String {
  to_dot_to_dot_alt_b::to_dot(tp, &ToDotOptions::default())
}
