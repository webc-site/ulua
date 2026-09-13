use crate::{
  records::find_user_type_function_blockers::FindUserTypeFunctionBlockers,
  type_aliases::type_pack_id::TypePackId,
};

impl FindUserTypeFunctionBlockers {
  pub fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    true
  }
}
