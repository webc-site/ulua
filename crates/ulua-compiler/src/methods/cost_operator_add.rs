use crate::{
  functions::{parallel_add_sat::parallel_add_sat, parallel_mul_sat::parallel_mul_sat},
  records::cost::Cost,
};

impl Cost {
  pub fn operator_add(&self, other: &Cost) -> Cost {
    // C++ operator+ default-constructs `Cost result` (constant = 0) and only sets
    // model; the constant mask is intentionally dropped here, unlike fold().
    Cost {
      model: parallel_add_sat(self.model, other.model),
      constant: 0,
    }
  }

  pub fn operator_mul(&self, other: i32) -> Cost {
    Cost {
      model: parallel_mul_sat(self.model, other),
      constant: 0,
    }
  }
}
