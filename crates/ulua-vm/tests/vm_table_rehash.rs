//! resize 收缩分支（`nasize < oldasize`）的探针与回归钉。
//!
//! 对照 cpp/VM/src/ltable.cpp:622-643：收缩迁移循环里每轮必须重读**活的**
//! `t->array`（`newkey` 可重入 rehash→resize→setarrayvector 改变数组指针）。
//! 本文件用两条路径钉住该语义：
//! 1. 默认分配器下的功能性回归钉：构造「数组段有洞 + 哈希段被非数组键填到
//!    接近满」的表，再触发边界整数键插入逼出 rehash 收缩，断言 #t 与逐键取值。
//! 2. 探针分配器（poison + 强制搬移）下的同一形态：free 时整块填 0xA5、
//!    realloc 永不原地，任何对已释放数组槽位的读取都会暴露为污染值。

use core::{
  ffi::c_void,
  ptr::{copy_nonoverlapping, null_mut, write_bytes},
};
use std::{
  alloc::{Layout, alloc_zeroed},
  collections::HashMap,
  ffi::c_char,
  sync::{Mutex, OnceLock},
};

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_close::lua_close, lua_createtable::lua_createtable, lua_l_newstate::lua_l_newstate,
    lua_newstate::lua_newstate, lua_next::lua_next, lua_objlen::lua_objlen,
    lua_pushlstring::lua_pushlstring, lua_pushnil::lua_pushnil, lua_pushnumber::lua_pushnumber,
    lua_rawgeti::lua_rawgeti, lua_rawset::lua_rawset, lua_rawseti::lua_rawseti,
    lua_settop::lua_settop, lua_tonumberx::lua_tonumberx, lua_type::lua_type,
  },
  records::lua_state::LuaState,
  type_aliases::lua_alloc::LuaAlloc,
};

/// 初始数组段大小（2^14 槽 ×16B = 256KB，远超 K_MAX_SMALL_SIZE=1024，
/// 数组块的 realloc/free 全部直通 frealloc 钩子，探针分配器可完整记账）
const ASIZE: i32 = 16384;
/// 低段稠密区：1..=DENSE 全部有值
const DENSE: i32 = 4000;
/// 中段连续带：8000..=9000（rehash 的 adjustasize 会把它并入新数组段）
const RUN_LO: i32 = 8000;
const RUN_HI: i32 = 9000;
/// 远端孤立键（收缩迁移的主角：落到哈希段的数组尾元素）
const FAR_KEYS: [i32; 2] = [12000, 13000];
/// 触发边界 rehash 的键（= sizearray + 1）
const EK: i32 = ASIZE + 1;
/// 插入的非数组键数量（把哈希段填到接近满并驱动多次增长 rehash）
const STR_KEYS: i32 = 200;

/// 键 k 的期望值；None 表示该键不应存在
fn expected(k: i32) -> Option<f64> {
  let present =
    (1..=DENSE).contains(&k) || (RUN_LO..=RUN_HI).contains(&k) || FAR_KEYS.contains(&k) || k == EK;
  present.then(|| f64::from(k) * 7.0 + 0.5)
}

/// 按既定形态填充表：数组洞 + 连续带 + 远端孤键 + 非数组字符串键 + 边界键。
/// 复刻脚本层 `{1,2,nil,4,nil,...}` + 连续插入非数组键触发 rehash 的形态。
fn build_shape(l: *mut LuaState) {
  // Safety: `l` 由各用例经 newstate 断言非空后传入，且在本函数调用期间独占存活；
  // 下面仅调用 C-API 常规栈操作。
  unsafe {
    lua_createtable(l, ASIZE, 0); // 栈: [t]（表恒在绝对索引 1）
    // 直接迭代应存在的键段（三段互斥），无需全段扫过后逐个筛
    for k in (1..=DENSE).chain(RUN_LO..=RUN_HI).chain(FAR_KEYS) {
      lua_pushnumber(l, f64::from(k) * 7.0 + 0.5);
      lua_rawseti(l, 1, k);
    }
    // 连续插入非数组键：撑满哈希段并触发多次增长 rehash
    for j in 0..STR_KEYS {
      let name = format!("probe-key-{j}");
      lua_pushlstring(l, name.as_ptr() as *const c_char, name.len());
      lua_pushnumber(l, f64::from(j) + 0.25);
      lua_rawset(l, -3); // 弹出 k、v，保留 t
    }
    // 边界键：nvalue == sizearray + 1 → newkey 快路 rehash → resize 收缩
    lua_pushnumber(l, f64::from(EK) * 7.0 + 0.5);
    lua_rawseti(l, 1, EK);
  }
}

/// 读栈顶数字（tag 污染时 isnum 归零，当场失守）
fn read_number(l: *mut LuaState) -> f64 {
  // Safety: `l` 为调用方持有的存活 VM 状态，栈顶可读。
  unsafe {
    lua_tonumberx(l, -1).unwrap_or_else(|| panic!("栈顶不是数字（污染 tag 读）"))
  }
}

