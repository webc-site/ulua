use crate::{
  records::apply_type_function::ApplyTypeFunction, type_aliases::type_pack_id::TypePackId,
};

impl ApplyTypeFunction {
  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    self.type_pack_arguments.find(&tp).is_some()
  }
}
