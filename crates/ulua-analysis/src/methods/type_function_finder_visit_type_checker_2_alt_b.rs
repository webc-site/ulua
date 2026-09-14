use crate::{
  records::{
    type_function_finder::TypeFunctionFinder,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl TypeFunctionFinder {
  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    _instance: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.mentioned_function_packs.insert(tp);
    true
  }
}
