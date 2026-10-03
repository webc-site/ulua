//! `__index` 继承链多点缓存（J4-real 多点扩展：解释器慢路与 JIT fallback 共用）。
//!
//! 动机：`lua_v_gettable` 沿 `__index` 链逐级走查时每跳一次哈希查找 + 字符串比较，
//! 3/4 跳链（inherit3 的 describe/breathe）每迭代多付 2-3 次哈希。本模块按
//! `(首表元表 mt0, 键字符串)` 键控 2 路组相联缓存已解析的链表序列
//! `chain[0..depth]`（`chain[depth-1]` 即 owner），命中后压缩为
//! 「逐级 `fasttm(__index)` 恒等守卫走查 + owner 单次哈希查找」。
//!
//! 正确性设计（红线：与原慢路逐格一致）：
//! 1. 守卫只做表指针恒等比较，不解引用缓存指针；全过 ⟺ 原慢路必沿同一表序列走查
//!    （元链逐级吻合），此时对 owner 的读取就是对活链末端表的读取——指针复用/悬垂
//!    场景下地址等值即当前对象，无假命中。
//! 2. 中间级「后插键遮蔽」（如运行期给父类补方法）由内容纪元（epoch）拦住：改变
//!    「字符串键 → 非 nil 值」存在性的写咽喉（`lua_h_newkey` 新键、`lua_h_setstr`/
//!    `lua_h_setslot!` 旧槽复用、`lua_h_clear` 清表）经 `index_chain_write` 失效——
//!    被写表先过 64 位 Bloom 位图（fill 时登记 t0 与全链表），只命中才 bump 全局
//!    epoch；Bloom 无假阴性，被缓存表被写必失效，严格性不降。未登记表（高频
//!    构造期写）零失效成本，避免 epoch 误伤击穿命中率。owner 槽值的失/存由命中
//!    路径现场哈希查找天然覆盖（槽地址不跨查找缓存，rehash 安全）。
//! 3. epoch 用 u64 单调递增：物理不可达回绕，无回绕假匹配洞。
//! 4. 缓存本体 thread_local（每线程独立，与 VM 线程模型对齐，无锁）；跨 state 指针
//!    空间由守卫恒等隔离，互踩仅损失命中率。2 路组相联根除两热站点同组互踢振荡。

use core::{
  cell::{Cell, UnsafeCell},
  mem::MaybeUninit,
  ptr::null_mut,
};

