/// 归一化器触达限额：文案逐字保留（旧 `Display` 即 `write!(f, "Normalizer hit limits")`）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, thiserror::Error)]
#[error("Normalizer hit limits")]
pub struct NormalizerHitLimits;
