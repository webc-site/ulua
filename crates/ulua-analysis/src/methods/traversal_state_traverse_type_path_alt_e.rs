//! Source: `Analysis/src/TypePath.cpp:559-593` (hand-ported)
use crate::{
  enums::pack_field::PackField,
  functions::{
    begin_type_pack::begin, end_type_pack::end,
    get_type_or_pack::get_type_or_pack_mut as get_type_or_pack,
    get_type_or_pack_alt_r::get_type_or_pack as get_type_or_pack_ty,
  },
  records::{function_type::FunctionType, traversal_state::TraversalState},
  type_aliases::type_pack_id::TypePackId,
};

impl TraversalState {
  pub fn traverse_type_path_pack_field(&mut self, field: PackField) -> bool {
    if self.check_invariants() {
      return false;
    }

    match field {
      PackField::Arguments | PackField::Returns => {
        let ft = get_type_or_pack_ty::<FunctionType>(&self.current);
        if !ft.is_null() {
          let target = if field == PackField::Arguments {
            unsafe { (*ft).arg_types }
          } else {
            unsafe { (*ft).ret_types }
          };
          self.update_current_type_pack_id(target);
          return true;
        }
        false
      }
      PackField::Tail => {
        let current_pack = get_type_or_pack::<TypePackId>(&self.current);
        if !current_pack.is_null() {
          let cp: TypePackId = unsafe { *current_pack };
          let mut it = begin(cp);
          while it.operator_ne(&end(cp)) {
            it.operator_inc();
          }

          if let Some(tail) = it.tail() {
            self.update_current_type_pack_id(tail);
            return true;
          }
        }
        false
      }
    }
  }
}
