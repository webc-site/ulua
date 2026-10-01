//! `Frontend::register_builtin_globals` —— cpp `registerBuiltinGlobals(frontend,
//! frontend.globals[, typeCheckForAutocomplete])`（`cpp/tests/Fixture.cpp:797-799`、
//! `cpp/CLI/src/Analyze.cpp:488`、`cpp/CLI/src/Web.cpp:157`）的单 `&mut Frontend`
//! 门面（#6 wave2 步④ chokepoint）。
//!
//! 上游 C++ 签名是 `(Frontend&, GlobalTypes&)`：两个形参在 Rust 里必然由同一个
//! place 拆出重叠可变借用（E0499），原先每个调用点都得就地手写
//! `*mut Frontend` + 字段裸句柄，共 7 处、契约各写各的。本文件把这层拆借收进
//! 类型自身唯一一处：调用方一律 `frontend.register_builtin_globals(..)`，
//! 重叠别名只在本模块内产生，且 `globals` 目标只能取自 `self` 的两个内置字段
//! （`globals` / `globals_for_autocomplete`），外部无法再传入任意 `GlobalTypes`。

use core::ptr::{self, NonNull};

use crate::{
  functions::register_builtin_globals::register_builtin_globals,
  records::{frontend::Frontend, global_types::GlobalTypes},
};

/// 注册目标全局表：与 cpp 的两个调用形态一一对应，不对外暴露以避免再传入
/// 任意 `GlobalTypes`（那等于把重叠借用的选择权放回调用点）。
enum GlobalsTarget {
  /// `registerBuiltinGlobals(f, f.globals)`
  Main,
  /// `registerBuiltinGlobals(f, f.globalsForAutocomplete, true)`
  ForAutocomplete,
}

impl Frontend {
  /// cpp `registerBuiltinGlobals(frontend, frontend.globals, typeCheckForAutocomplete)`。
  pub fn register_builtin_globals(&mut self, type_check_for_autocomplete: bool) {
    self.register_builtin_globals_target(GlobalsTarget::Main, type_check_for_autocomplete);
  }

  /// cpp `registerBuiltinGlobals(frontend, frontend.globalsForAutocomplete,
  /// /*typeCheckForAutocomplete*/ true)`（`cpp/tests/Fixture.cpp:799`）：注册进
  /// 补全专用的那套全局表，形参不可塌成主 `globals`，否则写错对象。
  pub fn register_builtin_globals_for_autocomplete(&mut self) {
    self.register_builtin_globals_target(GlobalsTarget::ForAutocomplete, true);
  }

  /// 唯一产生「整体 + 字段」重叠别名的地方，等价于原 7 处调用点的拆借形态。
  fn register_builtin_globals_target(
    &mut self,
    target: GlobalsTarget,
    type_check_for_autocomplete: bool,
  ) {
    // Safety: `this` 取自本 `&mut Frontend` 独占借用，`globals` 是该对象自身的
    // `GlobalTypes` 字段（`addr_of_mut!` 只取字段地址、不物化引用），两指针同源
    // 且必然指向同一存活堆块内的互异地址。被调方对两个参数的使用全程单线程
    // 顺序进行：任一语句只经一条路径触碰 `global_types` 字段（见
    // `functions/register_builtin_globals.rs` 内 `arena` 的串行不变量），两条
    // 可变借用均在 `&mut` 实参位、止于各自调用返回，无并存别名物化。
    let (this, globals): (*mut Frontend, *mut GlobalTypes) = unsafe {
      let this = NonNull::from(&mut *self).as_ptr();
      let globals = match target {
        GlobalsTarget::Main => ptr::addr_of_mut!((*this).globals),
        GlobalsTarget::ForAutocomplete => ptr::addr_of_mut!((*this).globals_for_autocomplete),
      };
      (this, globals)
    };
    // Safety: 上方句柄同源且指向存活对象；被调方为 `pub(crate)` 的唯一实现，
    // 其内部对 `frontend`/`globals` 的按序独占使用由本函数的单线程调用序保证。
    unsafe { register_builtin_globals(&mut *this, &mut *globals, type_check_for_autocomplete) };
  }
}
