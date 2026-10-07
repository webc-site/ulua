use ulua_analysis::{
  enums::solver_mode::SolverMode,
  records::{frontend::Frontend, frontend_options::FrontendOptions},
};
use ulua_ast::enums::mode::Mode;
use ulua_common::fflag;

use crate::{functions::freeze_globals_pair::freeze_globals_pair, records::fixture::Fixture};
impl Fixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    let newly_initialized = self.frontend.is_none();

    if newly_initialized {
      let mode = if fflag::DebugLuauForceOldSolver.get() {
        SolverMode::Old
      } else {
        SolverMode::New
      };

      let options = FrontendOptions {
        retain_full_type_graphs: true,
        for_autocomplete: false,
        run_lint_checks: false,
        ..Default::default()
      };
      // `new_boxed` 在 safe 边界内完成「构造 → 堆上落位 → 自指针布线」全序列：
      // Frontend 落在 Box 钉死的堆地址上，`builtin_types` 与两个 module_resolver
      // 的自指针从此恒有效（原「unsafe ctor + 搬入 self.frontend + 每次调用重跑
      // wire_self_pointers」的悬置/搬移窗口就此消失）。
      self.frontend = Some(Frontend::new_boxed(
        mode,
        // 移交解析器所有权：`Frontend` 独占克隆，可变状态经 `Rc` 共享槽与
        // `self.file_resolver` 互通（见 `TestFileResolver` 结构体注），测试侧
        // 后续读写照旧可见，无需第二把可变别名。
        Box::new(self.file_resolver.clone()),
        // 移交配置解析器所有权：`Frontend` 独占 `Box<dyn ConfigResolver>`，夹具侧
        // 另持一份克隆；两者的 `default_config` / `config_files` 是同一 `Rc` 共享槽，
        // 故移交后夹具继续改写仍对 frontend 可见（等价 cpp 裸句柄别名的可见性，
        // 借用窗不重叠的单线程契约见 `TestConfigResolver` 类型注）。
        self.config_resolver.clone(),
        options,
      ));
      self.config_resolver.default_config_mut().mode = Mode::Strict;
      self
        .config_resolver
        .default_config_mut()
        .enabled_lint
        .warning_mask = !0u64;
      self
        .config_resolver
        .default_config_mut()
        .parse_options
        .capture_comments = true;
    }

    let frontend = self
      .frontend
      .as_mut()
      .expect("fixture.frontend 由 new_boxed 置位");

    if newly_initialized {
      freeze_globals_pair(frontend);
    }

    // 缓存 Frontend 自指针的可变句柄地址（经 chokepoint 取，保留原 mutable
    // provenance，供 fixture_get_builtins 等据此物化 &mut 借用）。Frontend 已
    // `Box` 钉址，该缓存自首次置位起恒有效。
    self.builtin_types = frontend.builtin_types_handle().as_ptr();
    frontend
  }
}
