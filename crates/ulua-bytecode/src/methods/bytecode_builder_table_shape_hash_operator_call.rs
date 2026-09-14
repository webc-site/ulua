use crate::records::{table_shape::TableShape, table_shape_hash::TableShapeHash};

/// FNV-1a 偏移基数。
const FNV_OFFSET_BASIS: u32 = 2166136261;
/// FNV-1a 质数。
const FNV_PRIME: u32 = 16777619;

impl TableShapeHash {
  pub fn operator_call(&self, v: &TableShape) -> usize {
    // FNV-1a inspired hash (note that we feed integers instead of bytes)
    let mut hash = FNV_OFFSET_BASIS;

    for (i, &key) in v.keys.iter().enumerate().take(v.length as usize) {
      hash ^= key as u32;
      hash = hash.wrapping_mul(FNV_PRIME);

      // Note: FFlag::LuauCompileDuptableConstantPack2 is assumed true in this translation context
      // as we are translating the logic that depends on the shape's internal state.
      if v.has_constants {
        hash ^= v.constants[i] as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
      }
    }

    hash as usize
  }
}
