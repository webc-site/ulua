use crate::{
  functions::are_equal_structural_type_equality_alt_e::are_equal_seen_set_type_item_type_item,
  records::r#type::Type, type_aliases::seen_set_structural_type_equality::SeenSet,
};

impl Type {
  pub fn operator_eq(&self, rhs: &Type) -> bool {
    let mut seen: SeenSet = Default::default();
    are_equal_seen_set_type_item_type_item(&mut seen, self, rhs)
  }
}
