use crate::{functions::parallel_mul_sat::parallel_mul_sat, records::cost::Cost};

impl Cost {
  pub fn operator_mul(&self, other: i32) -> Cost {
    Cost {
      model: parallel_mul_sat(self.model, other),
      constant: 0,
    }
  }
}

pub fn cost_operator_deref(self_: &Cost, other: i32) -> Cost {
  self_.operator_mul(other)
}