use crate::{
  enums::tms::TMS,
  functions::lua_h_getstr::lua_h_getstr,
  macros::{fasttm::fasttm, gval_2_slot::gval2slot, setobj_2_s::setobj_2_s},
  records::{lua_state::LuaState, lua_table::LuaTable, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// 缓存链最大深度：覆盖现实继承链（3-4 跳为热点形态），超深链不缓存（走原慢路）。
pub(crate) const INDEX_CHAIN_MAX: usize = 8;

/// 组相联路数：两热站点哈希同组时互踢退化为每次慢路，2 路即根除振荡。
const INDEX_CHAIN_WAYS: usize = 2;

/// 组数（2 的幂）：单组 miss 即被下次 fill 覆写，无淘汰策略（空路优先，满则覆路 0）。
const INDEX_CHAIN_SETS: usize = 32;

/// 总槽数（编译期折算）。
const INDEX_CHAIN_SLOTS: usize = INDEX_CHAIN_SETS * INDEX_CHAIN_WAYS;

/// 组掩码。
const INDEX_CHAIN_MASK: usize = INDEX_CHAIN_SETS - 1;

/// 单缓存槽：键 `(mt0, key)` + 内容纪元 + 已解析链表序列。
#[derive(Clone, Copy)]
struct ChainSlot {
  /// 键基：首表 t0 的元表（t0 实例每迭代新建也不影响，元表即类表恒定）。
  mt0: *mut LuaTable,
  /// 键基：字符串键（interned，指针即身份）。
  key: *mut tstring,
  /// 末次回填的 t0：与当前 t0 一致时该表必在受监视位图（fill 登记），其写必失效
  /// 本缓存，probe 可免 t0 现场槽查找；漂移（实例换新）则退化为现场查找或拉黑。
  t0: *mut LuaTable,
  /// t0 漂移计数：连续以不同 t0 回填同组（实例高频新建形态）时递增，≥2 即拉黑
  /// 停用本组（probe 一次键比退出、fill 不再回填）——漂移形态下缓存恒负收益。
  drift: u8,
  /// 链表序列：`chain[i]` 为原慢路第 i+2 轮解析到的表，`chain[depth-1]` 即 owner。
  chain: [*mut LuaTable; INDEX_CHAIN_MAX],
  /// 链长（≥1 才为有效槽；0 即空槽）。
  depth: u8,
  /// 回填时的内容纪元；0 保留作空槽哨兵（有效纪元从 1 起）。
  epoch: u64,
}

impl ChainSlot {
  const EMPTY_SLOT: Self = Self {
    mt0: null_mut(),
    key: null_mut(),
    t0: null_mut(),
    drift: 0,
    chain: [null_mut(); INDEX_CHAIN_MAX],
    depth: 0,
    epoch: 0,
  };
}

/// 线程局部缓存表：内容纪元 + 受监视表 Bloom 位图 + 组相联槽阵列。
struct ChainCacheTable {
  epoch: Cell<u64>,
  /// 受监视表位图：fill 登记的 t0/mt0/链表地址指纹；写点位测命中才 bump。
  bloom: Cell<u64>,
  /// 拉黑组负缓存位：漂移停用组的组号置位，probe 一次位测即退（免两路键比）。
  negative: Cell<u64>,
  table: UnsafeCell<[ChainSlot; INDEX_CHAIN_SLOTS]>,
}

thread_local! {
  /// 每线程 `__index` 链缓存（const 初始化，访问无惰性分支）。
  static INDEX_CHAIN_CACHE: ChainCacheTable = const {
    ChainCacheTable {
      epoch: Cell::new(1),
      bloom: Cell::new(0),
      negative: Cell::new(0),
      table: UnsafeCell::new([ChainSlot::EMPTY_SLOT; INDEX_CHAIN_SLOTS]),
    }
  };
}

/// 组号：`(mt0 ^ key) >> 3` 后斐波那契乘法取**高位**——乘法低位对结构化地址差
/// （同元表站点键相邻分配）信息量差，高位才是黄金比例混淆的有效位。
#[inline(always)]
fn chain_set_index(mt0: *const LuaTable, key: *const tstring) -> usize {
  // u64 中转乘法：32 位目标（wasm32 usize）下 64 位常数不溢出，高位混淆保持
  let h = (((mt0 as usize >> 3) ^ (key as usize >> 3)) as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
  ((h >> (u64::BITS - INDEX_CHAIN_MASK.count_ones())) as usize) & INDEX_CHAIN_MASK
}

/// 表地址指纹（Bloom 位号）：乘法黄金比例取低 6 位，位图仅作「可能被缓存」判定，
/// 允许误报（多 bump 一次）、杜绝漏报（无假阴性），严格性不降。
#[inline(always)]
fn table_bloom_bit(t: *const LuaTable) -> u64 {
  1u64 << (((t as usize >> 4) as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) & 63)
}

/// 写侧失效咽喉：被写表命中受监视位图才 bump 内容纪元（`lua_h_newkey`/
/// `lua_h_setstr`/`lua_h_setslot!`/`lua_h_clear` 四类写点调用）。未登记表（典型：
/// 构造期新表键写）零成本穿过，避免全局失效击穿命中率。
#[inline]
pub(crate) fn index_chain_write(t: *const LuaTable) {
  INDEX_CHAIN_CACHE.with(|cache| {
    if cache.bloom.get() & table_bloom_bit(t) != 0 {
      cache.epoch.set(cache.epoch.get().wrapping_add(1));
    }
  });
}

/// 查找 + 守卫走查 + owner 现场槽查找（命中即写回 `val` 并回填 `cachedslot`）。
///
/// # Safety
/// `l` 指向存活 `LuaState`；`t0` 为存活表且 `(*t0).metatable` 非空（调用点契约）；
/// `key` 为存活 interned 字符串；`val` 为当前帧可写栈槽。体内仅 fasttm/哈希读与
/// cachedslot/栈槽写，无 VM 重入点，thread_local 槽借用不逃逸。
#[inline]
#[inline(never)]
pub(crate) unsafe fn index_chain_probe(
  l: *mut LuaState,
  t0: *mut LuaTable,
  key: *mut tstring,
  val: *mut TValue,
) -> bool {
  // Safety: 调用点契约——l 为存活 LuaState，t0/key/val 满足上述函数文档前置
  unsafe {
    let mt0 = (*t0).metatable;
    INDEX_CHAIN_CACHE.with(|cache| {
      let epoch = cache.epoch.get();
      // Safety: thread_local 单线程独占，本闭包内无重入点，槽借用不逃逸
      let set = chain_set_index(mt0, key);
      // 拉黑组负缓存：一次位测即退，免两路键比（漂移形态 probe 只剩此成本）
      if cache.negative.get() & (1u64 << set) != 0 {
        return false;
      }
      let slots = &mut *cache.table.get();
      let set = set * INDEX_CHAIN_WAYS;
      // 2 路组相联：任一路键/纪元全配即命中；全失配才落慢路
      let hit0 = slots[set].epoch == epoch && slots[set].mt0 == mt0 && slots[set].key == key;
      let hit1 =
        slots[set + 1].epoch == epoch && slots[set + 1].mt0 == mt0 && slots[set + 1].key == key;
      if !hit0 && !hit1 {
        return false;
      }
      let s = if hit0 {
        &mut slots[set]
      } else {
        &mut slots[set + 1]
      };
      let depth = s.depth as usize;
      if depth == 0 {
        return false;
      }
      // t0 稳定即免现场查找：该 t0 是末次回填对象，已登记受监视位图，其后被写必
      // bump 失效本槽——本探测能走到这里即槽仍然有效，t0 未被写过，无需再查
      if s.t0 != t0 {
        // t0 漂移（同元表不同实例）：现场槽查找确认 t0 无此键（遮蔽安全）——命中非 nil
        // ⟺ 原慢路轮 1 直接命中，写回与 cachedslot 同款；nil/miss 沿守卫走查
        let res0 = lua_h_getstr(&*t0, key);
        if let Some(res0) = res0 {
          let r0 = res0.as_const_ptr();
          if !(*r0).is_nil() {
            (*l).cachedslot = gval2slot!(t0, r0);
            setobj_2_s!(l, val, r0);
            return true;
          }
        }
      }
      // 守卫走查：逐级以「当前表的元表」为对象 fasttm(__index)，与缓存链表恒等比较
      // （表指针直比，无哈希无键比较，O(depth) 次取址）——与原慢路逐轮的
      // `fasttm(l, h.metatable, TmIndex)` 同形同输入。任一失配/元方法缺席或非表 →
      // 清槽落回完整慢路。
      let mut cur = t0;
      for i in 0..depth {
        let want = s.chain[i];
        // Safety: cur 为 t0 或守卫已证活表，其 metatable 读数存活期内可读；
        // fasttm 与原慢路同函数同输入（同查 t_i 的元表的 __index 槽），
        // 元链变化经 tmcache/元表写路径如实反映
        let tm = fasttm(l, (*cur).metatable, TMS::TmIndex);
        if tm.is_null() {
          s.depth = 0;
          s.epoch = 0;
          return false;
        }
        // Safety: tm 为元表节点槽，守卫已证表 tag，读载荷即活表指针
        let got = (*tm).as_table_ptr();
        if got != want {
          s.depth = 0;
          s.epoch = 0;
          return false;
        }
        cur = want;
      }
      // owner 现场哈希查找：主槽起沿 next 链与原慢路同构，rehash 天然安全；
      // owner 槽值已失（nil/删除）→ 清槽落慢路（原慢路会继续 owner 之后的链）
      // Safety: cur 为守卫全过后的活链末端表（地址等值即当前对象）
      let res = lua_h_getstr(&*cur, key);
      let Some(res) = res else {
        s.depth = 0;
        s.epoch = 0;
        return false;
      };
      let r = res.as_const_ptr();
      if (*r).is_nil() {
        s.depth = 0;
        s.epoch = 0;
        return false;
      }
      // Safety: r 为 owner 表内活值槽；gval2slot 仅做同表节点算术（原慢路命中同款）
      (*l).cachedslot = gval2slot!(cur, r);
      setobj_2_s!(l, val, r);
      true
    })
  }
}

/// 慢路走查伴随回填：原慢路沿全表链（无函数元方法、无报错）走到非 nil 命中时，
/// 把逐级解析出的链表序列记入槽（miss 即覆写，无淘汰策略）。
///
/// # Safety
/// `t0`/`mt0`/`key`/`chain` 各表指针均为本次慢路走查实际解析到的存活对象（调用点
/// 契约）；`chain` 非空且长 ≤ [`INDEX_CHAIN_MAX`]。
#[inline]
pub(crate) unsafe fn index_chain_fill(
  t0: *mut LuaTable,
  mt0: *mut LuaTable,
  key: *mut tstring,
  chain: &[MaybeUninit<*mut LuaTable>],
  chain_len: usize,
) {
  debug_assert!(chain_len > 0 && chain_len <= INDEX_CHAIN_MAX);
  INDEX_CHAIN_CACHE.with(|cache| {
    // Safety: thread_local 单线程独占，本闭包内无重入点，槽借用不逃逸
    let slots = unsafe { &mut *cache.table.get() };
    let set = chain_set_index(mt0, key) * INDEX_CHAIN_WAYS;
    // 空路优先；两路皆满覆路 0（无 LRU，从简）
    let s = &mut slots[set];
    // 漂移拉黑：同组已缓存且 t0 换了新实例 → 递增漂移，≥2 停用（清槽且不再回填，
    // 只留 probe 一次负位测退出的成本）；t0 回归（同一实例重新稳定）即复位解禁并清
    // 负位。漂移形态下每迭代失效/回填恒负收益。纯性能启发式，不触碰任何语义路径。
    if s.drift >= 2 {
      if s.t0 == t0 {
        s.drift = 0;
        cache
          .negative
          .set(cache.negative.get() & !(1u64 << (set / INDEX_CHAIN_WAYS)));
      } else {
        return;
      }
    } else if s.depth != 0 && s.t0 != t0 {
      s.drift += 1;
      if s.drift >= 2 {
        s.depth = 0;
        s.epoch = 0;
        s.t0 = t0;
        cache
          .negative
          .set(cache.negative.get() | (1u64 << (set / INDEX_CHAIN_WAYS)));
        return;
      }
    } else {
      s.drift = 0;
    }
    s.t0 = t0;
    s.mt0 = mt0;
    s.key = key;
    s.epoch = cache.epoch.get();
    s.depth = chain_len as u8;
    for (i, c) in chain[..chain_len].iter().enumerate() {
      s.chain[i] = unsafe { c.assume_init() };
    }
    // 登记受监视表：t0/元表基/全链（含 owner）此后任一被写即失效本缓存
    let mut b = cache.bloom.get() | table_bloom_bit(t0);
    for c in &chain[..chain_len] {
      b |= table_bloom_bit(unsafe { c.assume_init() });
    }
    cache.bloom.set(b);
  });
}
