use crate::{
  records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn operator_eq_type_function_type_id(&self, ty: TypeFunctionTypeId) -> bool {
    let as_ty = self.work_item_as_type();
    as_ty.is_some_and(|cur_ty_ptr| unsafe { *cur_ty_ptr == ty })
  }
}
