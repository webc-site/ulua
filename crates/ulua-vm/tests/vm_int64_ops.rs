//! int64 算术/位运算的直接契约测试（integer 库 → Rust `functions/int_64_*`）。
//!
//! 期望值全部从上游 cpp 语义推导，禁止按 Rust 实现现算「期望=实际」：
//! - `cpp/VM/src/lintlib.cpp`：`integer` 库 C 函数（本文件的逐条依据）
//! - `cpp/VM/src/lbuiltins.cpp`：`luauF_integer*` fastcall，语义与 lintlib 一致
//! - 本 cpp 快照无 LOP_ADDINT/LOP_LSHIFT 等 int64 专用操作码（该族指令在
//!   lintlib/lbuiltins 两处落地），故契约以这两处为准。
//!
//! 关键契约（cpp 依据）：
//! - add/sub/mul：`(uint64_t)` 域计算再回读 int64 → 溢出按 2^64 回绕（lintlib.cpp:70-98）
//! - div：除数 0 → "division by zero"；INT64_MIN/-1 → "integer overflow"；
//!   其余为 C++ `/`（向零截断）（lintlib.cpp:100-113）
//! - idiv：双守卫同 div；商为负且仍有余数时再减 1（lintlib.cpp:115-131）
//! - mod：除数 0 → 错误；余数非 0 且被除数/除数异号时 `remainder += b`
//!   （结果取除数符号）；INT64_MIN % -1 特判为 0（lintlib.cpp:153-172）
//! - band/bor/bxor：变参折叠，积元分别为全 1/0/0（空参时 band → -1、
//!   bor/bxor → 0）（lintlib.cpp:232-287）
//! - bnot：按位取补（二进制补码）（lintlib.cpp:264-271）
//! - lshift/rshift：以 uint64 做逻辑移位；守卫 `(i >= -63) && (i <= 63)`，
//!   越界一律返回 0（不是 `& 63` 掩码！），负 i 反向移（lintlib.cpp:369-393）
//!
//! 调用方式：`luaL_openlibs` 打开 integer 库后，从全局表取出各 C 函数，
//! 压入 LUA_TINTEGER 实参直接 `lua_pcall`——不走编译器，逐条命中 int_64_*。

