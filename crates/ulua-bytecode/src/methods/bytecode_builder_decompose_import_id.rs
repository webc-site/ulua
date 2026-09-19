use crate::records::bytecode_builder::BytecodeBuilder;

/// cpp 魔数 1023：import id 的 10 位掩码。
const MASK_10BIT: i32 = 1023;

impl BytecodeBuilder {
  /// 元组返回 `(count, id0, id1, id2)` 替代 cpp 的三个 `int32_t&` 出参。
  pub fn decompose_import_id(ids: u32) -> (i32, i32, i32, i32) {
    let count = (ids >> 30) as i32;
    let id0 = if count > 0 {
      (ids >> 20) as i32 & MASK_10BIT
    } else {
      -1
    };
    let id1 = if count > 1 {
      (ids >> 10) as i32 & MASK_10BIT
    } else {
      -1
    };
    let id2 = if count > 2 {
      ids as i32 & MASK_10BIT
    } else {
      -1
    };
    (count, id0, id1, id2)
  }
}
