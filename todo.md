# C 风格调用 Rust 化 — 进度

## 已完成

- [x] memchr/memcmp → memchr::memmem::find (lmemfind.rs)
- [x] strtod → fast-float2 + parse_c_double 全平台统一 (luai_str_2_num.rs, strtod_shim.rs)
- [x] strtod_shim 去 wasm-only cfg，全平台导出 (lib.rs)
- [x] match_keyword_ci 用 eq_ignore_ascii_case 现代化 (strtod_shim.rs)
- [x] snprintf → fmt_cstr_buf + format_args! (enumthread/closure/proto/object/class/table)
- [x] 审查反馈：lmemfind 补 Safety 文档 + .cast() + map_or
- [x] 审查反馈：luai_str_2_num 用 .cast_mut()
- [x] 审查反馈：去掉 ulua-vm 冗余 fast-float2 依赖

## 待做

### strtoul / strtoull 纯 Rust 化

文件：
- crates/ulua-vm/src/functions/lua_o_str_2_d.rs — strtoul(s, endptr, 16)
- crates/ulua-vm/src/functions/lua_o_str_2_l.rs — strtoull(s, endptr, base)
- crates/ulua-vm/src/functions/lua_b_tonumber.rs — strtoull(s, endptr, base)

改法：手写十六进制/任意进制解析（跳空白 → 解析数字 → 设 endptr），10-20 行状态机
或者复用 strtod_shim 中的模式，在 ulua-common 统一暴露

### snprintf 残留（format_directive.rs 测试 oracle）

文件：crates/ulua-vm/src/functions/format_directive.rs (第 549 行)

这是测试代码中的 C oracle（对照 C snprintf 验证 Rust 实现正确性），不影响运行时。
保留还是删除取决于是否仍需要 C oracle 交叉验证。

### 时钟 FFI 整理（阶段 3）

macOS mach_absolute_time / mach_timebase_info：
- crates/ulua-vm/src/functions/clock_timestamp.rs
- crates/ulua-vm/src/functions/clock_period.rs
- crates/ulua-common/src/functions/get_clock_timestamp.rs

可选：cargo add mach2 替代手写 extern C 声明
可选：合并 vm 和 common 两组重复时钟实现

Linux clock_gettime：
- crates/ulua-vm/src/functions/clock_timestamp.rs (手写 extern C)
- crates/ulua-common/src/functions/get_clock_timestamp.rs (用 libc crate)

可选：统一为 libc crate 或统一到 ulua-common

### 不动的部分

- free/realloc (l_alloc.rs) — VM C ABI 分配器架构需要
- pthread 栈探测 (native_stack_guard) — 无 Rust 替代品
- localtime_r/gmtime_r/strftime/time — 已有纯 Rust 实现 + C oracle
- mmap/mprotect (paged_allocate/freeze/codegen) — 平台 FFI 底层，低优先级
