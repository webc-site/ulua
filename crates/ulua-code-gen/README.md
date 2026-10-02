# Ulua Code Gen

Native A64/X64 JIT code generation for compiled Luau bytecode.

### Add to your project

```toml
[dependencies]
ulua-code-gen = "0.1.0"
```

## Links

- Docs: https://docs.rs/ulua-code-gen
- Repository: https://github.com/webc-site/ulua

## 已解决：native 入口被 CHECK_SAFE_ENV 全量弹回（2026-10 定案）

**根因**：`lua_l_openlibs` 漏译 cpp linit.cpp:105 的 `lua_setsafeenv(L, LUA_GLOBALSINDEX, true)`
——safeenv 恒 0，生成码每个 `implicit CHECK_SAFE_ENV` guard（GET_CACHED_IMPORT 等全局
读取路径）都当场 vmExit 弹回解释器。凡带全局读取的模块（nbody/oop/fasta 等——真实
Luau 代码的绝大多数）JIT 等于关闭；纯局部变量脚本（fannkuch/p4）不受影响，故此前
「部分用例 3.9x、部分零收益」的割裂表象掩盖了系统性缺陷。

修复：safeenv=1 挪到 `luau_codegen_create`（JIT 启用点）激活——纯解释路径保持旧值
（解释器 import 缓存 × userdata 全局在 safeenv=1 下另有待查问题：ulua-rt
tests::test_fields，6572 项其余全绿）。已知刻意偏差：cpp linit 同时 setreadonly
(GLOBALS)，本仓库嵌入 API 契约是可写全局，未随。

实测（exec_probe，同 binary 前后）：nbody 74ms→17ms（4.4x）、fasta -50%、
microbig_gettable -29%、oop -16%，samply 证实环路 100% 落在 JIT 代码段。
bench exec 组 A/B 持平的量化解释：runner 每次 eval 新建 state，**native codegen pass
（约 50ms/6KB，cpp 同量级任务 1-2ms）在计时窗内**，恰好吃掉 57ms 的执行节省；
codegen 吞吐是下一个独立优化靶点。

表访问 IC 三方案（动态键内联 / 两层 __index+mini-PIC / megamorphic stub cache，
见 git history 调研票）以此修复为前置。
