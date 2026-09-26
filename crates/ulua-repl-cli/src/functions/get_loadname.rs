//! cpp `ReplRequirer.cpp` 的 `static get_loadname`：`vfs.getAbsoluteFilePath()`；
//! `get_cache_key` 与之同为绝对实路径（require_config_init 把本回调同时挂到
//! `get_cache_key` 槽）。回调体为 `functions/mod.rs` 模板宏 `requirer_write_cb!`
//! 的一次调用，C-ABI 签名与安全契约见宏模板。

requirer_write_cb!(get_loadname | req -> Some(req.vfs.absolute_real_path.as_str()));
