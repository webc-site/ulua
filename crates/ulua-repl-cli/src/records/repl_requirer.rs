//! cpp `ReplRequirer.h` 的 `struct ReplRequirer` + `ReplRequirer.cpp` 的全部
//! require 宿主回调。原 C-ABI 函数指针表形态（每回调独立 `static` + `ctx`
//! 裸指针重建）在此收口为 [`RequireHost`] 的 Rust 原生实现：路径/组件按字节
//! 串入参、以值返回，无缓冲区写回协议。
//!
//! 导航会改 vfs 状态而 trait 接收者为 `&self`（require 允许重入：循环 require
//! 时外层 `load` 未返回、内层 require 已在同一宿主上导航），故 vfs 以
//! `RefCell` 内部可变持有——对应 cpp 里同一 `ReplRequirer` 实例被回调直接改写。

use std::{borrow::Cow, cell::RefCell};

use ulua_cli_lib::{
  functions::{
    convert_config_status::convert_config_status,
    convert_navigation_status::convert_navigation_status, is_absolute_path::is_absolute_path,
    is_file::is_file, is_require_allowed::is_require_allowed,
  },
  records::vfs_navigator::VfsNavigator,
};
use ulua_require::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
  },
  records::navigation_context::RequireHost,
};
use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::load::load,
  type_aliases::{bool_check::BoolCheck, compile_options::CompileOptions, coverage::Coverage},
};

#[derive(Debug, Clone)]
pub struct ReplRequirer {
  pub(crate) copts: CompileOptions,
  pub(crate) coverage_active: BoolCheck,
  pub(crate) codegen_enable: BoolCheck,
  pub(crate) coverage_track: Coverage,
  pub(crate) counters_active: BoolCheck,
  pub(crate) counters_track: Coverage,
  pub(crate) vfs: RefCell<VfsNavigator>,
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
      vfs: RefCell::new(VfsNavigator::default()),
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

/// require 宿主字节串入参的 Rust 化：先按首个 NUL 截断（cpp 回调收到的是 C
/// 串，消费规则即 `strlen`），再 lossy 转文本（非 UTF-8 字节 → U+FFFD，与
/// 原 `cstr_cow` 门面一致）；下游宿主文件系统接口为 `&str` 形态。
pub(crate) fn host_str(bytes: &[u8]) -> Cow<'_, str> {
  let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
  String::from_utf8_lossy(&bytes[..end])
}

impl RequireHost for ReplRequirer {
  fn is_require_allowed(&self, requirer_chunkname: &[u8]) -> bool {
    is_require_allowed(requirer_chunkname)
  }

  fn reset_to_requirer(&self, requirer_chunkname: &[u8]) -> NavigateResult {
    // 对应 cpp `reset`：`=stdin` 复位到 stdin 目录；`@path` 复位到该文件目录；
    // 其余形态无从定位，NotFound。
    let name = host_str(requirer_chunkname);
    if name == "=stdin" {
      convert_navigation_status(self.vfs.borrow_mut().reset_to_std_in())
    } else if let Some(path) = name.strip_prefix('@') {
      convert_navigation_status(self.vfs.borrow_mut().reset_to_path(path))
    } else {
      NavigateResult::NotFound
    }
  }

  fn jump_to_alias(&self, alias_path: &[u8]) -> NavigateResult {
    // cpp `jumpToAlias`：仅接受绝对路径
    let path = host_str(alias_path);
    if is_absolute_path(&path) {
      convert_navigation_status(self.vfs.borrow_mut().reset_to_path(&path))
    } else {
      NavigateResult::NotFound
    }
  }

  fn to_parent(&self) -> NavigateResult {
    convert_navigation_status(self.vfs.borrow_mut().to_parent())
  }

  fn to_child(&self, component: &[u8]) -> NavigateResult {
    let name = host_str(component);
    convert_navigation_status(self.vfs.borrow_mut().to_child(&name))
  }

  fn is_module_present(&self) -> bool {
    is_file(&self.vfs.borrow().real_path)
  }

  fn get_chunkname(&self) -> Option<Vec<u8>> {
    Some(format!("@{}", self.vfs.borrow().real_path).into_bytes())
  }

  fn get_loadname(&self) -> Option<Vec<u8>> {
    Some(self.vfs.borrow().absolute_real_path.clone().into_bytes())
  }

  fn get_cache_key(&self) -> Option<Vec<u8>> {
    // cpp ReplRequirer.cpp: get_cache_key 与 get_loadname 同为绝对实路径
    // （getCacheKey/getLoadName 函数体逐字一致），不另立实现
    self.get_loadname()
  }

  fn get_config_status(&self) -> ConfigStatus {
    convert_config_status(self.vfs.borrow().get_config_status())
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    // cpp 侧仅登记 get_config 槽（未设 get_alias），行为即 GetConfig
    ConfigBehavior::GetConfig
  }

  fn get_config(&self) -> Option<Vec<u8>> {
    self.vfs.borrow().get_config().map(String::into_bytes)
  }

  fn load(&self, l: *mut LuaState, path: &[u8], chunkname: &[u8], loadname: &[u8]) -> i32 {
    // Safety: `l` 是 ulua-require 在 require 同步执行窗口内交出的活跃状态
    // （trait `load` 方法契约）；chunkname/loadname/path 为该窗口内存活字节串。
    unsafe { load(self, l, path, chunkname, loadname) }
  }
}
