use crate::{
  records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
  type_aliases::type_function_type_pack_id::TypeFunctionTypePackId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn operator_eq_type_function_type_pack_id(&self, tp: TypeFunctionTypePackId) -> bool {
    let as_tp = self.work_item_as_type_pack();
    as_tp.is_some() && unsafe { *as_tp.unwrap() } == tp
  }
}
