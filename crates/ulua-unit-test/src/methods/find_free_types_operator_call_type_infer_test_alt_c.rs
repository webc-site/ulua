use ulua_analysis::{
  records::free_type_pack::FreeTypePack, type_aliases::type_pack_id::TypePackId,
};

use crate::records::find_free_types::FindFreeTypes;

impl FindFreeTypes {
  pub fn operator_call_mut_2(&mut self, _id: TypePackId, _free: FreeTypePack) -> bool {
    self.found_one = true;
    false
  }
}
