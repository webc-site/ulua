use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, type_ids::TypeIds},
};

impl TypeIds {
  pub fn is_never(&self) -> bool {
    // std::all_of(begin(), end(), [](TypeId i) { return get<NeverType>(i) != nullptr; })
    self.order.iter().all(|&i| {
      // If each typeid is never, then I guess typeid's is also never?
      !get_type_id::<NeverType>(i).is_none()
    })
  }
}
