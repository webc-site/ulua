use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::type_function_cloner::TypeFunctionCloner,
  type_aliases::{
    type_function_kind::TypeFunctionKind, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
  },
};

impl TypeFunctionCloner {
  pub fn clone_children_type_function_kind_type_function_kind(
    &mut self,
    kind: &TypeFunctionKind,
    tfkind: &TypeFunctionKind,
  ) {
    if let Some(ty) = TypeFunctionKind::get_if::<TypeFunctionTypeId>(kind)
      && let Some(tfty) = TypeFunctionKind::get_if::<TypeFunctionTypeId>(tfkind)
    {
      unsafe { self.clone_children_type_function_type_id_type_function_type_id(*ty, *tfty) };
      return;
    }

    if let Some(tp) = TypeFunctionKind::get_if::<TypeFunctionTypePackId>(kind)
      && let Some(tftp) = TypeFunctionKind::get_if::<TypeFunctionTypePackId>(tfkind)
    {
      unsafe {
        self.clone_children_type_function_type_pack_id_type_function_type_pack_id(*tp, *tftp)
      };
      return;
    }

    LUAU_ASSERT!(false);
  }
}
