use ulua_common::functions::import_layout::{K_IMPORT_COUNT_SHIFT, import_component};

/// `LOP_GETIMPORT` aux 的单一解码源：返回组件数与按顺的组件常量索引。
/// `rebuild_graph`（图重建）与 `validate_instructions`（校验）共用，编码侧见
/// `emit_instruction` 的 GETIMPORT 分支与 `get_import_id*`；布局见 `K_IMPORT_COUNT_SHIFT`。
pub(crate) fn decode_import_aux(aux: u32) -> (u32, [u32; 3]) {
  let count = aux >> K_IMPORT_COUNT_SHIFT;
  let mut components = [0u32; 3];
  for (k, slot) in components.iter_mut().enumerate().take(count as usize) {
    *slot = import_component(aux, k as u32);
  }
  (count, components)
}
