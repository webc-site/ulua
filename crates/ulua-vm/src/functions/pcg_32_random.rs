use crate::macros::pcg_32_inc::PCG32_INC;

/// PCG 家族的标准 LCG 乘数（Knuth MMIX），cpp `lmathlib.cpp:27` 同一枚字面量。
const PCG32_MULT: u64 = 6364136223846793005;
/// `pcg32_random` 输出函数 XSH-RR 的位型：异移 18、右移 27、旋量取高 5 位、
/// 旋转模 32（cpp `lmathlib.cpp:28-30` 逐字对应）。
const XSH_SHIFT: u32 = 18;
const XSR_SHIFT: u32 = 27;
const ROT_SHIFT: u32 = 59;
const ROT_MASK: u32 = 31;

/// cpp `lmathlib.cpp:pcg32_random`：Lua `math.random`/`randomseed` 的唯一随机源。
///
/// review.md §5 的 `fastrand` 在此**不引入**（有意例外，故写明理由）：`math.random`
/// 是**可复现的确定性流**——同一 `math.randomseed(s)` 在 cpp 与 Rust 侧必须产出逐值
/// 相同的结果序列（conformance 测试与用户代码都依赖这点），换任何第三方 PRNG 都
/// 会改写整条序列；本函数的状态推进/输出函数与 cpp 完全同型，且 `state` 由调用方
/// 以 `&mut u64` 传入、零分配，fastrand 的 Wyrand 既不给更好的分布也不给更快
/// （同为常数时间），只会造成行为漂移。
#[inline]
pub(crate) fn pcg_32_random(state: &mut u64) -> u32 {
  let oldstate = *state;
  // cpp 在 `uint64_t` 上做 `* +`，是无符号回绕；此处两条运算都必须是 wrapping
  // 形态，否则 debug 构建会在乘积接近 `u64::MAX` 时溢出 panic（同 `pcg_32_seed`）。
  *state = oldstate
    .wrapping_mul(PCG32_MULT)
    .wrapping_add(PCG32_INC | 1);
  let xorshifted = (((oldstate >> XSH_SHIFT) ^ oldstate) >> XSR_SHIFT) as u32;
  let rot = (oldstate >> ROT_SHIFT) as u32;
  let shift = (0u32.wrapping_sub(rot)) & ROT_MASK;
  (xorshifted >> rot) | (xorshifted << shift)
}
