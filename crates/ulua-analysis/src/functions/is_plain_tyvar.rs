use crate::{
  functions::get_type,
  records::{
    blocked_type::BlockedType, free_type::FreeType, generic_type::GenericType,
    pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_plain_tyvar(ty: TypeId) -> bool {
  get_type::get::<FreeType>(ty).is_some()
    || get_type::get::<GenericType>(ty).is_some()
    || get_type::get::<BlockedType>(ty).is_some()
    || get_type::get::<PendingExpansionType>(ty).is_some()
    || get_type::get::<TypeFunctionInstanceType>(ty).is_some()
}
