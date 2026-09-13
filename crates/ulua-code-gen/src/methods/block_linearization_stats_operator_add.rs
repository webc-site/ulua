use crate::records::block_linearization_stats::BlockLinearizationStats;

impl BlockLinearizationStats {
    pub fn block_linearization_stats_operator_add(
        &self,
        other: &BlockLinearizationStats,
    ) -> BlockLinearizationStats {
        let mut result = *self;
        result.block_linearization_stats_operator_add_assign(other);
        result
    }
}

impl core::ops::Add for BlockLinearizationStats {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.block_linearization_stats_operator_add(&rhs)
    }
}

impl core::ops::Add<&BlockLinearizationStats> for BlockLinearizationStats {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &BlockLinearizationStats) -> Self::Output {
        self.block_linearization_stats_operator_add(rhs)
    }
}