use ulua_common::{fflag::LuauIntegerLibrary, functions::c_str::with_c_str};
use ulua_vm::{
  enums::{lua_status::LuaStatus, lua_type::LuaType},
  functions::{
    lua_close::lua_close, lua_getfield::lua_getfield, lua_gettop::lua_gettop,
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_pcall::lua_pcall,
    lua_settop::lua_settop, lua_tolstring::lua_tolstring_ref, lua_type::lua_type,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lvalue::lvalue, setlvalue::setlvalue},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// `LuauIntegerLibrary` 的线程局部覆盖守卫：`luaL_openlibs` 仅在该旗标为
/// true 时装载 integer 库（对照 cpp/VM/src/linit.cpp:44-47）。
struct IntegerLibFlag;

impl IntegerLibFlag {
  fn new() -> Self {
    LuauIntegerLibrary.push_test_override(true);
    Self
  }
}

impl Drop for IntegerLibFlag {
  fn drop(&mut self) {
    LuauIntegerLibrary.pop_test_override();
  }
}

/// 装载好的 VM 状态：栈底（索引 1）常驻 int64 库表，测试期间不变。
struct Lib {
  l: *mut LuaState,
  _flag: IntegerLibFlag,
}

impl Lib {
  fn new() -> Self {
    let flag = IntegerLibFlag::new();
    let l = lua_l_newstate();
    assert!(!l.is_null(), "lua_l_newstate 失败");
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      lua_l_openlibs(l);
      // 库名按 cpp：LUA_INTLIBNAME="integer"（cpp/VM/include/lualib.h:154 +
      // lintlib.cpp:605），luaopen_integer 以同名注册。
      lua_getfield(l, LUA_GLOBALSINDEX, c"integer".as_ptr());
      assert_eq!(
        lua_type(l, -1),
        LuaType::Table as i32,
        "openlibs 后全局表必须含 integer 库表"
      );
    }
    Self { l, _flag: flag }
  }

  /// 压入一个 LUA_TINTEGER 值（等价 cpp `lua_pushinteger64`，
  /// 即 `setlvalue`：`lua_pushinteger` 会把值转成 double，不能用）。
  /// 主线程栈在 `luaL_newstate` 时预分配，本文件单次调用至多占 4 槽。
  unsafe fn push_int64(&self, v: i64) {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      let slot = (*self.l).top;
      assert!(
        (slot as *const TValue) < (*self.l).stack_last as *const TValue,
        "主栈预留不足"
      );
      setlvalue!(&mut *slot, v);
      (*self.l).top = slot.add(1);
    }
  }

  /// `call`/`call_err` 的共同前奏：从栈底库表取函数并断言存在，压实参后
  /// `lua_pcall(nargs, 1, 0)`；返回（进入时的栈基，pcall 状态码）。
  unsafe fn pcall1(&self, fname: &str, args: &[i64]) -> (i32, i32) {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`base` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      let base = lua_gettop(self.l);
      with_c_str(fname.as_bytes(), |c_fname| {
        lua_getfield(self.l, 1, c_fname);
      });
      assert_eq!(
        lua_type(self.l, -1),
        LuaType::Function as i32,
        "库函数 {fname:?} 必须存在"
      );
      for a in args {
        self.push_int64(*a);
      }
      (base, lua_pcall(self.l, args.len() as i32, 1, 0))
    }
  }

  /// 调用 int64 库函数并断言成功，返回栈顶 int64 结果；调用后栈复原。
  unsafe fn call(&self, fname: &str, args: &[i64]) -> i64 {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；结果槽由本用例独占，至本行读取前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      let (base, status) = self.pcall1(fname, args);
      assert_eq!(status, 0, "{fname:?} 调用不应报错");
      assert_eq!(
        lua_type(self.l, -1),
        LuaType::Integer as i32,
        "{fname:?} 结果必须是 int64"
      );
      let v = lvalue!((*self.l).top.sub(1));
      lua_settop(self.l, base);
      v
    }
  }

  /// 调用并断言运行时错误（LUA_ERRRUN，cpp lua.h:29-31），返回错误消息。
  unsafe fn call_err(&self, fname: &str, args: &[i64]) -> String {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；错误对象槽由本用例独占，至本行读取前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      let (base, status) = self.pcall1(fname, args);
      assert_eq!(
        status,
        LuaStatus::ErrRun as i32,
        "{fname:?} 应报 LUA_ERRRUN"
      );
      // Safety: 错误对象槽只读；`None`（非字符串错误对象）收敛为断言失败。
      let msg = lua_tolstring_ref(self.l, -1)
        .map(|s| String::from_utf8_lossy(s).into_owned())
        .expect("错误对象必须是字符串");
      lua_settop(self.l, base);
      msg
    }
  }
}

impl Drop for Lib {
  fn drop(&mut self) {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe { lua_close(self.l) };
  }
}

/// cpp lintlib.cpp:70-78：`(int64_t)((uint64_t)x + (uint64_t)y)`，
/// 即模 2^64 回绕：MAX+1 = 0x7FFF…+1 = 0x8000… = MIN；MIN+MIN = 2^64 → 0。
#[test]
fn int64_add_wraps_mod_2_pow_64() {
  let lib = Lib::new();
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    assert_eq!(lib.call("add", &[1, 2]), 3);
    assert_eq!(lib.call("add", &[-5, -7]), -12);
    assert_eq!(lib.call("add", &[i64::MAX, 1]), i64::MIN);
    assert_eq!(lib.call("add", &[i64::MIN, -1]), i64::MAX);
    assert_eq!(lib.call("add", &[i64::MAX, -1]), 9223372036854775806);
    assert_eq!(lib.call("add", &[i64::MIN, i64::MIN]), 0);
  }
}

/// cpp lintlib.cpp:80-88：`(int64_t)((uint64_t)x - (uint64_t)y)`，
/// 0-MIN = 0 - 0x8000… = 0x8000… = MIN（负数取相反数在 MIN 处回绕）。
#[test]
fn int64_sub_wraps_mod_2_pow_64() {
  let lib = Lib::new();
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    assert_eq!(lib.call("sub", &[3, 5]), -2);
    assert_eq!(lib.call("sub", &[i64::MIN, 1]), i64::MAX);
    assert_eq!(lib.call("sub", &[i64::MAX, -1]), i64::MIN);
    assert_eq!(lib.call("sub", &[0, i64::MIN]), i64::MIN);
    assert_eq!(lib.call("sub", &[i64::MIN, i64::MIN]), 0);
  }
}

