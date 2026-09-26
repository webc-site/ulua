use ulua_common::records::variant::Variant2;

use crate::{
  macros::variant_member,
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
  },
};

pub type TypeFunctionKind = Variant2<TypeFunctionTypeId, TypeFunctionTypePackId>;

variant_member! {
  /// `get_if<T>(&tfkind)` over the kind variant (TypeFunctionRuntimeBuilder.h:12).
  position TypeFunctionKindMember: TypeFunctionKind {
    0 => TypeFunctionTypeId,
    1 => TypeFunctionTypePackId,
  }
}
