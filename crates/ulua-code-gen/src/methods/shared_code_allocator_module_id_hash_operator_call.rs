use crate::{records::module_id_hash::ModuleIdHash, type_aliases::module_id::ModuleId};

/// module_id 散列固定种子：跨平台、跨进程稳定，替代 std DefaultHasher(SipHash)。
const MODULE_ID_HASH_SEED: u64 = 0;

impl ModuleIdHash {
  #[inline]
  pub fn shared_code_allocator_module_id_hash_operator_call(&self, module_id: &ModuleId) -> usize {
    // ModuleId 固定 16 字节，museair 一次成型。
    museair::hash(module_id, MODULE_ID_HASH_SEED) as usize
  }
}