/// cpp lintlib.cpp:90-98：`(int64_t)((uint64_t)x * (uint64_t)y)`，
/// MAX*2 = 0xFFFF…FE → -2；MIN*-1 按补码仍得 MIN；2^32*2^32 = 2^64 → 0。
#[test]
fn int64_mul_wraps_mod_2_pow_64() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("mul", &[6, 7]), 42);
    assert_eq!(lib.call("mul", &[-3, -7]), 21);
    assert_eq!(lib.call("mul", &[i64::MAX, 2]), -2);
    assert_eq!(lib.call("mul", &[i64::MIN, -1]), i64::MIN);
    assert_eq!(lib.call("mul", &[4294967296, 4294967296]), 0);
  }
}

/// cpp lintlib.cpp:100-113：守卫后走 C++ `a / b`（C++11 起向零截断），
/// 因此 div(-20,3) = -6 而非 -7（与 idiv 的“负商再减一”形成对照）。
#[test]
fn int64_div_truncates_toward_zero() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("div", &[20, 3]), 6);
    assert_eq!(lib.call("div", &[-20, 3]), -6);
    assert_eq!(lib.call("div", &[20, -3]), -6);
    assert_eq!(lib.call("div", &[-20, -3]), 6);
    assert_eq!(lib.call("div", &[1, 2]), 0);
    assert_eq!(lib.call("div", &[-1, 2]), 0);
  }
}

/// cpp lintlib.cpp:105-108：除数 0 → "division by zero"；
/// (LLONG_MIN, -1) → "integer overflow"（真机除法会 UB，必须被守卫拦下）。
#[test]
fn int64_div_rejects_zero_and_min_over_neg_one() {
  let lib = Lib::new();
  unsafe {
    assert!(lib.call_err("div", &[7, 0]).contains("division by zero"));
    assert!(
      lib
        .call_err("div", &[i64::MIN, -1])
        .contains("integer overflow")
    );
  }
}

/// cpp lintlib.cpp:125-129：`result = a / b` 后，仅当 result < 0 且仍有余数
/// 才 result - 1。注意被除数绝对值小于除数的负数（如 -1/2）商截断为 0、
/// 不满足 result < 0，cpp 原样返回 0——这是 lintlib 的既定语义，须照抄。
#[test]
fn int64_idiv_decrements_negative_quotient_with_remainder() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("idiv", &[20, 3]), 6);
    assert_eq!(lib.call("idiv", &[-20, 3]), -7);
    assert_eq!(lib.call("idiv", &[20, -3]), -7);
    assert_eq!(lib.call("idiv", &[-20, -3]), 6);
    assert_eq!(lib.call("idiv", &[-1, 2]), 0);
    assert_eq!(lib.call("idiv", &[-6, 3]), -2);
    assert_eq!(lib.call("idiv", &[1, 2]), 0);
  }
}

/// cpp lintlib.cpp:120-123：idiv 与 div 共用同一对守卫。
#[test]
fn int64_idiv_rejects_zero_and_min_over_neg_one() {
  let lib = Lib::new();
  unsafe {
    assert!(lib.call_err("idiv", &[7, 0]).contains("division by zero"));
    assert!(
      lib
        .call_err("idiv", &[i64::MIN, -1])
        .contains("integer overflow")
    );
  }
}

/// cpp lintlib.cpp:153-172：C++ `%`（余数随被除数符号）后，若余数非 0 且
/// a、b 异号则 `remainder += b` → 结果符号随除数；(LLONG_MIN, -1) 时
/// remainder 保持初值 0（跳过真实取模，规避硬件 UB）。
#[test]
fn int64_mod_result_takes_divisor_sign() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("mod", &[7, 3]), 1);
    assert_eq!(lib.call("mod", &[-7, 3]), 2);
    assert_eq!(lib.call("mod", &[7, -3]), -2);
    assert_eq!(lib.call("mod", &[-7, -3]), -1);
    assert_eq!(lib.call("mod", &[-8, 4]), 0);
    assert_eq!(lib.call("mod", &[i64::MIN, -1]), 0);
  }
}

/// cpp lintlib.cpp:158-159：mod 仅守卫除数 0（无 INT64_MIN/-1 错误分支）。
#[test]
fn int64_mod_rejects_zero_divisor() {
  let lib = Lib::new();
  unsafe {
    assert!(lib.call_err("mod", &[7, 0]).contains("division by zero"));
  }
}

/// cpp lintlib.cpp:232-246：变参 `tres = ULLONG_MAX` 起头逐一个与；
/// 空参折叠回 ULLONG_MAX，`lua_pushinteger64` 按 int64 回读即 -1。
#[test]
fn int64_band_folds_from_all_ones() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("band", &[0xF0, 0x3C]), 0x30);
    assert_eq!(lib.call("band", &[-1, 15]), 15);
    assert_eq!(lib.call("band", &[15, 240, 51]), 0);
    assert_eq!(lib.call("band", &[]), -1);
    assert_eq!(lib.call("band", &[i64::MIN]), i64::MIN);
  }
}

