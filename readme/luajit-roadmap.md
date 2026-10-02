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

### E1 稀疏整数键 array 直达（nsieve 3.2x 的已证根因）
nsieve 剖证：newkey 21.3% + resize 7.7% + rehash 6.5% + s_settable 1.3% +
lua_v_settable 1.9%（插入慢路径合计 ~40%），COUNT 循环读走 lua_h_get 哈希查找
18.7%——`sieve[j]=true`（j=i²..n 步进 i，稀疏升序）在我们这里走 hash 部分 +
rehash 迁移舞步；LuaJIT `lj_tab_setinth` 对 1≤k≤MAXASIZE 直接扩 array part 落数组。
落点：`lua_h_newkey.rs`/`arrayornewkey.rs`/`lua_h_resizearray.rs`
（int 键 k 超出现 array 界 → 按 next-pow2(k) 扩 array 直插；rehash 的
numusearray/computesizes 逻辑保留兜底）。`#t` border 语义按 cpp oracle 钉。
预期：nsieve 3.2→1.5；表类基准连带 -2~4%。

### E2 CALL/RETURN 快路内联 + arity 特化（fib 2.1 / micro_call 1.5 / binarytrees 1.4）
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
