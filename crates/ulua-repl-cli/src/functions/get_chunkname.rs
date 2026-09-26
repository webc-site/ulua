//! cpp `ReplRequirer.cpp` 的 `static get_chunkname`：`"@" + vfs.getFilePath()`。
//! 回调体为 `functions/mod.rs` 模板宏 `requirer_write_cb!` 的一次调用，
//! C-ABI 签名与安全契约见宏模板。

requirer_write_cb!(get_chunkname | req -> Some(format!("@{}", req.vfs.real_path).as_str()));
