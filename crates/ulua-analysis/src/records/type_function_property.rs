use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
#[derive(Debug, Clone)]
pub struct TypeFunctionProperty {
  pub(crate) read_ty: Option<TypeFunctionTypeId>,
  pub(crate) write_ty: Option<TypeFunctionTypeId>,
}
