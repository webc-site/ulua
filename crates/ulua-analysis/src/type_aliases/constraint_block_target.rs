use ulua_common::records::variant::Variant3;

use crate::{
  records::constraint::Constraint,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub type ConstraintBlockTarget = Variant3<TypeId, TypePackId, *const Constraint>;
