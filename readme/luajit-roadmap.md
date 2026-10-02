# 追平并超越 LuaJIT 2.1：路线图

依据：23 基准 × (解释器 / JIT) 对 LuaJIT 2.1 同模式实测（2026-10-02，M2 Max，
results.json 时间戳 2026-10-02T10:53:59Z）+ 文献调研（出处见文内链接）。

## 现状

| 模式 | 几何平均（ulua/LuaJIT） | 落后重灾 |
|---|---|---|
| 解释器 vs 解释器 | 1.05 | nsieve 3.2、coroutines 2.5、fib 2.1、patterns/micro_call 1.5 |
| JIT vs JIT | 2.53 | oop 16.8、spectralnorm 8.7、micro_call 6.8、nsieve 5.6、fib 4.3、nbody 3.8 |
| ulua JIT vs LuaJIT 解释器 | ~0.8（多数 <1） | nsieve 2.8、coroutines 2.5、patterns 1.5 |

已定性排除：coroutine 差距 ≠ 栈切换（LuaJIT 同为可重入解释器模型，
yield=跳出循环存 pc/base、resume=重入；差距是 re-entry 固定开销）；
codegen pass 吞吐（实测 1.25ms/模块）；表增长 vs cpp（微基准反超 1.6x）。

## 解释器组（目标 1.05 → <1.0）

### E1 稀疏整数键 array 直达——**已实测否决（语义红线）**
nsieve 剖证：插入慢路径合计 ~40%（newkey 21.3% + resize 7.7% + rehash 6.5% +
s_settable/lua_v_settable 3.2%），COUNT 循环读走 lua_h_get 哈希查找 18.7%——
`sieve[j]=true`（j=i²..n 步进 i，稀疏升序）在哈希部分 + rehash 迁移；LuaJIT
`lj_tab_setinth` 对 1≤k≤MAXASIZE 直接扩 array part。已实现受控扩容
（4×sizearray+16 上界 + next-pow2 resize）：nsieve **-32.7%**（44.1→29.7ms），
但 `#t` border 随布局改变（b 案 16 vs cpp 4、h 案 3 vs cpp 1；base 与 cpp 逐位
一致），**conformance_tables 当场红**（另触发 close_state 断言）→ 判定：表布局
语义被 cpp oracle 钉死，LuaJIT 式增长属语义分叉，不可发货。nsieve 3.2x 记为
**语义受限差距**；残余可做：cpp 同构下哈希路径微成本（getfreepos 扫描、
setnodekey 拷贝宽度），收益有限。

### E2 CALL/RETURN 快路内联 + arity 特化（fib 2.1 / micro_call 1.5 / binarytrees 1.4）

**2026-10-02 补充实测**：本轮核查发现 `call_arm` 宏已是 cpp 式内联快路
（Lua→Lua 调用直接 `continue` 同一循环，无 performcall 重入）——E2 的
「重入开销」前提不成立。剩余差距 = 派发次数 + 每调用帧建立开销（Rust ~20+ 指令
vs LuaJIT asm ~10），前者已到融合收益边界：micro_call 的 `LOADN → JUMPIFNOTLE`
（13.7% 边）融合实测 +1.2%（不显著）、fib 付探针税 -10% 风险 → 还原弃用
（LOADN 尾过热，单一用例的边不配探针）。call-boundary 边（Subk→Callfb 等）
不可融合（帧转换）。fib/micro_call 的剩余差距记为**调用帧建立成本的结构性差距**；
可做残余：checkstackfornewci 与 ci 字段写的微成本（需 samply 精确归因后另立项）。

**2026-10-02（第二次）E2-a 复测终审**：`LOADN → JUMPIFNOTLE` 融合以 CPU 时间
（getrusage 配对，13 轮交错）复测——micro_call +0.8%、fib +0.2%、mtg +1.3%，
三用例一致中性偏负，永久弃用。根因定论：LOADN 尾已有 SETTABLE/JUMPIFNOTLT
双探针，micro_call 命中 JUMPIFNOTLE 前需吃 2 次 probe miss，与省下的 1 次派发
零和。LOADN 尾探针已饱和，后续任何 LOADN 边（不同负载后继不同）的融合预期
收益为负，不再尝试。

