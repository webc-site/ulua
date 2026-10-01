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
      // 两个 resolver（fe-selfptr 挂账⑥收口）已随 `Fixture` 字段 `Box` 钉堆：
      // `&mut` 引用所指堆地址在 `Fixture` 生命周期内恒定，构造期这一次布线
      // （`NonNull::from` 记录裸句柄）即永久有效，此前「每次访问前刷新句柄」
      // 的绕行就此移除。
      self.frontend = Some(Frontend::new_boxed(
        mode,
        // 显式 deref：让泛型实参 `F` 推导为 `TestFileResolver` 本体而非 Box。
        &mut *self.file_resolver,
        Some(&mut self.config_resolver.base),
        options,
      ));
      self.config_resolver.default_config.mode = Mode::Strict;
      self
        .config_resolver
        .default_config
        .enabled_lint
        .warning_mask = !0u64;
      self
        .config_resolver
        .default_config
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
