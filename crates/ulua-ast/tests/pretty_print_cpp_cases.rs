//! cpp PrettyPrinter 语料（`cpp/tests/PrettyPrinter.test.cpp`，212 个
//! TEST_CASE/TEST_CASE_FIXTURE）在 Rust 侧的缺口登记与补测位。
//!
//! 清点结论（tst-r27，真实命令输出，数字可复量）：
//! - 语料本体 197/212 例已逐字移植于
//!   `crates/ulua-unit-test/tests/pretty_printer.rs`，实测
//!   `197 passed; 0 failed`（该文件与本 crate 走同一组 ulua-ast 公开入口：
//!   `pretty_print_string_view_parse_options_bool_bool` /
//!   `pretty_print_with_types_ast_stat_block` / `to_string_ast_node`），
//!   故不在 ulua-ast 内重复搬运（review.md §8：对照 cpp 补「Rust 缺」的
//!   用例；跨 crate 复制同一断言不增加 oracle 价值）。
//! - 本 crate 的 `tests/pretty_print.rs` 另有 11 枚 crate 级 golden
//!   （其中逐字对齐 cpp 的仅 `prettyPrint_parse_error`，cpp:1723）。
//! - 仓库级真缺口 15 例，去向如下：
//!   1. `if local`/`if const` 家族 12 例（cpp:92,106,114,126,134,142,153,
//!      161,172,186,194,206）——本端口 parser 未接入该语法：实测解析
//!      `if local result = getValue() then use(result) end` 报
//!      `Expected identifier when parsing expression, got 'local'`（cpp 侧由
//!      `DebugLuauIfLocalSyntax` 门控，ulua-common 未定义该旗标；
//!      `parser_parse_if_else_expr.rs:59` 已钉桩）。需 src 改动 → 待办。
//!   2. `attach_type_negate` 1 例（cpp:1032）——依赖 Fixture/TypeChecker 的
//!      `decorateWithTypes` 推断输出，ulua-ast 无法反向依赖分析层 → 待办。
//!   3. `pretty_print_incomplete_attr_list`（cpp:2763）与
//!      `pretty_print_incomplete_attr_args`（cpp:2775）2 例——**Rust 相对
//!      cpp 的真实缺陷**：不完整 `@[...]` 属性列表在 cpp 侧按 CST 原文逐字
//!      回环，Rust 侧 CST 回退丢失、打印器改以 AST 归一重排（两侧文本已录
//!      入 tst-r27 测试回报待裁决）。按「发现分歧不改实现、不水用例、不进
//!      提交」纪律，此二例暂不入库，裁决后以 cpp 逐字期望补入本文件。
