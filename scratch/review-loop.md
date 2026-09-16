# 生产级评审循环状态（每 crate 组连续通过计数，目标 10）

规则：每轮按组开子代理深度评审；FOUND（发现并修复问题）→ 该组计数清零；PASS → +1。全组 ≥10 结束。

| 组 | crates | 连续通过 | 总轮次 |
|---|---|---|---|
| G1 common | ulua-common, ulua-config | 0 | 1 |
| G2 front | ulua-ast, ulua-ast-cli | 0 | 0 |
| G3 compiler | ulua-compiler, ulua-bytecode, ulua-bytecode-cli, ulua-compile-cli | 0 | 0 |
| G4 vm | ulua-vm | 0 | 1 |
| G5 codegen | ulua-code-gen | 1 | 1 |
| G6 analysis | ulua-analysis, ulua-analyze-cli | 0 | 0 |
| G7 rt | ulua-rt, ulua-rt-derive, ulua-checked-macros, ulua-require | 0 | 0 |
| G8 cli | ulua, ulua-cli-lib, ulua-reduce-cli, ulua-repl-cli | 0 | 0 |
| G9 tests-web | ulua-unit-test, ulua-conformance, ulua-e2e, ulua-cli-test, ulua-web | 0 | 0 |

## 轮次记录
- R1-G4 FOUND：wasm strtoll 垫片 endptr null 解引用（高）；get_clock_period libc 门控误用+残留手写 FFI（中）；wasm_libc 死垫片删除（低）。已修。
- R1-G5 PASS：generate_vm_exit_blocks 逐行对照 OptimizeDeadStore.cpp 一致；s_code/s_closure/mem 收敛无逻辑改动；A64 编码/页分配抽查一致。
- R1-G1 FOUND：SmallVector/VecDeque ZST layout UB（防御性，高）；DenseHashTable rehash 每项 key clone → move（中，性能）；LuauProtoFlag 补 LPF_USES_EXPORT（低）。已修，全量 5682 过。
