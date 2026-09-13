use crate::{records::iterative_type_visitor::IterativeTypeVisitor, type_aliases::type_id::TypeId};

impl IterativeTypeVisitor {
  pub fn operator_eq_type_id(&mut self, _ty: TypeId) -> bool {
    let as_ty = self.iterative_type_visitor_work_item_as_type();
    as_ty.is_some_and(|cur_ty_ptr| unsafe { *cur_ty_ptr == _ty })
  }
}
