use ulua_common::records::variant::Variant3;

use crate::{
  records::{not_bindable::NotBindable, unmapped::Unmapped},
  type_aliases::type_pack_id::TypePackId,
};

pub type LookupResult = Variant3<TypePackId, Unmapped, NotBindable>;
