---
name: sync-cpp
description: 同步上游 Luau C++ 改动至 ulua Rust 仓库，审查实现、运行测试并更新文档。
---

# 同步上游 Luau C++ 到 ulua

本指南详细说明如何将上游 Luau C++ 代码库的更新同步至纯 Rust 实现的 ulua 工作区。

## 0. 前提与代码库结构

- `cpp` 目录由 `init.sh` 初始化为一个独立的根级 Git 仓库（不是子模块），位于项目根目录下的 `cpp`。
- `init.sh` 会克隆团队 Fork 并注入上游远端：
  - `origin = ssh://git@ssh.github.com:443/webc-fork/luau.git`
  - `upstream = https://github.com/luau-lang/luau.git`
- `cpp` 使用 `master` 分支作为集成/工作分支。
- `init.sh` 来源的原始仓库是 `webc-fork/luau.git`。

### 0.1 当前 cpp 远端状态（已核实）

- 远端 `origin`：`ssh://git@ssh.github.com:443/webc-fork/luau.git`
- 远端 `upstream`：`https://github.com/luau-lang/luau.git`
- 本地分支：`master`
- 已追踪远端分支：`origin/master`
- `init.sh` 初始化完成后，通常不再需要手动添加 `upstream`。

## 1. 同步全流程概览

流程顺序如下：

1. 从上游拉取最新状态；
2. 合并上游目标版本/提交到 `master`；
3. 合并成功后，再对 Rust 代码进行移植与优化；
4. 通过测试与代码检查后，再推送到 fork。

### 1.1 拉取上游最新进度

```bash
# 切换至 master 分支，拉取团队 Fork 最新提交
git -C cpp checkout master
git -C cpp pull origin master

# 获取官方上游所有分支与标签
git -C cpp fetch upstream --tags
```

如果 `init.sh` 已正确注入远端，通常不需要手动添加 `upstream`。可通过以下命令确认：

```bash
git -C cpp remote -v
```

### 1.2 合并上游改动到 master

```bash
# 合并上游目标版本或提交到 master 分支
git -C cpp merge upstream/<目标标签或分支，如 0.737>
```

合并成功后，再继续移植与验证。

### 1.3 审查改动差异

```bash
# 查看最近提交记录
git -C cpp log -n 20 --oneline

# 对比具体代码改动
git -C cpp diff <旧哈希>..<新哈希>

# 仅查看文件变更统计与列表
git -C cpp diff --stat <旧哈希>..<新哈希>
```

## 2. 上游 C++ 模块与 Rust Crate 对应关系

Luau C++ 目录与 ulua 内部各 crate 保持严格对应：

- cpp/Ast -> crates/ulua-ast
- cpp/Compiler -> crates/ulua-compiler
- cpp/VM -> crates/ulua-vm
- cpp/Analysis -> crates/ulua-analysis
- cpp/CodeGen -> crates/ulua-code-gen
- cpp/Config -> crates/ulua-config
- cpp/Require -> crates/ulua-require
- cpp/Repl -> crates/ulua-repl-cli
- cpp/CLI -> crates/ulua-cli-lib, crates/ulua-analyze-cli
- cpp/tests -> crates/ulua-unit-test, crates/ulua-conformance-test

## 3. 将 C++ 改动移植到 Rust 的最佳实践

移植上游改动时的审查与编码准则：

1. 定位对应的 Rust 文件与符号：
   根据 C++ 源文件和函数名在相应 crate 中定位。例如：
   cpp/VM/src/lapi.cpp 对应 crates/ulua-vm/src/functions/lua_*.rs；
   cpp/Compiler/src/Compiler.cpp 对应 crates/ulua-compiler/src/methods/。

2. 遵循 Rust 风格与工程实践：
   - 简洁优雅：避免冗长的多层类型转换与不必要的 transmute，直接使用结构体字段访问或标准库方法。
   - 高效性能：虚拟机关键热点路径、解释器主循环及编译器 pass 避免引入额外堆分配与多余指针间接层。
   - 内存安全：尽可能发挥 Rust 安全抽象优势；在底层虚拟机因性能或 C-ABI 兼容需要原始指针与 unsafe 块时，使用清晰的指针操作，严格保证并注释安全不变式。
   - Rust 2024 规范：使用 unsafe extern "C" 声明 C 外部块，使用 #[unsafe(no_mangle)] 或 #[unsafe(export_name = "...")] 标注导出符号，涉及关键字时使用 r#gen 转义，访问静态可变变量时使用 &raw mut 或原子封装，避免隐式引用。

## 4. 验证与运行测试

代码移植完成后，必须运行全套测试以确保无语义退化与功能回归。

一键运行全部工作区测试（优先使用 nextest 并预编译测试二进制）：

./test.sh

针对单个 crate 运行测试：

cargo nextest run -p ulua-vm
cargo nextest run -p ulua-compiler
cargo nextest run -p ulua-analysis

移植单元测试：
- 检查 cpp/tests 中是否有新增测试用例。
- 在 crates/ulua-unit-test/src/tests/ 下新增对应 Rust 测试。
- 在 crates/ulua-unit-test/src/tests/mod.rs 中注册新模块。

代码静态检查与格式化：

./sh/clippy.sh

cargo fmt --check

确保遵循 rustfmt.toml 中配置的 4 空格缩进规范。

## 5. 更新项目文档

同步完成后需同步更新相关文档：

1. docs/CONFORMANCE.md：
   - 记录同步的上游 Git 提交哈希或版本号。
   - 更新单元测试与一致性测试通过数量。
   - 说明新支持的语法、功能特性、FastFlag 或有意保留的行为差异。

2. README.md：
   - 更新顶部的对标上游版本号与测试通过统计数据。

3. 提交信息规范：
   推荐使用符合约定式提交的格式：
   feat: sync upstream luau <版本或哈希> changes

## 6. 推送至 Fork 与主仓库

完成 Rust 代码修订、验证测试与文档更新后，再执行推送。

### 6.1 推送 C++ 仓库至团队 Fork

```bash
# 确保 cpp 仓库 master 分支已包含合并后的上游提交
git -C cpp checkout master
git -C cpp pull origin master

# 将合并后的 C++ 提交推送到团队 Fork
git -C cpp push origin master
```

### 6.2 提交主仓库（ulua）改动并推送到 Fork

```bash
# 暂存主仓库改动（包括 .rs 移植代码、文档及 cpp 指针/状态）
git add cpp crates/ docs/ .agents/

# 按照规范提交
git commit -m "feat: sync upstream luau <版本或哈希> changes"

# 推送到主仓库 Fork
git push
```

### 6.3 推送方向说明

- `cpp` 仓库与团队 Fork 同步时，仅涉及 `origin`：
  - `git -C cpp pull origin master`
  - `git -C cpp push origin master`
- 主仓库（ulua）推送时，相关 Rust 代码、文档与 `cpp` 状态一起提交到 ulua 的 Fork。
