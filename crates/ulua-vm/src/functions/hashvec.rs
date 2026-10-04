use crate::{
  macros::{hashpow_2::hashpow2, lua_vector_size::LUA_VECTOR_SIZE},
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// f32 的负零位型：cpp `ltable.cpp:153` 把 `-0` 归一到 `+0`，令二者哈希相同。
const F32_NEG_ZERO_BITS: u32 = 0x8000_0000;

/// 分量打散右移位数（cpp `ltable.cpp:160`：把高位熵混入低位，使整数坐标也具散列性）。
const SCRAMBLE_SHIFT: u32 = 17;

/// OSH 优化空间哈希各分量质数（cpp `ltable.cpp:166/171`，取自
/// "Optimized Spatial Hashing for Collision Detection of Deformable Objects"）。
const OSH_PRIMES: [u32; 4] = [73856093, 19349663, 83492791, 39916801];

/// 单分量归一并打散：先把 f32 `-0` 位型归一到 `+0`（保证 ±0 同哈希），再右移混入
/// 高位熵（cpp `ltable.cpp:153/160`）。分量彼此独立，逐分量处理与 cpp「先归一全部、
/// 再打散全部」两段式严格同形。
const fn norm_scramble(x: u32) -> u32 {
  let x = if x == F32_NEG_ZERO_BITS { 0 } else { x };
  x ^ (x >> SCRAMBLE_SHIFT)
}

/// 向量键哈希桶定位（w6e 入侧收形：`t: *const LuaTable → &LuaTable`、
/// `v: *const f32 → &[f32; LUA_VECTOR_SIZE]`，体内不再经裸指针逐分量 `add` 游走；
/// 位型读取由 `f32::to_bits` 与 cpp  reinterpret 逐位等价）。
///
/// 调用序契约（正确性，非内存安全——两个引用形参的存活已由类型承载）：表哈希部分
/// 须已分配，`sizenode(t)` 为 2 的幂，返回值恒落 `node[0..sizenode(t))` 内某槽。
///
/// 出口形态判定：返回 `*mut LuaNode` 保留（`LuaNode` 属表 `d array` 柔性成员的
/// 布局绑定节点地址，判据同 [`hashint`]）；构造点收口为末句 `hashpow2!` 单表达式。
/// cpp `ltable.cpp:143-175`。
pub(crate) fn hashvec(t: &LuaTable, v: &[f32; LUA_VECTOR_SIZE as usize]) -> *mut LuaNode {
  let x0 = norm_scramble(v[0].to_bits());
  let x1 = norm_scramble(v[1].to_bits());
  let x2 = norm_scramble(v[2].to_bits());

  // OSH 空间哈希：各分量乘不同质数再异或（cpp ltable.cpp:166）
  let h = x0.wrapping_mul(OSH_PRIMES[0])
    ^ x1.wrapping_mul(OSH_PRIMES[1])
    ^ x2.wrapping_mul(OSH_PRIMES[2]);

  // 本仓向量恒 3 分量（LUA_VECTOR_SIZE=3，`as_vector_ref` 视图同宽）：cpp
  // `#if LUA_VECTOR_SIZE == 4` 追加第 4 分量臂（ltable.cpp:171）在本形下由数组
  // 长度静态排除，无运行期死支（§3 死代码消除，行为对 3 分量 oracle 逐位一致）。

  // 哈希恒按 2^k 归约取节点（cpp ltable.cpp:174 `hashpow2(t, h)`，与
  // hashint/hashnum/hashpointer 同收口至 hashpow2!）
  // SAFETY: `hashpow2!` 内 `lmod!(…, sizenode!(t))` 恒落 `[0, sizenode(t))`，
  // `node.add` 不越出表哈希数组（上方调用序契约；宏自带 E1 裁决口径保留面）。
  unsafe { hashpow2!(t, h) }
}
