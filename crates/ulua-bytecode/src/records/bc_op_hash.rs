use core::hash::{BuildHasher, Hasher};

use ulua_common::records::dense_hash_table::DenseHasher;

use crate::records::bc_op::BcOp;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct BcOpHash;

/// `BcOpKind` 变体数 < 16：kind 占低 4 位，index 左移 4 位拼成单字。
const KIND_MASK: usize = 0x0F;
const INDEX_SHIFT: usize = 4;

impl BcOpHash {
  pub(crate) fn operator_call(&self, p: &BcOp) -> usize {
    ((p.kind as usize) & KIND_MASK) | ((p.index as usize) << INDEX_SHIFT)
  }
}

/// 64 位雪崩混合常数（黄金比例，与 foldhash/splitmix64 同族）。
const MIX_MULTIPLIER: u64 = 0x9E37_79B9_7F4A_7C15;

/// `BcOp` 的零成本 `Hasher`。
///
/// `BcFunction::regs` 是工作区别名 `HashMap<BcOp, Reg, BcOpHash>`（hashbrown +
/// 本自定义 hasher），每次 `getRegister` 都要哈希一次，属编译热
/// 路径；原先 `BuildHasher` 返回 `DefaultHasher`（SipHash，含密钥初始化与逐字节
/// 吸收），比 `operator_call` 的位打包贵一个数量级。`BcOp` 只有 `kind`（判别值）
/// 与 `index`（u32）两个定长字段，抗碰撞由 `Eq` 兜底，无需密码学强度。
/// 本类型是 `BcOpHash: BuildHasher` 的关联 `Hasher`，随之外泄，故为 `pub`；
/// 内部状态字段保持私有，调用方只能经 `BcOpHash::build_hasher` 构造。
#[derive(Default)]
pub struct BcOpHasher(u64);

impl BcOpHasher {
  #[inline]
  fn mix(&mut self, value: u64) {
    let mut x = (self.0 ^ value).wrapping_mul(MIX_MULTIPLIER);
    x ^= x >> 29;
    x = x.wrapping_mul(MIX_MULTIPLIER);
    x ^= x >> 32;
    self.0 = x;
  }
}

impl Hasher for BcOpHasher {
  #[inline]
  fn write(&mut self, bytes: &[u8]) {
    // 变长字节流按 8 字节块折叠；`#[derive(Hash)]` 对 BcOp 只走定长路径，
    // 本分支是 `write_*` 默认实现转发到的字节流兜底（契约要求 `write` 可读
    // 任意 `&[u8]`），保留以防 derive 展开形态变化。
    for chunk in bytes.chunks(8) {
      let mut buf = [0u8; 8];
      buf[..chunk.len()].copy_from_slice(chunk);
      self.mix(u64::from_le_bytes(buf));
    }
  }

  // 探针实测（derive Hash on BcOp{kind: repr(u32) 无 payload 枚举, index: u32}）
  // 只调用 `write_u32` 两次：本 hasher 仅服务 `HashMap/HashSet<DenseHash*, BcOp, BcOpHash>`
  // 键形如 BcOp 的哈希，其余整数宽度的 `write_*` 覆写零消费，已删（默认实现
  // 会转发到上方 `write` 兜底，正确性不受影响）。
  #[inline]
  fn write_u32(&mut self, i: u32) {
    self.mix(u64::from(i));
  }

  #[inline]
  fn finish(&self) -> u64 {
    self.0
  }
}

impl BuildHasher for BcOpHash {
  type Hasher = BcOpHasher;

  fn build_hasher(&self) -> Self::Hasher {
    BcOpHasher::default()
  }
}

impl DenseHasher<BcOp> for BcOpHash {
  fn hash(&self, key: &BcOp) -> usize {
    self.operator_call(key)
  }
}
