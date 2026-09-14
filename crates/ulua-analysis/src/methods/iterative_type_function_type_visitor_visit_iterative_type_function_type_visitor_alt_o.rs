use crate::{
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    type_function_type_pack::TypeFunctionTypePack,
  },
  type_aliases::type_function_type_pack_id::TypeFunctionTypePackId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn visit_type_function_type_pack_id_type_function_type_pack(
    &mut self,
    tp: TypeFunctionTypePackId,
    _tftp: &TypeFunctionTypePack,
  ) -> bool {
    self.visit_type_function_type_pack_id(tp)
  }
}
