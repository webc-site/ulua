use core::hash::{BuildHasher, Hasher};

use ulua_common::records::dense_hash_table::DenseHasher;

use crate::records::{bc_op::BcOp, bc_op_hash::BcOpHash};

impl BcOpHash {
  pub fn operator_call(&self, p: &BcOp) -> usize {
    ((p.kind as usize) & 0x0F) | ((p.index as usize) << 4)
  }
}

/// 64 位雪崩混合常数（黄金比例，与 foldhash/splitmix64 同族）。
const MIX_MULTIPLIER: u64 = 0x9E37_79B9_7F4A_7C15;

/// `BcOp` 的零成本 `Hasher`。
///
/// `BcFunction::regs` 是 std `HashMap`，每次 `getRegister` 都要哈希一次，属编译热
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
    // 本分支仅为满足 `Hasher` 契约（`write_*` 默认实现都会转发到这里）。
    for chunk in bytes.chunks(8) {
      let mut buf = [0u8; 8];
      buf[..chunk.len()].copy_from_slice(chunk);
      self.mix(u64::from_le_bytes(buf));
    }
  }

  #[inline]
  fn write_u8(&mut self, i: u8) {
    self.mix(u64::from(i));
  }

  #[inline]
  fn write_i8(&mut self, i: i8) {
    self.mix(i as u8 as u64);
  }

  #[inline]
  fn write_u32(&mut self, i: u32) {
    self.mix(u64::from(i));
  }

  #[inline]
  fn write_i32(&mut self, i: i32) {
    self.mix(i as u32 as u64);
  }

  #[inline]
  fn write_u64(&mut self, i: u64) {
    self.mix(i);
  }

  #[inline]
  fn write_i64(&mut self, i: i64) {
    self.mix(i as u64);
  }

  #[inline]
  fn write_usize(&mut self, i: usize) {
    self.mix(i as u64);
  }

  #[inline]
  fn write_isize(&mut self, i: isize) {
    self.mix(i as u64);
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
