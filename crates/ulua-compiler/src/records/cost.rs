//! Source: `Compiler/src/CostModel.cpp:50-101`

use core::ops::{Add, AddAssign, Mul};

use crate::functions::{parallel_add_sat::parallel_add_sat, parallel_mul_sat::parallel_mul_sat};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Cost {
  pub(crate) model: u64,
  pub(crate) constant: u64,
}

impl Cost {
  pub(crate) const K_LITERAL: u64 = !0u64;

  /// 单字节通道基线代价的饱和上限（cpp `CostModel.cpp:56` 的 `cost < 0x7f ? cost : 0x7f`）
  const K_MAX_CHANNEL_COST: i32 = 0x7f;
  /// 「每字节通道各置 1」的 SWAR 广播常量（cpp `CostModel.cpp:89` 的
  /// `0x0101010101010101ull`）：`fold` 用它给共享变量那一通道各加 1 折扣
  const K_LANE_ONES: u64 = 0x0101_0101_0101_0101;

  pub fn new(cost: i32, constant: u64) -> Self {
    Self {
      model: if cost < Self::K_MAX_CHANNEL_COST {
        cost as u64
      } else {
        Self::K_MAX_CHANNEL_COST as u64
      },
      constant,
    }
  }

  pub fn add(&self, other: &Cost) -> Cost {
    *self + other
  }

  pub fn add_assign(&mut self, other: &Cost) {
    *self += other;
  }

  pub fn fold(x: &Cost, y: &Cost) -> Cost {
    let new_model = parallel_add_sat(x.model, y.model);
    let newconstant: u64 = x.constant & y.constant;

    let extra = if newconstant == Self::K_LITERAL {
      0
    } else {
      1 | (Self::K_LANE_ONES & newconstant)
    };

    Cost {
      model: parallel_add_sat(new_model, extra),
      constant: newconstant,
    }
  }
}

impl Add<Cost> for Cost {
  type Output = Cost;

  #[inline]
  fn add(self, rhs: Cost) -> Cost {
    Cost {
      model: parallel_add_sat(self.model, rhs.model),
      constant: 0,
    }
  }
}

impl Add<&Cost> for Cost {
  type Output = Cost;

  #[inline]
  fn add(self, rhs: &Cost) -> Cost {
    self + *rhs
  }
}

impl AddAssign<Cost> for Cost {
  #[inline]
  fn add_assign(&mut self, rhs: Cost) {
    self.model = parallel_add_sat(self.model, rhs.model);
    self.constant = 0;
  }
}

impl AddAssign<&Cost> for Cost {
  #[inline]
  fn add_assign(&mut self, rhs: &Cost) {
    *self += *rhs;
  }
}

impl Mul<i32> for Cost {
  type Output = Cost;

  #[inline]
  fn mul(self, rhs: i32) -> Cost {
    Cost {
      model: parallel_mul_sat(self.model, rhs),
      constant: 0,
    }
  }
}
