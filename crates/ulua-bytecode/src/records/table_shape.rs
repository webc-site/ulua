#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableShape {
  pub keys: [i32; 32],
  pub constants: [i32; 32],
  pub length: u32,
  pub has_constants: bool,
}

impl TableShape {
  pub const K_MAX_LENGTH: u32 = 32;

  /// 形状数组长度（`K_MAX_LENGTH` 的 usize 形式，避免各处重复 as 转换）。
  const KEYS_LEN: usize = Self::K_MAX_LENGTH as usize;

  /// `DenseHashMap::new` 所需的空键哨兵：**必须永不作为真实键出现**。
  ///
  /// `length = u32::MAX` 远大于解析入口对 `LBC_CONSTANT_TABLE` 的硬上界
  /// `K_MAX_LENGTH`（见 `functions/from_function_bytecode.rs`），编译器侧同样
  /// 不可能产出这么长的形状，故不可达。
  ///
  /// 旧实现直接用 `TableShape::default()`，而 default 的 `{length: 0,
  /// has_constants: false}` 恰是零长度 DUPTABLE（不可信输入里 length 只被上界
  /// 校验，允许为 0）能取到的形状：命中哨兵后 `find` 恒返回 `None`（去重彻底
  /// 失效），`insert_unsafe` 又在 debug 下 `debug_assert` panic、release 下把
  /// 哨兵键写进槽位破坏开放寻址。`has_constants: true` 是第二道防线。
  pub const EMPTY_KEY_SENTINEL: Self = Self {
    keys: [0; Self::KEYS_LEN],
    constants: [-1; Self::KEYS_LEN],
    length: u32::MAX,
    has_constants: true,
  };
}

// 编译期不变量：哨兵长度必须严格超出解析上界，否则真实键会与哨兵相撞。
const _: () = assert!(TableShape::EMPTY_KEY_SENTINEL.length > TableShape::K_MAX_LENGTH);

impl Default for TableShape {
  fn default() -> Self {
    Self {
      keys: [0; Self::KEYS_LEN],
      constants: [-1; Self::KEYS_LEN],
      length: 0,
      has_constants: false,
    }
  }
}