/// 逐键取值断言：所有整数键读回 = 期望值；计划外键必须为 nil。
/// UAF/悬垂读把 0xA5 垃圾搬进表时这里必然失守。
fn assert_values_intact(l: *mut LuaState) {
  // Safety: `l` 为调用方持有的存活 VM 状态，rawgeti/settop 走常规栈协议。
  unsafe {
    for k in 1..=(ASIZE + 1) {
      let want = expected(k);
      let tt = lua_rawgeti(l, 1, k);
      match want {
        Some(w) => {
          assert_eq!(tt, LuaType::Number as i32, "键 {k} 类型错位（UAF 暴露）");
          let got = read_number(l);
          assert_eq!(got, w, "键 {k} 取值被污染（期望 {w}）");
        }
        None => {
          if tt != LuaType::Nil as i32 {
            let got = read_number(l);
            panic!("键 {k} 不应存在，却读到 {got}（poison 读/UAF 暴露）");
          }
        }
      }
      lua_settop(l, 1); // 弹出读回值
    }
  }
}

/// 全量遍历：键值对总数与期望一致，且没有计划外的整数键
fn assert_traversal_clean(l: *mut LuaState) {
  // Safety: `l` 为调用方持有的存活 VM 状态，lua_next 迭代协议要求栈顶为 key（此处
  // 由 pushnil + settop(2) 维持）。
  unsafe {
    let mut total: usize = 0;
    let mut int_extra: Vec<i32> = Vec::new();
    lua_pushnil(l);
    while lua_next(l, 1) != 0 {
      total += 1;
      // 栈: [t, k, v]
      if lua_type(l, -2) == LuaType::Number as i32 {
        let k = lua_tonumberx(l, -2).unwrap_or(0.0);
        if k.fract() == 0.0 {
          let ki = k as i32;
          if (1..=(ASIZE + 1)).contains(&ki) && expected(ki).is_none() {
            int_extra.push(ki);
          }
        }
      }
      lua_settop(l, 2); // 弹 value，留 key 继续迭代
    }
    assert!(int_extra.is_empty(), "出现计划外整数键: {int_extra:?}");
    let want_total: usize =
      (DENSE as usize) + (RUN_HI - RUN_LO + 1) as usize + FAR_KEYS.len() + 1 + STR_KEYS as usize;
    assert_eq!(total, want_total, "键值对总数不符（收缩迁移丢失/复制？）");
  }
}

// ---------------------------------------------------------------------------
// 探针分配器：bump-only 零初始化块 + free 时 poison 0xA5 + realloc 永不原地。
// ---------------------------------------------------------------------------

static PROBE: OnceLock<Mutex<ProbeState>> = OnceLock::new();

struct ProbeState {
  cursor: usize,
  end: usize,
  live: HashMap<usize, usize>,
}

/// # Safety
///
/// `ProbeState` 的字段（`usize` 游标 + `HashMap<usize, usize>`）本身皆 `Send`，且始终
/// 经 `Mutex<ProbeState>` 串行访问；两个测试各自单线程驱动 VM，无真实跨线程并发。故
/// `Send` 成立（此显式 impl 只是把契约写明，与自动推导一致，未声称指针可跨线程解引用）。
unsafe impl Send for ProbeState {}

const CHUNK: usize = 32 << 20;

impl ProbeState {
  fn bump(&mut self, nsize: usize) -> *mut u8 {
    let need = (nsize + 15) & !15;
    if self.cursor + need > self.end {
      let layout = Layout::from_size_align(CHUNK, 16).expect("合法 layout");
      // Safety: layout 尺寸对齐均合法且非零，申请失败由下方 is_null 断言兜底。
      let base = unsafe { alloc_zeroed(layout) };
      assert!(!base.is_null(), "探针分配器 chunk 申请失败");
      self.cursor = base as usize;
      self.end = self.cursor + CHUNK;
    }
    let p = self.cursor as *mut u8;
    self.cursor += need;
    p
  }
}

fn probe() -> &'static Mutex<ProbeState> {
  PROBE.get_or_init(|| {
    Mutex::new(ProbeState {
      cursor: 0,
      end: 0,
      live: HashMap::new(),
    })
  })
}

fn poison(ptr: *mut u8, size: usize) {
  // Safety: 故意把已归还的块整体涂成 0xA5（探针语义本体，不是可安全化的数据）；
  // `ptr`/`size` 来自 live 记账的真实分配区间。
  unsafe { write_bytes(ptr, 0xA5, size) };
}

