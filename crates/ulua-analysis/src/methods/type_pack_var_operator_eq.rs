use crate::{
  functions::are_equal_structural_type_equality::are_equal_seen_set_type_pack_var_type_pack_var,
  records::type_pack_var::TypePackVar, type_aliases::seen_set_structural_type_equality::SeenSet,
};

impl TypePackVar {
  pub fn type_pack_var_operator_eq(&self, rhs: &TypePackVar) -> bool {
    let mut seen = SeenSet::new();
    are_equal_seen_set_type_pack_var_type_pack_var(&mut seen, self, rhs)
  }
}
