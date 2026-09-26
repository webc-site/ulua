use core::ptr::copy_nonoverlapping;

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

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn hashvec(t: *const LuaTable, v: *const f32) -> *mut LuaNode {
  unsafe {
    let mut i = [0u32; 4];

    copy_nonoverlapping(v as *const u32, i.as_mut_ptr(), LUA_VECTOR_SIZE as usize);

    // 逐分量归一 + 打散：仅覆盖实际存在的 LUA_VECTOR_SIZE 个分量，尾槽保持 0 不参与
    for x in i.iter_mut().take(LUA_VECTOR_SIZE as usize) {
      *x = norm_scramble(*x);
    }

    // OSH 空间哈希：各分量乘不同质数再异或（cpp ltable.cpp:166）
    let mut h = i[0].wrapping_mul(OSH_PRIMES[0])
      ^ i[1].wrapping_mul(OSH_PRIMES[1])
      ^ i[2].wrapping_mul(OSH_PRIMES[2]);

    if LUA_VECTOR_SIZE == 4 {
      h ^= i[3].wrapping_mul(OSH_PRIMES[3]);
    }

    // 哈希恒按 2^k 归约取节点（cpp ltable.cpp:174 `hashpow2(t, h)`，与
    // hashint/hashnum/hashpointer 同收口至 hashpow2!）
    hashpow2!(t, h)
  }
}
