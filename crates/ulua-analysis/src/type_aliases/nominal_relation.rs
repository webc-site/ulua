use ulua_common::records::variant::Variant2;

use crate::records::{klass::Klass, obj::Obj};

pub type NominalRelation = Variant2<Obj, Klass>;
