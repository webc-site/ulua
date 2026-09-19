use crate::{
  records::{
    iterative_type_visitor::IterativeTypeVisitor,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    _tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.visit_type_pack_id(tp)
  }
}
