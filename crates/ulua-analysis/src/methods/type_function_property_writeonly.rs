use crate::{
  records::type_function_property::TypeFunctionProperty,
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl TypeFunctionProperty {
  #[inline]
  pub fn writeonly(ty: TypeFunctionTypeId) -> Self {
    let mut p = TypeFunctionProperty {
      read_ty: None,
      write_ty: None,
    };
    p.write_ty = Some(ty);
    p
  }
}
