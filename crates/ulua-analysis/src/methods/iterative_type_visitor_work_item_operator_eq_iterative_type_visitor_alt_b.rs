use crate::{
  records::iterative_type_visitor::IterativeTypeVisitor, type_aliases::type_pack_id::TypePackId,
};

impl IterativeTypeVisitor {
  pub fn operator_eq_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let as_tp = self.iterative_type_visitor_work_item_as_type_pack();
    as_tp.is_some() && unsafe { *as_tp.unwrap() } == tp
  }
}
