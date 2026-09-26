use crate::{
  enums::normalized_part::{ALL_PARTS, NormalizedPart},
  records::normalized_type::NormalizedType,
};

impl NormalizedType {
  /// cpp 各 `isX()` 谓词与多个 `types.*` 归约入口的共同形状：`allowed` 名单之外
  /// 的部件全部为空即为 `true`。名单内的正部件（如 `is_nil` 的 nils）由调用方
  /// 另行正判，与原判定链的短路顺序一致。
  pub fn has_parts_other_than(&self, allowed: &[NormalizedPart]) -> bool {
    ALL_PARTS
      .iter()
      .any(|part| !allowed.contains(part) && self.has_part(*part))
  }
}
