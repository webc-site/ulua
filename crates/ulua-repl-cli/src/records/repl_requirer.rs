//! cpp `ReplRequirer.h` 的 `struct ReplRequirer` + 构造函数
//! （`ReplRequirer.cpp` 末尾）；探针方法对应 cpp 里 `req->coverageActive()` 等
//! 直接调用成员函数指针的写法。

use ulua_cli_lib::records::vfs_navigator::VfsNavigator;

use crate::type_aliases::{
  bool_check::BoolCheck, compile_options::CompileOptions, coverage::Coverage,
};

#[derive(Debug, Clone)]
pub struct ReplRequirer {
  pub(crate) copts: CompileOptions,
  pub(crate) coverage_active: BoolCheck,
  pub(crate) codegen_enable: BoolCheck,
  pub(crate) coverage_track: Coverage,
  pub(crate) counters_active: BoolCheck,
  pub(crate) counters_track: Coverage,
  pub(crate) vfs: VfsNavigator,
}

impl ReplRequirer {
  /// cpp `ReplRequirer(copts, coverageActive, codegenEnabled, coverageTrack,
  /// countersActive, countersTrack)`；`vfs` 在 cpp 中默认构造，此处一致。
  pub(crate) fn new(
    copts: CompileOptions,
    coverage_active: BoolCheck,
    codegen_enable: BoolCheck,
    coverage_track: Coverage,
    counters_active: BoolCheck,
    counters_track: Coverage,
  ) -> Self {
    Self {
      copts,
      coverage_active,
      codegen_enable,
      coverage_track,
      counters_active,
      counters_track,
      vfs: VfsNavigator::default(),
    }
  }

  pub(crate) fn coverage_active(&self) -> bool {
    (self.coverage_active)()
  }

  pub(crate) fn codegen_enable(&self) -> bool {
    (self.codegen_enable)()
  }

  pub(crate) fn counters_active(&self) -> bool {
    (self.counters_active)()
  }
}