/// free 时按记账真实大小 poison；realloc 永远换块 + 复制 + poison 旧块。
/// 收缩循环若复用 `newkey` 前算出的槽位指针，读到的就是 0xA5 垃圾。
unsafe extern "C-unwind" fn probe_alloc(
  _ud: *mut c_void,
  ptr: *mut u8,
  osize: usize,
  nsize: usize,
) -> *mut u8 {
  let mut st = probe().lock().unwrap_or_else(|e| e.into_inner());
  if ptr.is_null() {
    let p = st.bump(nsize);
    st.live.insert(p as usize, nsize);
    return p;
  }
  let true_size = st.live.remove(&(ptr as usize)).unwrap_or(osize);
  if nsize == 0 {
    poison(ptr, true_size);
    return null_mut();
  }
  let newp = st.bump(nsize);
  let copy = true_size.min(nsize);
  // Safety: ptr..ptr+true_size 与 newp..newp+nsize 均为已分配的活区域，
  // copy 不越两者长度。
  unsafe { copy_nonoverlapping(ptr, newp, copy) };
  poison(ptr, true_size);
  st.live.insert(newp as usize, nsize);
  newp
}

/// 探针状态 RAII：测试退出关闭 VM
struct ProbeGuard {
  l: *mut LuaState,
}

impl Drop for ProbeGuard {
  fn drop(&mut self) {
    // Safety: `self.l` 由 probe_state 断言非空后独占持有。
    unsafe { lua_close(self.l) };
  }
}

fn probe_state() -> ProbeGuard {
  let f: LuaAlloc = Some(probe_alloc);
  // ud 故意传 null：探针分配器不需要 host 上下文（C-API 允许的哨兵形态）。
  let l = unsafe { lua_newstate(f, null_mut()) };
  assert!(!l.is_null(), "lua_newstate(probe_alloc) 失败");
  ProbeGuard { l }
}

// ---------------------------------------------------------------------------
// 用例
// ---------------------------------------------------------------------------

/// 默认分配器：收缩迁移 + #t + 逐键取值的功能回归钉
#[test]
fn shrink_migration_preserves_all_values_default_alloc() {
  let l = lua_l_newstate();
  assert!(!l.is_null());
  unsafe {
    build_shape(l);
    assert_values_intact(l);
    assert_traversal_clean(l);
    // #t 语义钉：数组段收缩后边界仍由 getn 在「数组+哈希」联合上导出，
    // 必须覆盖 EK（16385 有值 → 边界即 16385）
    assert_eq!(
      lua_objlen(l, 1),
      DENSE,
      "稀疏形态下 #t 是 getn 导出的边界（t[4000]≠nil 且 t[4001]==nil）"
    );
    lua_close(l);
  }
}

/// 探针分配器（poison + 强制搬移）：任何跨 newkey 复用旧数组槽位指针的
/// 悬垂读都会把 0xA5 垃圾搬进表里，被逐键断言与遍历断言当场抓住。
#[test]
fn shrink_migration_under_poison_allocator() {
  let g = probe_state();
  unsafe {
    build_shape(g.l);
    assert_values_intact(g.l);
    assert_traversal_clean(g.l);
    assert_eq!(lua_objlen(g.l, 1), DENSE, "#t 边界与默认分配器版不一致");
  }
}

/// 反复增删（洞 + 非数组键交替）压测 rehash 增长/收缩组合：终态逐键一致
#[test]
fn churn_rehash_cycles_keep_table_consistent() {
  let g = probe_state();
  let ls = g.l;
  unsafe {
    lua_createtable(ls, 4096, 0);
    // 交替写入：偶数整数键 + 字符串键 → 混合 rehash 增长
    let mut expect: Vec<(i32, f64)> = Vec::new();
    for round in 0..6 {
      for k in (2..4096).step_by(2 + round) {
        let v = f64::from(k) + round as f64;
        lua_pushnumber(ls, v);
        lua_rawseti(ls, 1, k);
        // 同键重复写入时先丢弃旧记账，expect 始终反映最终值
        expect.retain(|(ek, _)| *ek != k);
        expect.push((k, v));
      }
      for j in 0..500 {
        let name = format!("churn-{round}-{j}");
        lua_pushlstring(ls, name.as_ptr() as *const c_char, name.len());
        lua_pushnumber(ls, f64::from(j) - 0.5);
        lua_rawset(ls, -3);
      }
      // 挖掉一半偶数键 → 数组段出洞
      for k in (2..4096).step_by(2 + round).skip(1).step_by(2) {
        lua_pushnil(ls);
        lua_rawseti(ls, 1, k);
        expect.retain(|(ek, _)| *ek != k);
      }
    }
    for (k, v) in &expect {
      let tt = lua_rawgeti(ls, 1, *k);
      if tt == LuaType::Nil as i32 {
        panic!("churn 后键 {k} 丢失");
      }
      let got = read_number(ls);
      assert_eq!(got, *v, "churn 后键 {k} 取值被污染");
      lua_settop(ls, 1);
    }
  }
}
