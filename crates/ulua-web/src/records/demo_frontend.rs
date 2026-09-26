//! demo `Frontend` 会话：拥有一对 demo resolver 与自引用的 `Frontend`，
//! 把 C++ `Web.cpp` 里「构造 → 落位 → `wireSelfPointers` → 注册内置全局」的
//! 裸指针布线契约收敛进本类型，令 `check_script` 等入口保持纯安全代码。

use std::string::{String, ToString};

use itoa::Buffer;
use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{freeze::freeze, to_string_error::to_string_type_error, unfreeze::unfreeze},
  records::{frontend::Frontend, frontend_options::FrontendOptions},
  type_aliases::module_name_type::ModuleName,
};
use ulua_common::fflag;

use crate::records::{
  demo_config_resolver::DemoConfigResolver, demo_file_resolver::DemoFileResolver,
};

/// 一次 demo 类型检查会话。
///
/// `Frontend` 是 C++ 移植来的自引用结构（`builtin_types`、两个 module_resolver
/// 均指向自身），契约要求「落位后不得移动」。这里用 `Box` 把 frontend 固定堆上
/// （Box 句柄可自由移动，堆内容地址恒定），从类型结构上消灭栈上落位假设。
pub(crate) struct DemoFrontend {
  /// Drop 按字段声明序：frontend 先析构、resolver 后析构，与原实现
  /// （栈局部逆序析构）一致；frontend 持有的 resolver 裸指针在此期间不会被回访。
  ///
  /// 三个指针字段全部 `Box` 固定堆地址：`Frontend` 内存的是构造时布线的裸
  /// 指针，任何一次对 resolver/frontend 值的移动都会让其内部指针悬垂；Box
  /// 句柄可自由移动而堆内容恒定，从类型结构上消灭「落位后不得移动」假设。
  frontend: Box<Frontend>,
  /// 被 `frontend` 以裸指针引用，须与其同生共死；本类型不再直接读它
  /// （读取全部经 frontend 内部的 `*mut ConfigResolver`），前缀 `_` 声明该意图。
  _config_resolver: Box<DemoConfigResolver>,
  /// 同 [`Self::_config_resolver`]；`source` 表在每次检查前被清空重写。
  file_resolver: Box<DemoFileResolver>,
}

impl DemoFrontend {
  /// 对应 `CLI/src/Web.cpp:142-182` 的构造段：建 resolver → 构造 `Frontend` →
  /// 设求解器模式 → 注册 Luau 内置全局。
  pub(crate) fn new(use_new_solver: bool) -> Self {
    let mut file_resolver = Box::new(DemoFileResolver::default());
    // cpp `DemoConfigResolver()` 构造即默认配置，故 `default()` 直接对应
    // （原 `new()` 只是 `default()` 的转发，已删）。
    let mut config_resolver = Box::new(DemoConfigResolver::default());
    let options = FrontendOptions::default();

    // 构造入参为 `&mut dyn FileResolver`：`&mut *Box` 在实参位隐式 unsizing。
    // `new_boxed` 在 safe 边界内完成「构造 → 堆上落位 → 自指针布线」全序列，
    // 调用点免手写 unsafe ctor + `wire_self_pointers`；resolver 以 `&mut` 引用
    // 传入、被 `Box` 钉住堆地址，与本结构同生命周期，满足其外部句柄长寿契约。
    // solver mode 与原 `frontend_file_resolver_config_resolver_frontend_options`
    // 一致地由 `LuauSolverV2` 快标志派生（该模式在构造期决定 `GlobalTypes` 建法）。
    let mode = if fflag::LuauSolverV2.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    };
    let frontend = Frontend::new_boxed(
      mode,
      &mut *file_resolver,
      Some(&mut config_resolver.base),
      options,
    );

    let mut this = Self {
      frontend,
      _config_resolver: config_resolver,
      file_resolver,
    };
    this.frontend.set_luau_solver_mode(if use_new_solver {
      SolverMode::New
    } else {
      SolverMode::Old
    });
    this.register_builtin_globals();
    this
  }

  /// C++ `unfreeze → registerBuiltinGlobals → freeze`（`Web.cpp:156-158`）。
  ///
  /// 上游 `registerBuiltinGlobals(Frontend&, GlobalTypes&)` 的两个引用实参在 Rust
  /// 里无法由同一个 place 拆出不重叠借用（E0499），该拆借已在 #6 wave2 步④
  /// 收进 ulua-analysis 的单 `&mut Frontend` 门面
  /// [`Frontend::register_builtin_globals`]（重叠别名窗口只剩库内一处，且
  /// `globals` 目标只能取自 frontend 自身的两张内置表），故本入口全为安全借用。
  fn register_builtin_globals(&mut self) {
    unfreeze(self.frontend.globals.global_types_mut());
    self.frontend.register_builtin_globals(false);
    freeze(self.frontend.globals.global_types_mut());
  }

  /// 复用会话检查单模块源码（对应 `Web.cpp` 的 restart 段）：清空 frontend 与
  /// `source` 表，注入源码后检查，返回换行连接的 `line: message` 诊断（无错为空串）。
  pub(crate) fn check_source(&mut self, module: &str, source: &str) -> String {
    // frontend.clear(); fileResolver.source.clear();
    self.frontend.clear();
    self.file_resolver.source.clear();

    // fileResolver.source[module] = source;
    let name: ModuleName = module.into();
    self
      .file_resolver
      .source
      .insert(name.clone(), source.to_string());

    // Luau::CheckResult checkResult = frontend.check("main");
    let check_result = self
      .frontend
      .check_module_name_optional_frontend_options(&name, None);

    let mut out = String::new();
    for err in &check_result.errors {
      if !out.is_empty() {
        out.push('\n');
      }
      // std::to_string(err.location.begin.line + 1)
      // itoa 栈缓冲直拼，免 `to_string()` 的堆分配；十进制输出与 `core::fmt` 逐字节一致
      out.push_str(Buffer::new().format(err.location.begin.line + 1));
      out.push_str(": ");
      // Luau::to_string(err)
      out.push_str(&to_string_type_error(err));
    }
    out
  }
}
