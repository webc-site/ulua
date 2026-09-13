use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{any_type::AnyType, never_type::NeverType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn intersection_of_tops(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    let here_is_never = !get_type_id::<NeverType>(here).is_none();
    let there_is_never = !get_type_id::<NeverType>(there).is_none();
    let here_is_any = !get_type_id::<AnyType>(here).is_none();
    let there_is_any = !get_type_id::<AnyType>(there).is_none();

    if here_is_never || there_is_never {
      return unsafe { (*self.builtin_types).never_type };
    }

    if here_is_any || there_is_any {
      return unsafe { (*self.builtin_types).any_type };
    }

    unsafe { (*self.builtin_types).unknown_type }
  }
}
