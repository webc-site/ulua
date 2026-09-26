//! cpp `ReplRequirer.cpp` 的 `static get_config`：读取到的配置文件内容写回缓冲。
//! 回调体为 `functions/mod.rs` 模板宏 `requirer_write_cb!` 的一次调用，
//! C-ABI 签名与安全契约见宏模板；`get_config()` 的 `Option<String>` 在本条
//! 语句内 `.as_deref()`，临时量活到 write 调用结束。

requirer_write_cb!(get_config | req -> req.vfs.get_config().as_deref());