/// cpp lintlib.cpp:248-262：`tres = 0` 起头逐一个或；空参得 0。
#[test]
fn int64_bor_folds_from_zero() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("bor", &[240, 15]), 255);
    assert_eq!(lib.call("bor", &[1, 2, 4]), 7);
    assert_eq!(lib.call("bor", &[-1, 5]), -1);
    assert_eq!(lib.call("bor", &[]), 0);
  }
}

/// cpp lintlib.cpp:273-287：`tres = 0` 起头逐一个异或，自身相消为 0。
#[test]
fn int64_bxor_folds_from_zero() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("bxor", &[5, 3]), 6);
    assert_eq!(lib.call("bxor", &[255, 15, 240]), 0);
    assert_eq!(lib.call("bxor", &[-1, -1]), 0);
    assert_eq!(lib.call("bxor", &[0, -1]), -1);
    assert_eq!(lib.call("bxor", &[]), 0);
  }
}

/// cpp lintlib.cpp:264-271：`~a`（无符号域按位取补后回读 int64），
/// 即二进制补码恒等式 `bnot(x) = -x - 1`。
#[test]
fn int64_bnot_is_twos_complement_flip() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("bnot", &[0]), -1);
    assert_eq!(lib.call("bnot", &[-1]), 0);
    assert_eq!(lib.call("bnot", &[41]), -42);
    assert_eq!(lib.call("bnot", &[i64::MIN]), i64::MAX);
    assert_eq!(lib.call("bnot", &[i64::MAX]), i64::MIN);
  }
}

/// cpp lintlib.cpp:369-380：n 取 uint64；守卫 `i >= -63 && i <= 63`，
/// 越界（含 ±64 及以上）直接推 0——不是 `& 63` 掩码回卷；i<0 时反向右移。
#[test]
fn int64_lshift_out_of_range_returns_zero() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("lshift", &[3, 2]), 12);
    assert_eq!(lib.call("lshift", &[1, 0]), 1);
    assert_eq!(lib.call("lshift", &[1, 63]), i64::MIN);
    assert_eq!(lib.call("lshift", &[i64::MAX, 1]), -2);
    assert_eq!(lib.call("lshift", &[-1, 1]), -2);
    assert_eq!(lib.call("lshift", &[1, 64]), 0);
    assert_eq!(lib.call("lshift", &[1, -64]), 0);
    assert_eq!(lib.call("lshift", &[1, -1]), 0);
    assert_eq!(lib.call("lshift", &[i64::MIN, -63]), 1);
  }
}

/// cpp lintlib.cpp:382-393：逻辑右移（uint64 域，不复制符号位）；
/// 越界同 lshift 推 0；i<0 时反向左移：rshift(-1,-63) = 0xFFFF…F<<63。
#[test]
fn int64_rshift_is_logical_and_clamps_out_of_range() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("rshift", &[255, 4]), 15);
    assert_eq!(lib.call("rshift", &[i64::MIN, 1]), 4611686018427387904);
    assert_eq!(lib.call("rshift", &[-1, 1]), 9223372036854775807);
    assert_eq!(lib.call("rshift", &[i64::MAX, 62]), 1);
    assert_eq!(lib.call("rshift", &[i64::MIN, 62]), 2);
    assert_eq!(lib.call("rshift", &[1, 64]), 0);
    assert_eq!(lib.call("rshift", &[1, -1]), 2);
    assert_eq!(lib.call("rshift", &[-1, -63]), i64::MIN);
  }
}

/// cpp lintlib.cpp:200/216：max/min 变参择优，以 1 号实参为初值逐个比较——
/// 含单实参恒等（循环零次）、负值域与 i64 两端边界。
#[test]
fn int64_max_min_pick_extreme() {
  let lib = Lib::new();
  unsafe {
    assert_eq!(lib.call("max", &[3, 1, 2]), 3);
    assert_eq!(lib.call("max", &[-5, -1, -3]), -1);
    assert_eq!(lib.call("max", &[i64::MIN, 0, i64::MAX]), i64::MAX);
    assert_eq!(lib.call("max", &[42]), 42);
    assert_eq!(lib.call("min", &[3, 1, 2]), 1);
    assert_eq!(lib.call("min", &[-5, -1, -3]), -5);
    assert_eq!(lib.call("min", &[i64::MIN, 0, i64::MAX]), i64::MIN);
    assert_eq!(lib.call("min", &[-42]), -42);
  }
}
