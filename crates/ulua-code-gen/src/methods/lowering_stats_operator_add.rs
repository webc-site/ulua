use crate::records::lowering_stats::LoweringStats;
use core::ops::Add;

impl LoweringStats {
    #[inline]
    pub fn lowering_stats_operator_add(&self, other: &LoweringStats) -> LoweringStats {
        let mut result = self.clone();
        result.lowering_stats_operator_add_assign(other);
        result
    }
}

impl Add for LoweringStats {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.lowering_stats_operator_add(&rhs)
    }
}

impl Add<&LoweringStats> for LoweringStats {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &LoweringStats) -> Self::Output {
        self.lowering_stats_operator_add(rhs)
    }
}

impl Add<&LoweringStats> for &LoweringStats {
    type Output = LoweringStats;

    #[inline]
    fn add(self, rhs: &LoweringStats) -> Self::Output {
        self.lowering_stats_operator_add(rhs)
    }
}