**2026-10-02 E2 残余归因的教训**：samply 剖证必须先看样本量——fib 归因仅 429 样本，
「SETTABLE 探测 miss 占 7.2%」实为 31 个样本的统计噪声；据此做的双探针换序
（JUMPIFNOTLT 先探）在 CPU 时间（getrusage RUSAGE_CHILDREN，抗调度噪声）配对下
fib -1.2%、mtg -7.5%、micro_call +3.4%——方向与逻辑预期相反，纯属布局彩票，
已还原。结论：①单热点归因需 ≥5000 样本（拉长采样窗或提高采样率）；②微改动
（探针顺序/代码位移）的 A/B 必须用 CPU 时间 + 多用例交叉验证，wall-clock 在
共享机器上不可用。
fib/micro_call/binarytrees 的转移表被 CALLFB/RETURN 边主导（各 15%），
融合无法跨调用帧。LuaJIT asm fast path：判 Lua 闭包 + nargs==nparams + 推帧
全在解释器循环内完成，不重入。落点：`luau_execute.rs` CALL 臂内联 precall 的
PCRLUA 分支（写 CallInfo、参数直拷、continue），`performcall.rs` 降级为 C 边界
专用；RETURN 臂消费 `LUA_CALLINFO_RETURN`。预期 fib→1.3、micro_call→1.1。

### E3 coroutine re-entry 轻量化（coroutines 2.5）
yield 只存 pc/base 快照（已有）；`resume_continue.rs` 恢复 4 状态量后直跳
dispatch，入口一次性逻辑（VM_HAS_NATIVE、SCHEDULED_REENTRY、checkcstack）
拆 `#[cold]` 首次分支。预期 coroutines→1.3。

### E4/E5（余量）：gslot_hit 判据合并；装载期超级指令融合（ADD+JMP 等运行期计数驱动）。

## JIT 组（目标 2.53 → <1.0）

路线 = J1 → J2 → J3 → J4，全部复用现有 feedback_vector_slot +
analyze_bytecode_types + 线性 IR/块链设施（LBBV 论文数据：95% 块单版本，
method JIT 内可拿 tracing 的大部分类型特化收益）：

- **J1 BBV 类型版本化**（Lazy Basic Block Versioning, Chevalier-Boisvert &
  Feeley, ECOOP'15）：guard 失败按反馈类型重编译块，后继沿类型传播——删 tag
  检查、int 索引直达 array。落点 `analyze_bytecode_types.rs`、
  `build_bytecode_blocks.rs`、`records/feedback_vector_slot.rs`。预期 JIT -30~40%。
- **J2 单态调用内联 + fastcall**（PIC ECOOP'91）：调用站点 guard 闭包指纹，
  内联被调体/直跳入口。预期 oop 16.8→4、micro_call 6.8→2。
- **J3 HREFK 式表特化**（lj_asm.c asm_hrefk）：常量键 hash 编译期折入 +
  next 字段验证（已部分具备）。预期 nsieve/micro_arraywrite 残余。
- **J4 IC 单槽 → 2~4 槽 PIC**；**J5 profile-guided block ordering +
  循环超块**（Pettis & Hansen PLDI'90）；**J6 regalloc2**（spill 热度确认后）。

诚实上限：纯 FP 循环（spectralnorm）是 method JIT 对 trace JIT 最难档，
J1+J5 到 1.1~1.3x；综合 <1.0 需要 J1+J2+J3 全部到位且站点类基准反超拉开。

## 明确不做

- corosensei 切栈协程（模型不等价且对本基准更慢）
- cranelift/inkwell 后端、dynasm-rs 迁移（现有手写 assembler 已工作）
- patchouly/copy-and-patch（nightly 依赖 + 定位是 baseline tier 不是优化 tier）

## 实测记录

- 2026-10-02：nsieve 融合补全（Fornloop→Loadb / Gettable→Jumpif /
  Jumpif→Fornloop，含 fuse 函数路径的探针安装）——功能正确（探针证实触发、
  指纹一致），但派发仅占 nsieve 16%，实测中性（-0~2% med，不达 5% 门）→
  还原弃用。教训：融合前先看剖面，nsieve 的时间在表慢路径（40%）与哈希读
  （19%），不在派发。
