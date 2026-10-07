//! ulua C ABI 导出层：从 ulua-vm 拆出的 `ulua_*` 符号壳。
//! 全 workspace 唯一允许保留 `unsafe` 的 FFI 边界：每处 unsafe 均带 `# Safety`
//! 契约（透传壳单源定义于 `functions/shells.rs` 模板宏），且块体收在最小范围；
//! `unsafe` 与 `c_char`/NUL 结尾处理只存在于本层，不向其它 crate 泄漏。
//! 多数壳仅逐参数透传（零逻辑）；少数壳做 C 表示适配（`*const c_char`→`&str`
//! 经 `cstr` 辅助单源转换、裸指针→引用重建），适配点逐壳在 `/// # Safety`
//! 契约中声明。
//! panic 边界策略：全部壳统一 `extern "C-unwind"`，宿主回调（`Pfunc`）同为
//! C-unwind 签名，Rust panic 经展开正常传播，不以 `catch_unwind` 静默截断，
//! 亦无 plain `extern "C"` 帧被展开的 UB 面。
//! 数据符号说明：vm 内部哨兵/表（`luaH_dummynode`、`luaO_nilobject_`、
//! `LUAU_F_TABLE`）在 oracle 中是 VM 内部链接符号（ltable.h/lobject.h/
//! lbuiltins.h 均非公共 API 头），且指针同一性以 vm 内部地址为准——独立副本
//! 导出只会给 C 宿主错误的比较基准，故不以 `export_name` 复制导出。

mod cstr;
pub mod functions;
