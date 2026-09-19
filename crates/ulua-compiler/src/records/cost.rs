//! Source: `Compiler/src/CostModel.cpp:50-101`

use crate::functions::{parallel_add_sat::parallel_add_sat, parallel_mul_sat::parallel_mul_sat};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Cost {
  pub(crate) model: u64,
  pub(crate) constant: u64,
}

impl Cost {
  pub const K_LITERAL: u64 = !0u64;

  pub fn new(cost: i32, constant: u64) -> Self {
    Self {
      model: if cost < 0x7f { cost as u64 } else { 0x7f },
      constant,
    }
  }

  pub fn add(&self, other: &Cost) -> Cost {
    Cost {
      model: parallel_add_sat(self.model, other.model),
      constant: 0,
    }
  }

  pub fn add_assign(&mut self, other: &Cost) {
    self.model = parallel_add_sat(self.model, other.model);
    self.constant = 0;
  }

  pub fn mul(&self, other: i32) -> Cost {
    Cost {
      model: parallel_mul_sat(self.model, other),
      constant: 0,
    }
  }

  pub fn fold(x: &Cost, y: &Cost) -> Cost {
    let new_model = parallel_add_sat(x.model, y.model);
    let newconstant: u64 = x.constant & y.constant;

    let extra = if newconstant == Self::K_LITERAL {
      0
    } else {
      1 | (0x0101010101010101u64 & newconstant)
    };

    Cost {
      model: parallel_add_sat(new_model, extra),
      constant: newconstant,
    }
  }
}
