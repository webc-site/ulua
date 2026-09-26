use alloc::{borrow::Cow, format};

use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::records::luarequire_configuration::luarequire_Configuration;

/// 配置面校验的缺口种类（文案与 cpp `validateConfig` 逐字一致）。
enum ConfigGap {
  /// 必填回调缺失（携带 cpp 中的字段名）
  Missing(&'static str),
  /// `get_alias` 与 `get_config` 同时提供
  BothAliasAndConfig,
  /// `get_alias` 与 `get_config` 都没提供
  MissingAliasOrConfig,
}

impl ConfigGap {
  /// 缺口 → cpp 原文报错文案。仅字段名分支需拼接，其余为静态串（`Cow` 免无谓分配）。
  fn message(self) -> Cow<'static, str> {
    match self {
      ConfigGap::Missing(name) =>
        format!("require configuration is missing required function pointer: {name}").into(),
      ConfigGap::BothAliasAndConfig =>
        "require configuration cannot define both get_alias and get_config".into(),
      ConfigGap::MissingAliasOrConfig =>
        "require configuration is missing required function pointer: either get_alias or get_config (not both)".into(),
    }
  }
}

/// cpp `validateConfig` 的**判定**部分：纯逻辑遍历配置字段，按 cpp 的逐条顺序
/// 返回首个缺口；全部齐备返回 `None`。不碰 `LuaState`，故完全在 `unsafe` 之外。
fn configuration_gap(config: &luarequire_Configuration) -> Option<ConfigGap> {
  // 必填指针清单（名称, 是否存在），顺序与 cpp 判定一致
  let required: [(&str, bool); 10] = [
    ("is_require_allowed", config.is_require_allowed.is_some()),
    ("reset", config.reset.is_some()),
    ("jump_to_alias", config.jump_to_alias.is_some()),
    ("to_parent", config.to_parent.is_some()),
    ("to_child", config.to_child.is_some()),
    ("is_module_present", config.is_module_present.is_some()),
    ("get_chunkname", config.get_chunkname.is_some()),
    ("get_loadname", config.get_loadname.is_some()),
    ("get_cache_key", config.get_cache_key.is_some()),
    ("get_config_status", config.get_config_status.is_some()),
  ];
  if let Some(name) = required
    .into_iter()
    .find_map(|(name, present)| (!present).then_some(name))
  {
    return Some(ConfigGap::Missing(name));
  }

  // get_alias 与 get_config 二选一
  if config.get_alias.is_some() && config.get_config.is_some() {
    return Some(ConfigGap::BothAliasAndConfig);
  }
  if config.get_alias.is_none() && config.get_config.is_none() {
    return Some(ConfigGap::MissingAliasOrConfig);
  }

  // cpp 把 load 放在二选一判定之后，顺序保持
  config.load.is_none().then_some(ConfigGap::Missing("load"))
}

/// # Safety
///
/// Caller must ensure `l` is a valid pointer to a `LuaState`.
pub(crate) unsafe fn validate_config(l: *mut LuaState, config: &luarequire_Configuration) {
  // 缺口判定与文案组装都是纯逻辑，留在 unsafe 之外；仅把错误抛回 VM 这一步是真边界。
  let Some(gap) = configuration_gap(config) else {
    return;
  };
  let message = gap.message();
  // Safety: l 为调用方（pushrequireclosureinternal，已持有效 state）传入的存活
  // LuaState；message 是本地串引用，lua_l_error_l 格式化后经 lua_error 必然发散、不返回。
  unsafe { luaL_error!(l, "{message}") }
}
